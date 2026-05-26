# Unica

Unica - это плагин для Codex, который помогает работать с проектами 1С:Предприятие.

Обычным языком: репозиторий содержит набор инструкций, сценариев и подключаемых инструментов, чтобы Codex мог выполнять типовые задачи 1С-разработчика: создавать объекты конфигурации, собирать внешние обработки и отчеты, обновлять базы, запускать проверки и искать код в больших 1С-проектах.

## Что в этой репе

- `plugins/unica/skills/` - прикладные навыки Codex: формы, метаданные, EPF/ERF, базы, роли, СКД, веб-публикация и другие задачи 1С.
- `plugins/unica/.mcp.json` - MCP-подключения для поиска кода, работы с инструментами 1С и справочными материалами.
- `plugins/unica/scripts/` - безопасные запускатели bundled-инструментов.
- `plugins/unica/third-party/tools.lock.json` - единый список версий внешних инструментов.
- `.github/workflows/unica-plugin-release.yml` - сборка готового пакета плагина для установки.

Исходники в репозитории не хранят готовые бинарные утилиты. Они собираются в GitHub Actions и попадают в готовый marketplace-пакет.

## Для кого

- Для 1С-разработчиков, которые хотят использовать Codex как помощника по реальным задачам разработки.
- Для тех, кто поддерживает или расширяет сам плагин Unica.
- Для команд, которым нужен воспроизводимый набор 1С-инструментов внутри Codex.

## Установка

Одна команда скачивает installer из последнего GitHub Release, определяет
платформу, скачивает нужный пакет Unica и устанавливает его в Codex:

```sh
curl -fsSL https://github.com/IngvarConsulting/unica/releases/latest/download/install-unica.sh | sh
```

Для установки конкретного релиза:

```sh
curl -fsSL https://github.com/IngvarConsulting/unica/releases/latest/download/install-unica.sh | sh -s -- --version v0.4.2
```

Release assets собираются отдельно под платформы:

- `unica-codex-marketplace-darwin-arm64.tar.gz`
- `unica-codex-marketplace-linux-x64.tar.gz`
- `unica-codex-marketplace-win-x64.zip`

Installer выбирает нужный архив, регистрирует marketplace `unica-local`,
обновляет cache Codex и включает `unica@unica-local`.

### Windows-first runtime

Windows package `unica-codex-marketplace-win-x64.zip` is designed to run from
native PowerShell 7 (`pwsh`). It does not require WSL, Git Bash, MSYS2, or any
other POSIX shell at runtime. The public MCP entrypoint for the packaged Windows
plugin is:

```powershell
pwsh -NoProfile -ExecutionPolicy Bypass -File plugins/unica/scripts/run-unica.ps1 --help
```

The public MCP contract is still a single stdio server named `unica`; other
bundled tools are private adapters behind that server. For Windows smoke checks
against an extracted package, use native PowerShell launchers:

```powershell
pwsh -NoProfile -ExecutionPolicy Bypass -File plugins/unica/scripts/run-tool.ps1 unica --help
pwsh -NoProfile -ExecutionPolicy Bypass -File plugins/unica/scripts/run-unica.ps1 --help
codex debug prompt-input 'test'
```

Runtime prerequisites are intentionally external: Codex CLI, PowerShell 7,
local 1C platform binaries for real 1C operations, and network access for remote
standards lookup. Failures in `rlm-tools-bsl` `service.json` discovery or the
external standards endpoint are runtime configuration or network issues, not
requirements to install WSL/Git Bash/MSYS2.

Проверка:

```sh
codex debug prompt-input 'test'
```

В выводе должны быть видны plugin `Unica` и навыки вида `unica:meta-compile`, `unica:v8-runner`, `unica:epf-bsp-init`.

## Установка из исходников для разработки

Этот режим нужен, если вы меняете сам плагин:

```sh
git clone https://github.com/IngvarConsulting/unica.git
cd unica
scripts/dev/install-local-unica.sh
```

Скрипт соберет пакет под текущую машину из локальных исходников, установит его
в Codex как `unica-local` и проверит свежую сессию через
`codex debug prompt-input`.

## Что нужно для работы

- Установленный Codex CLI.
- Для реальных операций с базами и конфигурациями - установленная платформа 1С.
- Для Windows-сценариев и Windows MCP runtime - PowerShell 7 (`pwsh`).
- Для macOS/Linux MCP-сценариев - shell-окружение.

## Где смотреть детали

- Техническое описание плагина: `plugins/unica/README.md`.
- Внутренняя схема инструментов и сборки: `plugins/unica/references/tooling/internal-package.md`.
- Список pinned-инструментов: `plugins/unica/third-party/tools.lock.json`.

Официальная публикация в публичный каталог Codex будет отдельным шагом, когда OpenAI откроет self-serve публикацию плагинов. Сейчас репозиторий готовит воспроизводимый marketplace-пакет, который можно устанавливать как локальный или Git-backed marketplace.
