---
name: role-validate
description: Валидация роли 1С. Используй после создания или модификации роли для проверки корректности
argument-hint: <RightsPath> [-Detailed] [-MaxErrors 30]
allowed-tools:
  - Bash
  - Read
---

# /role-validate — валидация роли 1С

## MCP routing

- Preferred path: use MCP `unica` tool `unica.role.validate`; `unica` owns XML/JSON DSL work and refreshes related workspace caches after mutations.
- Do not call internal MCP/CLI adapters directly. They are hidden behind `unica` and synchronized by the orchestrator.
- Current Python/PowerShell scripts are fallback implementation details until Rust parity is complete.
- For mutating operations, pass `dryRun: false` only when the user explicitly requested the change; otherwise keep the default dry run.

Проверяет корректность `Rights.xml` роли: формат XML, namespace, глобальные флаги, типы объектов, имена прав, RLS-ограничения, шаблоны. Опционально проверяет метаданные роли (UUID, имя, синоним).

## Параметры

| Параметр     | Обяз. | Умолч. | Описание                                        |
|--------------|:-----:|---------|-------------------------------------------------|
| RightsPath   | да    | —       | Путь к роли (директория или `Rights.xml`)        |
| Detailed     | нет   | —       | Подробный вывод (все проверки, включая успешные)  |
| MaxErrors    | нет   | 30      | Макс. ошибок до остановки (по умолчанию 30)      |
| OutFile      | нет   | —       | Записать результат в файл (UTF-8 BOM)            |

## Команда

```powershell
powershell.exe -NoProfile -File scripts/role-validate.ps1 -RightsPath "Roles/МояРоль"
```
