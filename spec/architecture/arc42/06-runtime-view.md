# 6. Представление времени выполнения

## Initialize

1. Codex starts `plugins/unica/scripts/run-unica.sh`.
2. Source checkout path runs `cargo run --package unica-coder --bin unica`.
3. Packaged path runs `scripts/run-tool.sh unica`.
4. MCP `initialize` returns `serverInfo.name = "unica"`.

## Tool List

1. MCP `tools/list` calls the application tool registry.
2. The response contains only `unica.*` tools.
3. Internal adapters are not listed.

## Mutating Dry Run

1. Caller invokes a mutating tool without `dryRun: false`.
2. Application resolves `dryRun: true`.
3. Adapter returns planned command or placeholder outcome without changing files.
4. Application emits the relevant domain event for impact calculation.
5. Cache report returns `mode = "dry-run"` and impacted cache names.

## Applied Mutation

1. Caller explicitly passes `dryRun: false`.
2. Native MCP handler executes the operation. A transitional adapter may execute
   only for not-yet-migrated operations.
3. Successful mutation emits domain events.
4. `WorkspaceStateRepository` marks affected caches stale and records eager
   refreshes.
5. Result returns `{ ok, summary, changes, warnings, errors, artifacts, cache }`.

## Read Operation

Read tools do not emit mutation events by default. They may inspect current cache
state and, in future slices, trigger lazy refresh if a required cache is stale.
