use crate::domain::cache::CacheReport;
use crate::domain::events::{DomainEvent, DomainEventKind};
use crate::domain::workspace::WorkspaceContext;
use crate::infrastructure::internal_adapters::{CliAdapter, StandardsAdapter};
use crate::infrastructure::legacy_scripts::LegacyScriptAdapter;
use crate::infrastructure::workspace_state::WorkspaceStateRepository;
use crate::infrastructure::AdapterOutcome;
use serde::Serialize;
use serde_json::{Map, Value};
use std::env;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy)]
pub struct ToolSpec {
    pub name: &'static str,
    pub description: &'static str,
    pub mutating: bool,
    pub handler: ToolHandler,
}

#[derive(Debug, Clone, Copy)]
pub enum ToolHandler {
    LegacyScript {
        skill: &'static str,
        script: &'static str,
        event: Option<DomainEventKind>,
    },
    ProjectStatus,
    BuildRuntime {
        command: &'static [&'static str],
        event: Option<DomainEventKind>,
    },
    CodeAdapter {
        command: &'static [&'static str],
    },
    StandardsAdapter {
        operation: &'static str,
    },
}

#[derive(Debug, Serialize)]
pub struct OperationResult {
    pub ok: bool,
    pub summary: String,
    pub changes: Vec<String>,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
    pub artifacts: Vec<String>,
    pub cache: CacheReport,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stdout: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stderr: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,
}

pub struct UnicaApplication;

impl UnicaApplication {
    pub fn new() -> Self {
        Self
    }

    pub fn tools(&self) -> Vec<ToolSpec> {
        tools()
    }

    pub fn call_tool(
        &self,
        name: &str,
        args: &Map<String, Value>,
    ) -> Result<OperationResult, String> {
        let spec = tools()
            .into_iter()
            .find(|tool| tool.name == name)
            .ok_or_else(|| format!("unknown unica tool: {name}"))?;
        call_tool(spec, args)
    }
}

impl Default for UnicaApplication {
    fn default() -> Self {
        Self::new()
    }
}

pub fn tools() -> Vec<ToolSpec> {
    let mut specs = configuration_tools();
    specs.extend([
        ToolSpec {
            name: "unica.project.status",
            description: "Inspect current Unica workspace, source set, and cache state.",
            mutating: false,
            handler: ToolHandler::ProjectStatus,
        },
        ToolSpec {
            name: "unica.build.dump",
            description: "Dump source set through the internal build/runtime adapter.",
            mutating: true,
            handler: ToolHandler::BuildRuntime {
                command: &["dump"],
                event: Some(DomainEventKind::SourceSetChanged),
            },
        },
        ToolSpec {
            name: "unica.build.load",
            description: "Load/build XML source set through the internal build/runtime adapter.",
            mutating: true,
            handler: ToolHandler::BuildRuntime {
                command: &["build"],
                event: Some(DomainEventKind::BuildCompleted),
            },
        },
        ToolSpec {
            name: "unica.build.update",
            description:
                "Apply built configuration changes through the internal build/runtime adapter.",
            mutating: true,
            handler: ToolHandler::BuildRuntime {
                command: &["build", "--update"],
                event: Some(DomainEventKind::BuildCompleted),
            },
        },
        ToolSpec {
            name: "unica.build.make",
            description: "Create CF/CFE artifact through the internal build/runtime adapter.",
            mutating: true,
            handler: ToolHandler::BuildRuntime {
                command: &["make"],
                event: None,
            },
        },
        ToolSpec {
            name: "unica.build.run",
            description:
                "Launch 1C runtime or Designer through the internal build/runtime adapter.",
            mutating: true,
            handler: ToolHandler::BuildRuntime {
                command: &["launch"],
                event: None,
            },
        },
        ToolSpec {
            name: "unica.code.search",
            description: "Search BSL code through the internal code index adapter.",
            mutating: false,
            handler: ToolHandler::CodeAdapter {
                command: &["search"],
            },
        },
        ToolSpec {
            name: "unica.code.diagnostics",
            description: "Run BSL diagnostics through the internal code analysis adapter.",
            mutating: false,
            handler: ToolHandler::CodeAdapter {
                command: &["analyze"],
            },
        },
        ToolSpec {
            name: "unica.standards.search",
            description: "Search 1C standards through the internal standards adapter.",
            mutating: false,
            handler: ToolHandler::StandardsAdapter {
                operation: "search",
            },
        },
        ToolSpec {
            name: "unica.standards.explain",
            description:
                "Explain 1C diagnostics or standards through the internal standards adapter.",
            mutating: false,
            handler: ToolHandler::StandardsAdapter {
                operation: "explain",
            },
        },
    ]);
    specs
}

fn call_tool(spec: ToolSpec, args: &Map<String, Value>) -> Result<OperationResult, String> {
    let dry_run = args
        .get("dryRun")
        .and_then(Value::as_bool)
        .unwrap_or(spec.mutating);
    let cwd = args
        .get("cwd")
        .and_then(Value::as_str)
        .map(PathBuf::from)
        .unwrap_or(
            env::current_dir().map_err(|err| format!("failed to read current directory: {err}"))?,
        );
    let context = WorkspaceContext::discover(cwd)?;
    let state_repo = WorkspaceStateRepository::new(&context);

    let outcome = match spec.handler {
        ToolHandler::LegacyScript { skill, script, .. } => LegacyScriptAdapter::invoke(
            skill,
            script,
            spec.name,
            args,
            &context,
            dry_run,
            spec.mutating,
        )?,
        ToolHandler::ProjectStatus => project_status(&context),
        ToolHandler::BuildRuntime { command, .. } => CliAdapter::new(
            "run-v8-runner.sh",
            command,
            "build/runtime",
        )
        .invoke(spec.name, args, &context, dry_run, spec.mutating)?,
        ToolHandler::CodeAdapter { command } => CliAdapter::new(
            "run-bsl-analyzer.sh",
            command,
            "code analysis",
        )
        .invoke(spec.name, args, &context, dry_run, spec.mutating)?,
        ToolHandler::StandardsAdapter { operation } => StandardsAdapter::invoke(operation, args),
    };

    let events = if should_emit_events(spec, dry_run, &outcome) {
        domain_events(spec)
    } else {
        Vec::new()
    };
    let cache = state_repo.report(&context, &events, dry_run)?;

    Ok(OperationResult {
        ok: outcome.ok,
        summary: outcome.summary,
        changes: outcome.changes,
        warnings: outcome.warnings,
        errors: outcome.errors,
        artifacts: outcome.artifacts,
        cache,
        stdout: outcome.stdout,
        stderr: outcome.stderr,
        command: outcome.command,
    })
}

fn should_emit_events(spec: ToolSpec, dry_run: bool, outcome: &AdapterOutcome) -> bool {
    spec.mutating && (dry_run || outcome.ok)
}

fn domain_events(spec: ToolSpec) -> Vec<DomainEvent> {
    match spec.handler {
        ToolHandler::LegacyScript {
            event: Some(event), ..
        } => vec![DomainEvent::new(event, spec.name)],
        ToolHandler::BuildRuntime {
            event: Some(event), ..
        } => vec![DomainEvent::new(event, spec.name)],
        _ => Vec::new(),
    }
}

fn project_status(context: &WorkspaceContext) -> AdapterOutcome {
    let mut outcome = AdapterOutcome::ok(format!(
        "workspace root: {}; cache root: {}",
        context.workspace_root.display(),
        context.cache_root.display()
    ));
    outcome
        .artifacts
        .push(context.workspace_root.display().to_string());
    outcome
        .artifacts
        .push(context.cache_root.display().to_string());
    outcome
}

fn configuration_tools() -> Vec<ToolSpec> {
    vec![
        legacy(
            "unica.cf.edit",
            "cf-edit",
            "cf-edit.py",
            "Edit root Configuration.xml properties, ChildObjects, panels, and home page.",
            true,
            Some(DomainEventKind::ConfigXmlChanged),
        ),
        legacy(
            "unica.cf.info",
            "cf-info",
            "cf-info.py",
            "Inspect root Configuration.xml.",
            false,
            None,
        ),
        legacy(
            "unica.cf.init",
            "cf-init",
            "cf-init.py",
            "Create empty 1C configuration XML scaffold.",
            true,
            Some(DomainEventKind::ConfigXmlChanged),
        ),
        legacy(
            "unica.cf.validate",
            "cf-validate",
            "cf-validate.py",
            "Validate root configuration XML structure.",
            false,
            None,
        ),
        legacy(
            "unica.cfe.borrow",
            "cfe-borrow",
            "cfe-borrow.py",
            "Borrow configuration objects/forms into an extension.",
            true,
            Some(DomainEventKind::CfeChanged),
        ),
        legacy(
            "unica.cfe.diff",
            "cfe-diff",
            "cfe-diff.py",
            "Inspect extension contents and transferred insertion blocks.",
            false,
            None,
        ),
        legacy(
            "unica.cfe.init",
            "cfe-init",
            "cfe-init.py",
            "Create extension XML scaffold.",
            true,
            Some(DomainEventKind::CfeChanged),
        ),
        legacy(
            "unica.cfe.patch_method",
            "cfe-patch-method",
            "cfe-patch-method.py",
            "Generate a CFE method interceptor.",
            true,
            Some(DomainEventKind::ModuleChanged),
        ),
        legacy(
            "unica.cfe.validate",
            "cfe-validate",
            "cfe-validate.py",
            "Validate extension XML structure.",
            false,
            None,
        ),
        legacy(
            "unica.meta.compile",
            "meta-compile",
            "meta-compile.py",
            "Compile metadata object XML from JSON DSL.",
            true,
            Some(DomainEventKind::MetadataChanged),
        ),
        legacy(
            "unica.meta.edit",
            "meta-edit",
            "meta-edit.py",
            "Edit metadata object XML.",
            true,
            Some(DomainEventKind::MetadataChanged),
        ),
        legacy(
            "unica.meta.info",
            "meta-info",
            "meta-info.py",
            "Inspect metadata object XML.",
            false,
            None,
        ),
        legacy(
            "unica.meta.remove",
            "meta-remove",
            "meta-remove.py",
            "Remove metadata object XML and registration.",
            true,
            Some(DomainEventKind::MetadataChanged),
        ),
        legacy(
            "unica.meta.validate",
            "meta-validate",
            "meta-validate.py",
            "Validate metadata object XML.",
            false,
            None,
        ),
        legacy(
            "unica.form.add",
            "form-add",
            "form-add.py",
            "Add managed form metadata and files.",
            true,
            Some(DomainEventKind::FormChanged),
        ),
        legacy(
            "unica.form.compile",
            "form-compile",
            "form-compile.py",
            "Compile managed Form.xml from JSON DSL or metadata.",
            true,
            Some(DomainEventKind::FormChanged),
        ),
        legacy(
            "unica.form.edit",
            "form-edit",
            "form-edit.py",
            "Edit managed Form.xml elements, attributes, and commands.",
            true,
            Some(DomainEventKind::FormChanged),
        ),
        legacy(
            "unica.form.info",
            "form-info",
            "form-info.py",
            "Inspect managed Form.xml.",
            false,
            None,
        ),
        legacy(
            "unica.form.remove",
            "form-remove",
            "remove-form.py",
            "Remove a managed form and registration.",
            true,
            Some(DomainEventKind::FormChanged),
        ),
        legacy(
            "unica.form.validate",
            "form-validate",
            "form-validate.py",
            "Validate managed Form.xml.",
            false,
            None,
        ),
        legacy(
            "unica.interface.edit",
            "interface-edit",
            "interface-edit.py",
            "Edit subsystem CommandInterface.xml.",
            true,
            Some(DomainEventKind::SubsystemChanged),
        ),
        legacy(
            "unica.interface.validate",
            "interface-validate",
            "interface-validate.py",
            "Validate CommandInterface.xml.",
            false,
            None,
        ),
        legacy(
            "unica.subsystem.compile",
            "subsystem-compile",
            "subsystem-compile.py",
            "Compile subsystem XML from JSON DSL.",
            true,
            Some(DomainEventKind::SubsystemChanged),
        ),
        legacy(
            "unica.subsystem.edit",
            "subsystem-edit",
            "subsystem-edit.py",
            "Edit subsystem XML content and hierarchy.",
            true,
            Some(DomainEventKind::SubsystemChanged),
        ),
        legacy(
            "unica.subsystem.info",
            "subsystem-info",
            "subsystem-info.py",
            "Inspect subsystem XML and command interface.",
            false,
            None,
        ),
        legacy(
            "unica.subsystem.validate",
            "subsystem-validate",
            "subsystem-validate.py",
            "Validate subsystem XML.",
            false,
            None,
        ),
        legacy(
            "unica.template.add",
            "template-add",
            "add-template.py",
            "Add a template to an object and register it.",
            true,
            Some(DomainEventKind::TemplateChanged),
        ),
        legacy(
            "unica.template.remove",
            "template-remove",
            "remove-template.py",
            "Remove a template from an object.",
            true,
            Some(DomainEventKind::TemplateChanged),
        ),
        legacy(
            "unica.skd.compile",
            "skd-compile",
            "skd-compile.py",
            "Compile Data Composition Schema XML from JSON DSL.",
            true,
            Some(DomainEventKind::SkdChanged),
        ),
        legacy(
            "unica.skd.edit",
            "skd-edit",
            "skd-edit.py",
            "Edit Data Composition Schema Template.xml.",
            true,
            Some(DomainEventKind::SkdChanged),
        ),
        legacy(
            "unica.skd.info",
            "skd-info",
            "skd-info.py",
            "Inspect Data Composition Schema Template.xml.",
            false,
            None,
        ),
        legacy(
            "unica.skd.validate",
            "skd-validate",
            "skd-validate.py",
            "Validate Data Composition Schema Template.xml.",
            false,
            None,
        ),
        legacy(
            "unica.mxl.compile",
            "mxl-compile",
            "mxl-compile.py",
            "Compile spreadsheet Template.xml from JSON DSL.",
            true,
            Some(DomainEventKind::MxlChanged),
        ),
        legacy(
            "unica.mxl.decompile",
            "mxl-decompile",
            "mxl-decompile.py",
            "Decompile spreadsheet Template.xml to JSON DSL.",
            false,
            None,
        ),
        legacy(
            "unica.mxl.info",
            "mxl-info",
            "mxl-info.py",
            "Inspect spreadsheet Template.xml.",
            false,
            None,
        ),
        legacy(
            "unica.mxl.validate",
            "mxl-validate",
            "mxl-validate.py",
            "Validate spreadsheet Template.xml.",
            false,
            None,
        ),
        legacy(
            "unica.role.compile",
            "role-compile",
            "role-compile.py",
            "Compile role metadata and Rights.xml from JSON DSL.",
            true,
            Some(DomainEventKind::RoleChanged),
        ),
        legacy(
            "unica.role.info",
            "role-info",
            "role-info.py",
            "Inspect role Rights.xml.",
            false,
            None,
        ),
        legacy(
            "unica.role.validate",
            "role-validate",
            "role-validate.py",
            "Validate role Rights.xml.",
            false,
            None,
        ),
    ]
}

fn legacy(
    name: &'static str,
    skill: &'static str,
    script: &'static str,
    description: &'static str,
    mutating: bool,
    event: Option<DomainEventKind>,
) -> ToolSpec {
    ToolSpec {
        name,
        description,
        mutating,
        handler: ToolHandler::LegacyScript {
            skill,
            script,
            event,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Map;

    #[test]
    fn lists_unica_orchestrator_scope() {
        let names = tools().iter().map(|tool| tool.name).collect::<Vec<_>>();
        assert!(names.contains(&"unica.project.status"));
        assert!(names.contains(&"unica.form.validate"));
        assert!(names.contains(&"unica.skd.edit"));
        assert!(names.contains(&"unica.mxl.compile"));
        assert!(names.contains(&"unica.role.validate"));
        assert!(names.contains(&"unica.build.load"));
        assert!(names.contains(&"unica.standards.explain"));
        assert!(!names.contains(&"unica-coder"));
    }

    #[test]
    fn mutating_tool_defaults_to_dry_run_and_reports_cache() {
        let result = UnicaApplication::new()
            .call_tool("unica.form.edit", &Map::new())
            .unwrap();
        assert!(result.ok);
        assert!(result.summary.contains("dry run"));
        assert!(result.command.unwrap()[0].contains("python3"));
        assert_eq!(result.cache.mode, "dry-run");
        assert!(result.cache.events.contains(&"FormChanged".to_string()));
        assert!(result
            .cache
            .invalidated
            .contains(&"metadata_graph".to_string()));
    }

    #[test]
    fn project_status_is_read_only_and_cache_aware() {
        let result = UnicaApplication::new()
            .call_tool("unica.project.status", &Map::new())
            .unwrap();
        assert!(result.ok);
        assert_eq!(result.cache.mode, "read");
        assert!(result.summary.contains("workspace root"));
    }
}
