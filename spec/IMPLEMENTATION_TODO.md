# Active Tasks For `unica`

This file tracks open implementation work only.

## Current Tasks

- [ ] Implement former skill-local Python/PowerShell command semantics inside
  native `unica.*` MCP tools. Start with read-only `info` and `validate`, then
  generators/removers, then editors and complex CFE/UI operations.
- [ ] Rewrite migrated skills so the workflow path references only MCP `unica`
  tools and does not include operation commands from skill-local files.
- [ ] Remove migrated skill-local operation files after fixture parity is proven.
  Package launchers under `plugins/unica/scripts/` remain infrastructure and are
  not part of this removal.
- [ ] Add a CI test that rejects operation guidance in migrated skills when it
  points to skill-local Python/PowerShell files instead of `unica.*` MCP tools.
- [ ] Implement the native HTTP client/proxy for the standards adapter so
  `unica.standards.search` and `unica.standards.explain` execute against the
  v8std MCP endpoint internally instead of returning the current placeholder
  adapter response.
- [ ] Expand `WorkspaceStateRepository` from reporting invalidation impact to lazy
  cache rebuild for heavy indexes and eager rebuild for cheap critical graphs.
- [ ] Add fake-adapter contract tests for build/runtime, code analysis and
  standards adapters.
- [ ] Add package-level smoke that verifies generated marketplace `.mcp.json` exposes
  only `unica` and that the packaged `run-unica.sh` starts the bundled binary.

## Rules

- Keep this file short and active-only.
- If a task changes a public or architectural contract, update the ADR and active
  docs layer before implementation.
- Promote only immediately executable work here.

## Done Criteria

- The behavior is covered by a focused test.
- The relevant ADR or invariant is updated if the public contract changes.
- `python3.12 -m unittest discover -s tests/ci` and
  `cargo test --package unica-coder` pass.
- `plugins/unica/scripts/run-unica.sh --help` still reports the public server as
  `unica`.
