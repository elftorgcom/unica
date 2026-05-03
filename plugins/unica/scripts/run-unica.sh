#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PLUGIN_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
REPO_ROOT="$(cd "$PLUGIN_ROOT/../.." && pwd)"

if [ -f "$REPO_ROOT/Cargo.toml" ] && command -v cargo >/dev/null 2>&1; then
  export UNICA_PLUGIN_ROOT="$PLUGIN_ROOT"
  exec cargo run --quiet --package unica-coder --bin unica --manifest-path "$REPO_ROOT/Cargo.toml" -- "$@"
fi

exec "$SCRIPT_DIR/run-tool.sh" unica "$@"
