---
name: cfe-validate
description: Валидация расширения конфигурации 1С (CFE). Используй после создания или модификации расширения для проверки корректности
argument-hint: <ExtensionPath> [-Detailed] [-MaxErrors 30]
allowed-tools:
  - Bash
  - Read
  - Glob
---

# /cfe-validate — валидация расширения конфигурации (CFE)

## MCP routing

- Preferred path: use MCP `unica` tool `unica.cfe.validate`; `unica` owns XML/JSON DSL work and refreshes related workspace caches after mutations.
- Do not call internal MCP/CLI adapters directly. They are hidden behind `unica` and synchronized by the orchestrator.
- Current Python/PowerShell scripts are fallback implementation details until Rust parity is complete.
- For mutating operations, pass `dryRun: false` only when the user explicitly requested the change; otherwise keep the default dry run.

Проверяет структурную корректность расширения: XML-формат, свойства, состав, заимствованные объекты. Аналог `/cf-validate`, но для расширений.

## Параметры

| Параметр      | Обяз. | Умолч. | Описание                                        |
|---------------|:-----:|---------|-------------------------------------------------|
| ExtensionPath | да    | —       | Путь к каталогу или Configuration.xml расширения |
| Detailed      | нет   | —       | Подробный вывод (все проверки, включая успешные)  |
| MaxErrors     | нет   | 30      | Остановиться после N ошибок                      |
| OutFile       | нет   | —       | Записать результат в файл                        |

## Команда

```powershell
powershell.exe -NoProfile -File scripts/cfe-validate.ps1 -ExtensionPath "src"
powershell.exe -NoProfile -File scripts/cfe-validate.ps1 -ExtensionPath "src/Configuration.xml"
```
