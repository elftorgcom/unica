# Architecture Change Checklist

Use this checklist when changing public MCP tools, skill routing, adapters,
cache behavior, or packaging metadata.

## MCP Surface

- [ ] `.mcp.json` still declares exactly one public server: `unica`.
- [ ] `initialize` still returns `serverInfo.name = "unica"`.
- [ ] `tools/list` contains intended `unica.*` tools only.
- [ ] Public tool name changes are covered by tests and ADR updates.

## Skill Routing

- [ ] Updated skills mention MCP `unica`.
- [ ] Updated skills do not expose internal adapter server names as user-facing
  routing.
- [ ] Updated migrated skills do not point users to skill-local Python/PowerShell
  operation files.
- [ ] Mutating skills preserve explicit `dryRun: false` guidance.

## Cache And Events

- [ ] Mutating operation emits the right `DomainEventKind`.
- [ ] `CacheImpact` invalidates affected caches.
- [ ] Dry-run reports impact without writing state.
- [ ] Applied operation writes state only after successful mutation or approved
  state transition.

## Adapters

- [ ] Internal adapter errors are summarized in `warnings` or `errors`.
- [ ] Adapter command construction is covered by focused tests when behavior is
  non-trivial.
- [ ] If a skill-local operation file is still used, it is recorded as migration
  debt with a parity/removal task.
- [ ] Native MCP implementation and fixture parity exist before deleting the
  corresponding operation file.

## Packaging

- [ ] `third-party/tools.lock.json` names the bundled binary `unica`.
- [ ] Generated `third-party/manifest.json` matches the lock.
- [ ] `run-unica.sh --help` works from source checkout and generated package.
- [ ] Windows `win-x64` package `.mcp.json` uses
  `pwsh -NoProfile -File ./plugins/unica/scripts/run-unica.ps1`.
- [ ] Fresh Codex visibility is checked from a clean cache when changing plugin
  metadata.

## Verification

Run:

```sh
cargo fmt --all -- --check
cargo clippy --package unica-coder --all-targets -- -D warnings
cargo test --package unica-coder
python3.12 -m unittest discover -s tests/ci
git diff --check
```
