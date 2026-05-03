# 4. Стратегия решения

## Strategy

Use a pragmatic DDD split:

- domain: workspace identity, cache impact, domain events;
- application: tool registry, use case dispatch, orchestration;
- infrastructure: internal adapters and filesystem state;
- interfaces: MCP JSON-RPC transport.

## Key Decisions

1. Hide all engines behind one MCP server.
2. Keep application logic transport-neutral.
3. Emit domain events for mutating operations.
4. Let cache invalidation happen inside `unica`.
5. Treat current skill-local operation files as tracked migration debt; target
   handlers live inside the Rust MCP.

## Migration Shape

The first slice prioritizes public contract and orchestration. Native Rust
replacement of skill-local operation files happens in waves after contract tests
and fixtures exist.
