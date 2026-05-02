# Unica Internal Tooling Notes

This document is internal reference material for the Unica plugin package. Public
skills should describe 1C developer operations, not the bundled tools themselves.

## Public Skill Boundary

Public skills live in `skills/` and model tasks a 1C developer performs:

- create, edit, validate, compile, dump, and load metadata;
- build and validate EPF/ERF artifacts;
- create and update infobases;
- inspect and edit forms, roles, SKD, MXL, subsystems, interfaces, and web publication;
- search, analyze, and validate BSL code as part of those workflows.

Tool-specific behavior is an implementation detail of those workflows.

## Bundled Tools

The pinned bundled tools are declared in `third-party/manifest.json`.

- `bsl-analyzer`: BSL diagnostics, metadata/code inspection, and local/reference MCP profiles.
- `v8-runner`: 1C build, syntax, test, and platform-oriented automation.
- `rlm-tools-bsl`: token-efficient exploration of large 1C BSL repositories.
- `rlm-bsl-index`: repository indexing for `rlm-tools-bsl`.
- `unica-v8std`: remote streamable HTTP MCP endpoint for standards, APK codes, and v8-code-style context.

Never replace a binary without updating the manifest SHA-256 and bumping the plugin version.

## Launchers

Bundled tools are launched through checksum-verifying wrappers instead of direct
binary paths. This prevents accidental use of a globally installed tool with a
different version.

- `scripts/run-tool.sh <tool-name> [args...]` is the macOS/Linux launcher.
- `scripts/run-tool.ps1 <tool-name> [args...]` is the PowerShell launcher.
- Per-tool shell wrappers call `run-tool.sh` for current macOS MCP entries.

Launcher responsibilities:

- locate the plugin root from the launcher path;
- read `third-party/manifest.json`;
- reject unsupported host triples;
- verify the tool binary exists;
- verify SHA-256 before every execution;
- forward all remaining arguments unchanged.

## MCP Contract

The plugin declares MCP servers in `.mcp.json`:

- `unica-bsl-reference`
- `unica-bsl-workspace`
- `unica-v8-runner`
- `unica-rlm-tools-bsl`
- `unica-v8std`

Operation skills should choose these MCP servers by task:

- code search and large-repository exploration: `unica-rlm-tools-bsl` first, `unica-bsl-workspace` for diagnostics and metadata-aware checks;
- build, syntax, tests, dump/load, and infobase operations: `unica-v8-runner` and local 1C platform scripts;
- standards, APK diagnostics, BSLLS/v8-code-style context: `unica-v8std` and references under `references/ai-rules-1c/`.

## Reference Material

- `references/cc-1c-skills/docs/`: original operation-skill DSL and format docs.
- `references/ai-rules-1c/rules/`: BSL and 1C development rules.
- `references/ai-rules-1c/agents/`: role/playbook material.
- `references/ai-rules-1c/openspec-bundle/`: OpenSpec workflow material.
- `references/ai-rules-1c/mcp-servers.json`: MCP playbook source reference.
- `references/v8project.md`: Unica's project-config contract. `v8project.yaml` is the only project configuration format used by packaged skills.

## Verification

From the repository root:

```sh
python3 -m json.tool plugins/unica/.codex-plugin/plugin.json >/dev/null
python3 -m json.tool plugins/unica/.mcp.json >/dev/null
python3 -m json.tool plugins/unica/third-party/manifest.json >/dev/null
bash -n plugins/unica/scripts/*.sh
plugins/unica/scripts/run-bsl-analyzer.sh --version
plugins/unica/scripts/run-v8-runner.sh --help
plugins/unica/scripts/run-rlm-tools-bsl.sh --version
plugins/unica/scripts/run-rlm-bsl-index.sh --version
rg '\\.claude/skills|unica-(setup|bsl|v8-runner|v8std|rlm-tools-bsl)' plugins/unica/skills
codex debug prompt-input 'test'
```
