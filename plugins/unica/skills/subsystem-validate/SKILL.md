---
name: subsystem-validate
description: Валидация подсистемы 1С. Используй после создания или модификации подсистемы для проверки корректности
argument-hint: <SubsystemPath> [-Detailed] [-MaxErrors 30]
allowed-tools:
  - Bash
  - Read
  - Glob
---

# /subsystem-validate — валидация подсистемы 1С

## MCP routing

- Preferred path: use MCP `unica` tool `unica.subsystem.validate`; `unica` owns XML/JSON DSL work and refreshes related workspace caches after mutations.
- Do not call internal MCP/CLI adapters directly. They are hidden behind `unica` and synchronized by the orchestrator.
- Current Python/PowerShell scripts are fallback implementation details until Rust parity is complete.
- For mutating operations, pass `dryRun: false` only when the user explicitly requested the change; otherwise keep the default dry run.

Проверяет структурную корректность XML-файла подсистемы из выгрузки конфигурации.

## Параметры

| Параметр      | Обяз. | Умолч. | Описание                                  |
|---------------|:-----:|---------|--------------------------------------------|
| SubsystemPath | да    | —       | Путь к XML-файлу подсистемы                |
| Detailed      | нет   | —       | Подробный вывод (все проверки, включая успешные) |
| MaxErrors     | нет   | 30      | Остановиться после N ошибок                |
| OutFile       | нет   | —       | Записать результат в файл                  |

## Команда

```powershell
powershell.exe -NoProfile -File "scripts/subsystem-validate.ps1" -SubsystemPath "Subsystems/Продажи"
powershell.exe -NoProfile -File "scripts/subsystem-validate.ps1" -SubsystemPath "Subsystems/Продажи.xml"
```
