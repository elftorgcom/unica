use crate::domain::cache::{path_for_report, CacheImpact, CacheReport};
use crate::domain::events::DomainEvent;
use crate::domain::workspace::WorkspaceContext;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct WorkspaceState {
    workspace_root: String,
    workspace_epoch: u64,
    caches: BTreeMap<String, CacheEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CacheEntry {
    status: CacheStatus,
    epoch: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum CacheStatus {
    Fresh,
    Stale,
}

pub struct WorkspaceStateRepository {
    state_path: PathBuf,
}

impl WorkspaceStateRepository {
    pub fn new(context: &WorkspaceContext) -> Self {
        Self {
            state_path: context.cache_root.join("state.json"),
        }
    }

    pub fn report(
        &self,
        context: &WorkspaceContext,
        events: &[DomainEvent],
        dry_run: bool,
    ) -> Result<CacheReport, String> {
        let impact = CacheImpact::from_events(events);
        let mut state = self.load(context);
        let mut invalidated = sorted(impact.invalidated);
        let mut refreshed = sorted(impact.eager_refresh);

        if !events.is_empty() && !dry_run {
            for name in &invalidated {
                state.caches.insert(
                    name.clone(),
                    CacheEntry {
                        status: CacheStatus::Stale,
                        epoch: context.workspace_epoch,
                    },
                );
            }
            for name in &refreshed {
                state.caches.insert(
                    name.clone(),
                    CacheEntry {
                        status: CacheStatus::Fresh,
                        epoch: context.workspace_epoch,
                    },
                );
            }
            state.workspace_epoch = context.workspace_epoch;
            self.save(&state)?;
        }

        if dry_run {
            refreshed.clear();
        }
        if events.is_empty() {
            invalidated.clear();
            refreshed.clear();
        }

        let mut stale = Vec::new();
        let mut fresh = Vec::new();
        for (name, entry) in state.caches {
            match entry.status {
                CacheStatus::Fresh => fresh.push(name),
                CacheStatus::Stale => stale.push(name),
            }
        }

        Ok(CacheReport {
            mode: if events.is_empty() {
                "read".to_string()
            } else if dry_run {
                "dry-run".to_string()
            } else {
                "applied".to_string()
            },
            root: path_for_report(&context.cache_root),
            workspace_epoch: context.workspace_epoch,
            events: events
                .iter()
                .map(|event| event.name().to_string())
                .collect(),
            invalidated,
            refreshed,
            stale,
            fresh,
        })
    }

    fn load(&self, context: &WorkspaceContext) -> WorkspaceState {
        let Ok(text) = fs::read_to_string(&self.state_path) else {
            return default_state(context);
        };
        serde_json::from_str(&text).unwrap_or_else(|_| default_state(context))
    }

    fn save(&self, state: &WorkspaceState) -> Result<(), String> {
        if let Some(parent) = self.state_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|err| format!("failed to create Unica cache directory: {err}"))?;
        }
        let text = serde_json::to_string_pretty(state).map_err(|err| err.to_string())?;
        fs::write(&self.state_path, text + "\n")
            .map_err(|err| format!("failed to write Unica cache state: {err}"))
    }
}

fn default_state(context: &WorkspaceContext) -> WorkspaceState {
    WorkspaceState {
        workspace_root: context.workspace_root.display().to_string(),
        workspace_epoch: context.workspace_epoch,
        caches: BTreeMap::new(),
    }
}

fn sorted(values: std::collections::BTreeSet<String>) -> Vec<String> {
    values.into_iter().collect()
}
