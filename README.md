# Unica для Codex

Windows-first форк плагина Unica для задач 1С:Предприятия в Codex.

Unica добавляет в Codex навыки и MCP-инструменты для повседневной 1С-разработки:
метаданные, формы, роли, СКД, MXL, EPF/ERF, сборка и загрузка конфигураций,
поиск и диагностика BSL-кода, работа с runtime-сценариями через v8-runner.

## Что изменено в этом форке

- Выпуск `0.4.2` адаптирован под Windows-first runtime.
- Windows-пакет запускается через PowerShell 7 (`pwsh`).
- Для MCP-сценариев на Windows больше не нужны `bash`, `sh`, WSL, Git Bash,
  MSYS2 или другое POSIX shell-окружение.
- Единственный публичный MCP-сервер по-прежнему называется `unica`.
- Внутренние бинарные инструменты запускаются только через checksum-wrapper'ы.
- GitHub Actions собирает и публикует release assets для `win-x64`, `linux-x64`
  и `darwin-arm64`, а также установщики `install-unica.ps1` и
  `install-unica.sh`.

## Быстрая установка на Windows

Требования:

- Windows x64;
- Codex CLI;
- PowerShell 7+ (`pwsh`);
- платформа 1С нужна только для реальных операций с базами и конфигурациями.

Установка из релиза форка:

```powershell
$env:UNICA_REPO = "elftorgcom/unica"
irm https://github.com/elftorgcom/unica/releases/download/v0.4.2/install-unica.ps1 -OutFile install-unica.ps1
pwsh -NoProfile -ExecutionPolicy Bypass -File .\install-unica.ps1 -Version v0.4.2
```

Установщик скачает `unica-codex-marketplace-win-x64.zip`, зарегистрирует
marketplace `unica-local`, обновит cache Codex и включит `unica@unica-local`.

## Проверка

```powershell
pwsh -NoProfile -ExecutionPolicy Bypass -File "$env:CODEX_HOME\marketplaces\unica-local\plugins\unica\scripts\run-unica.ps1" --help
codex debug prompt-input "test"
```

В свежем prompt-input должны быть видны `Unica` и навыки вида
`unica:v8-runner`, `unica:meta-compile`, `unica:epf-bsp-init`.

Проверка MCP без Codex:

```powershell
$server = "$env:CODEX_HOME\marketplaces\unica-local\plugins\unica\scripts\run-unica.ps1"
'{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"smoke","version":"0"}}}' |
  pwsh -NoProfile -File $server
```

Ожидаемый ответ содержит `serverInfo.name = unica` и версию `0.4.2`.

## Release assets

- `install-unica.ps1` - установщик для Windows.
- `install-unica.sh` - установщик для Linux/macOS.
- `unica-codex-marketplace-win-x64.zip` - основной Windows-пакет.
- `unica-codex-marketplace-linux-x64.*`
- `unica-codex-marketplace-darwin-arm64.*`

Страница релиза:

<https://github.com/elftorgcom/unica/releases/tag/v0.4.2>

## Для разработки

```powershell
git clone https://github.com/elftorgcom/unica.git
cd unica
python -m unittest discover -s tests/ci
```

Основные файлы:

- `plugins/unica/skills/` - навыки Codex для 1С.
- `plugins/unica/scripts/` - Windows PowerShell и shell launchers.
- `plugins/unica/.mcp.json` - публичный MCP entrypoint `unica`.
- `plugins/unica/third-party/tools.lock.json` - pinned версии внешних инструментов.
- `.github/workflows/unica-plugin-release.yml` - сборка и публикация пакетов.

## Лицензия

Unica распространяется по `LGPL-3.0-or-later`. См. `LICENSE`.
