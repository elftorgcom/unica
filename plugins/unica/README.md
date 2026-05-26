# Unica Codex Plugin

Unica - плагин Codex для практической 1С-разработки.

Он даёт Codex навыки для работы с конфигурациями, расширениями, формами,
ролями, СКД, MXL, внешними обработками и отчетами, BSL-кодом и runtime-задачами
через v8-runner.

## Windows-first в 0.4.2

Windows-пакет работает нативно через PowerShell 7:

```powershell
pwsh -NoProfile -ExecutionPolicy Bypass -File plugins/unica/scripts/run-unica.ps1 --help
```

Для Windows runtime не нужны:

- WSL;
- Git Bash;
- MSYS2;
- `bash` / `sh`;
- любое другое POSIX shell-окружение.

Это относится и к MCP-сценариям: публичный stdio MCP-сервер запускается через
`run-unica.ps1`, а внутренние инструменты вызываются через PowerShell wrappers.

## MCP

Публичный MCP-сервер один:

- `unica`

`.mcp.json` для Windows-пакета указывает на:

```json
{
  "command": "pwsh",
  "args": ["-NoProfile", "-File", "./plugins/unica/scripts/run-unica.ps1"]
}
```

Остальные инструменты (`v8-runner`, `bsl-analyzer`, `rlm-tools-bsl`,
`rlm-bsl-index`) - внутренние adapters за границей MCP `unica`.

## Навыки

Основные группы skills:

- `cf-*`, `cfe-*` - конфигурации и расширения;
- `meta-*`, `form-*`, `role-*`, `skd-*`, `mxl-*` - объекты, формы, роли, СКД и макеты;
- `epf-*` - внешние обработки и отчеты;
- `v8-runner`, `db-auth-check` - runtime, базы, сборка, загрузка;
- `code-search`, `code-diagnostics`, `code-review` - BSL-код;
- `platform-help`, `bsp-patterns`, `query-optimize`, `test-authoring` - справка, БСП, запросы и тесты.

## Установка

Windows:

```powershell
$env:UNICA_REPO = "elftorgcom/unica"
irm https://github.com/elftorgcom/unica/releases/download/v0.4.2/install-unica.ps1 -OutFile install-unica.ps1
pwsh -NoProfile -ExecutionPolicy Bypass -File .\install-unica.ps1 -Version v0.4.2
```

Linux/macOS:

```sh
curl -fsSL https://github.com/elftorgcom/unica/releases/download/v0.4.2/install-unica.sh | sh -s -- --version v0.4.2
```

## Проверка пакета

После распаковки `unica-codex-marketplace-win-x64.zip`:

```powershell
pwsh -NoProfile -ExecutionPolicy Bypass -File plugins/unica/scripts/run-bsl-analyzer.ps1 --version
pwsh -NoProfile -ExecutionPolicy Bypass -File plugins/unica/scripts/run-v8-runner.ps1 --help
pwsh -NoProfile -ExecutionPolicy Bypass -File plugins/unica/scripts/run-unica.ps1 --help
```

MCP smoke:

```powershell
'{"jsonrpc":"2.0","id":1,"method":"tools/list"}' |
  pwsh -NoProfile -File plugins/unica/scripts/run-unica.ps1
```

## Внутренние инструменты

Release-пакет содержит pinned binaries для целевой платформы. Они не лежат в
исходниках репозитория и собираются в GitHub Actions.

Запуск идёт через wrappers:

- `scripts/run-tool.ps1` - основной Windows wrapper;
- `scripts/run-unica.ps1` - публичный Windows MCP entrypoint;
- `scripts/run-tool.sh` и shell wrappers - Linux/macOS runtime.

Wrappers читают `third-party/manifest.json`, проверяют целевую платформу и
SHA-256 перед запуском бинарника.

## Поддержка

Подробности для сопровождающих:

- `references/tooling/internal-package.md`
- `third-party/tools.lock.json`
- `.github/workflows/unica-plugin-release.yml`

Лицензия: `LGPL-3.0-or-later`.
