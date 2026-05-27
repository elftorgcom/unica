# Changelog

## 0.4.2

Windows-first release readiness:

- Added native PowerShell launchers for Windows package execution.
- Removed any WSL, Git Bash, or MSYS2 runtime requirement from the Windows
  package contract.
- Added the target-specific Windows MCP entrypoint
  `plugins/unica/scripts/run-unica.ps1`; the public MCP surface remains the
  single `unica` stdio server.
- Updated installer and CI packaging expectations for target-specific
  marketplace artifacts, including the Windows `win-x64` package.
- Fixed Windows package MCP startup by generating and repairing `win-x64`
  `.mcp.json` with `pwsh -NoProfile -File ./plugins/unica/scripts/run-unica.ps1`
  instead of a direct bundled binary command.
- Documented known runtime prerequisites: Codex CLI, PowerShell 7 (`pwsh`), the
  local 1C platform for real 1C operations, and network access for remote
  standards lookup.
- Clarified that `rlm-tools-bsl` `service.json` discovery failures and remote
  standards endpoint failures are runtime configuration or network issues, not
  Windows shell compatibility requirements.
