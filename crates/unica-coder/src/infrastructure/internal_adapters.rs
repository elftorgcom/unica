use crate::domain::workspace::WorkspaceContext;
use crate::infrastructure::legacy_scripts::{find_plugin_root, value_to_cli_string};
use crate::infrastructure::AdapterOutcome;
use serde_json::{Map, Value};
use std::path::PathBuf;
use std::process::Command;

pub struct CliAdapter {
    launcher: &'static str,
    default_command: &'static [&'static str],
    label: &'static str,
}

impl CliAdapter {
    pub fn new(
        launcher: &'static str,
        default_command: &'static [&'static str],
        label: &'static str,
    ) -> Self {
        Self {
            launcher,
            default_command,
            label,
        }
    }

    pub fn invoke(
        &self,
        tool_name: &str,
        args: &Map<String, Value>,
        context: &WorkspaceContext,
        dry_run: bool,
        mutating: bool,
    ) -> Result<AdapterOutcome, String> {
        let plugin_root = find_plugin_root(&context.cwd).ok_or_else(|| {
            "could not locate Unica plugin root for internal adapter lookup".to_string()
        })?;
        let launcher = plugin_root.join("scripts").join(self.launcher);
        let mut command = vec![launcher.display().to_string()];
        command.extend(self.default_command.iter().map(|part| (*part).to_string()));
        command.extend(cli_args(args));

        if dry_run {
            return Ok(AdapterOutcome {
                ok: true,
                summary: format!(
                    "dry run: {tool_name} would call internal {} adapter",
                    self.label
                ),
                changes: if mutating {
                    vec!["no files changed because dryRun is true".to_string()]
                } else {
                    Vec::new()
                },
                warnings: if launcher.exists() {
                    Vec::new()
                } else {
                    vec![format!(
                        "internal adapter launcher not found: {}",
                        launcher.display()
                    )]
                },
                errors: Vec::new(),
                artifacts: Vec::new(),
                stdout: None,
                stderr: None,
                command: Some(command),
            });
        }

        if !launcher.exists() {
            return Err(format!(
                "internal adapter launcher not found: {}",
                launcher.display()
            ));
        }

        let output = Command::new(&launcher)
            .args(self.default_command)
            .args(cli_args(args))
            .current_dir(&context.cwd)
            .output()
            .map_err(|err| format!("failed to execute internal {} adapter: {err}", self.label))?;
        let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
        let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
        let ok = output.status.success();
        Ok(AdapterOutcome {
            ok,
            summary: if ok {
                format!(
                    "{tool_name} completed through internal {} adapter",
                    self.label
                )
            } else {
                format!("{tool_name} failed through internal {} adapter", self.label)
            },
            changes: if mutating {
                vec![format!("internal {} adapter executed", self.label)]
            } else {
                Vec::new()
            },
            warnings: if ok {
                Vec::new()
            } else {
                vec![format!(
                    "internal {} adapter exited with status {}",
                    self.label, output.status
                )]
            },
            errors: if ok {
                Vec::new()
            } else {
                vec![stderr.trim().to_string()]
            },
            artifacts: Vec::new(),
            stdout: Some(stdout),
            stderr: Some(stderr),
            command: Some(command),
        })
    }
}

pub struct StandardsAdapter;

impl StandardsAdapter {
    pub fn invoke(operation: &str, args: &Map<String, Value>) -> AdapterOutcome {
        let mut outcome = AdapterOutcome::ok(format!(
            "internal standards adapter accepted {operation}; HTTP MCP proxy is hidden behind unica"
        ));
        outcome.warnings.push(
            "native HTTP proxy execution is not enabled in this migration slice; the request is kept inside the unica contract".to_string(),
        );
        outcome
            .artifacts
            .push("https://ai.v8std.ru/mcp".to_string());
        if !args.is_empty() {
            outcome.stdout = Some(Value::Object(args.clone()).to_string());
        }
        outcome
    }
}

fn cli_args(args: &Map<String, Value>) -> Vec<String> {
    if let Some(Value::Array(items)) = args.get("args") {
        return items.iter().map(value_to_cli_string).collect();
    }

    let mut result = Vec::new();
    for (key, value) in args {
        if matches!(key.as_str(), "dryRun" | "cwd" | "confirm") {
            continue;
        }
        let flag = format!("--{}", kebab_case(key));
        match value {
            Value::Bool(true) => result.push(flag),
            Value::Bool(false) | Value::Null => {}
            Value::Array(items) => {
                for item in items {
                    result.push(flag.clone());
                    result.push(value_to_cli_string(item));
                }
            }
            other => {
                result.push(flag);
                result.push(value_to_cli_string(other));
            }
        }
    }
    result
}

fn kebab_case(key: &str) -> String {
    let mut out = String::new();
    for (index, ch) in key.chars().enumerate() {
        if ch == '_' {
            out.push('-');
        } else if ch.is_ascii_uppercase() {
            if index > 0 {
                out.push('-');
            }
            out.push(ch.to_ascii_lowercase());
        } else {
            out.push(ch);
        }
    }
    out
}

#[allow(dead_code)]
fn _path_list(paths: &[PathBuf]) -> Vec<String> {
    paths
        .iter()
        .map(|path| path.display().to_string())
        .collect()
}
