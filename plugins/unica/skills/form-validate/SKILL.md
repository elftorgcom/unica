---
name: form-validate
description: Валидация управляемой формы 1С. Используй после создания или модификации формы для проверки корректности. При наличии BaseForm автоматически проверяет callType и ID расширений
argument-hint: <FormPath> [-Detailed] [-MaxErrors 30]
allowed-tools:
  - Bash
  - Read
  - Glob
---

# /form-validate — валидация управляемой формы 1С

## MCP routing

- Preferred path: use MCP `unica` tool `unica.form.validate`; `unica` owns XML/JSON DSL work and refreshes related workspace caches after mutations.
- Do not call internal MCP/CLI adapters directly. They are hidden behind `unica` and synchronized by the orchestrator.
- Current Python/PowerShell scripts are fallback implementation details until Rust parity is complete.
- For mutating operations, pass `dryRun: false` only when the user explicitly requested the change; otherwise keep the default dry run.

Проверяет Form.xml на структурные ошибки: уникальность ID, наличие companion-элементов, корректность ссылок DataPath и команд.

## Параметры

| Параметр  | Обяз. | Умолч. | Описание                                |
|-----------|:-----:|---------|-----------------------------------------|
| FormPath  | да    | —       | Путь к файлу Form.xml                   |
| Detailed  | нет   | —       | Подробный вывод (все проверки, включая успешные) |
| MaxErrors | нет   | 30      | Остановиться после N ошибок              |

## Команда

```powershell
powershell.exe -NoProfile -File scripts/form-validate.ps1 -FormPath "Catalogs/Номенклатура/Forms/ФормаЭлемента"
powershell.exe -NoProfile -File scripts/form-validate.ps1 -FormPath "src/МояОбработка/Forms/Форма/Ext/Form.xml"
```

