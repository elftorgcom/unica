# 8. Сквозные концепции

## Single Public MCP

The LLM sees one server and does not coordinate multiple MCP caches or indexes.
This is the primary token and context saving mechanism.

## Dry Run Safety

Mutating tools default to dry-run. Skills pass `dryRun: false` only for explicit
user-requested mutations.

## Cache Ownership

The orchestrator owns cache state. Adapter calls must report through application
use cases so domain events and cache invalidation cannot be bypassed.

## Internal Adapter Pattern

Adapters are typed boundaries around existing engines. They may use CLI or MCP
protocol internally, but their names and cache lifecycle are not exposed to LLM.

Skill-local Python/PowerShell operation files are not a target adapter class.
They are temporary migration sources for behavior that must move into native
`unica.*` MCP handlers.

## Source Of Truth Order

When documents disagree, use this order:

1. current code and tests;
2. package manifests and `.mcp.json`;
3. active `spec/`;
4. README and skill prose;
5. archived or research docs.
