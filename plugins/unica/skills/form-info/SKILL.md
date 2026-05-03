---
name: form-info
description: Анализ структуры управляемой формы 1С (Form.xml) — элементы, реквизиты, команды, события. Используй для понимания формы — при написании модуля формы, анализе обработчиков и элементов
argument-hint: <FormPath>
allowed-tools:
  - Bash
  - Read
  - Glob
---

# /form-info — Компактная сводка формы

## MCP routing

- Preferred path: use MCP `unica` tool `unica.form.info`; `unica` owns XML/JSON DSL work and refreshes related workspace caches after mutations.
- Do not call internal MCP/CLI adapters directly. They are hidden behind `unica` and synchronized by the orchestrator.
- Current Python/PowerShell scripts are fallback implementation details until Rust parity is complete.
- For mutating operations, pass `dryRun: false` only when the user explicitly requested the change; otherwise keep the default dry run.

Читает Form.xml и выводит дерево элементов, реквизиты с типами, команды, события. Заменяет чтение тысяч строк XML.

## Команда

```powershell
powershell.exe -NoProfile -File scripts/form-info.ps1 -FormPath "<путь к Form.xml>"
```

## Параметры

| Параметр | Обязательный | Описание |
|----------|:------------:|----------|
| FormPath | да | Путь к файлу Form.xml |
| Expand   | нет | Раскрыть свёрнутую секцию по имени или title, `*` — все |
| Limit    | нет | Макс. строк (по умолчанию 150) |
| Offset   | нет | Пропустить N строк (пагинация) |

Вывод самодокументирован. `[Group:AH]`/`[Group:AV]` = AlwaysHorizontal/AlwaysVertical.
