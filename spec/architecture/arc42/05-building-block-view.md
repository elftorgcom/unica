# 5. Представление строительных блоков

## Top Level

- `interfaces::mcp`: stdio MCP transport, JSON-RPC methods, tool list and call
  response mapping.
- `application`: `UnicaApplication`, `ToolSpec`, `ToolHandler`,
  `OperationResult`.
- `domain`: `WorkspaceContext`, `DomainEvent`, `CacheImpact`, `CacheReport`.
- `infrastructure`: command adapters, standards adapter, package launchers, and
  `WorkspaceStateRepository`.

## Domain Blocks

- `WorkspaceContext` discovers cwd, workspace root, cache root, and workspace
  epoch.
- `DomainEventKind` describes state-changing facts such as `FormChanged`,
  `BuildCompleted`, and `SourceSetChanged`.
- `CacheImpact` maps events to invalidated and eagerly refreshed cache names.

## Application Blocks

- `tools()` is the canonical public tool registry.
- `call_tool()` resolves dry-run semantics, workspace context, adapter dispatch,
  event emission, and cache reporting.
- Mutating tools default to `dryRun: true`.

## Infrastructure Blocks

- Transitional operation-file adapters may exist only to preserve behavior while
  native MCP handlers are implemented.
- `CliAdapter` invokes checksum-wrapped bundled tools.
- `StandardsAdapter` is the internal standards boundary and must become the real
  HTTP MCP client before closing the standards gap.
- `WorkspaceStateRepository` persists volatile cache state under the configured
  cache root.

## Target Native MCP Handlers

The target implementation for configuration, form, SKD, MXL, role, subsystem,
interface, and template operations is native Rust logic behind `unica.*` tools.
Skill-local Python/PowerShell operation files must not remain as a target
building block.
