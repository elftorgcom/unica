---
name: form-remove
description: Удалить форму из объекта 1С (обработка, отчёт, справочник, документ и др.)
argument-hint: <ObjectName> <FormName>
disable-model-invocation: true
allowed-tools:
  - Bash
  - Read
  - Write
  - Edit
  - Glob
  - Grep
---

# /form-remove — Удаление формы

## MCP routing

- Preferred path: use MCP `unica` tool `unica.form.remove`; `unica` owns XML/JSON DSL work and refreshes related workspace caches after mutations.
- Do not call internal MCP/CLI adapters directly. They are hidden behind `unica` and synchronized by the orchestrator.
- Current Python/PowerShell scripts are fallback implementation details until Rust parity is complete.
- For mutating operations, pass `dryRun: false` only when the user explicitly requested the change; otherwise keep the default dry run.

Удаляет форму и убирает её регистрацию из корневого XML объекта.

## Usage

```
/form-remove <ObjectName> <FormName>
```

| Параметр   | Обязательный | По умолчанию | Описание                            |
|------------|:------------:|--------------|-------------------------------------|
| ObjectName | да           | —            | Имя объекта                         |
| FormName   | да           | —            | Имя формы для удаления              |
| SrcDir     | нет          | `src`        | Каталог исходников                  |

## Команда

```powershell
powershell.exe -NoProfile -File scripts/remove-form.ps1 -ObjectName "<ObjectName>" -FormName "<FormName>" [-SrcDir "<SrcDir>"]
```

## Что удаляется

```
<SrcDir>/<ObjectName>/Forms/<FormName>.xml     # Метаданные формы
<SrcDir>/<ObjectName>/Forms/<FormName>/         # Каталог формы (рекурсивно)
```

## Что модифицируется

- `<SrcDir>/<ObjectName>.xml` — убирается `<Form>` из `ChildObjects`
- Если удаляемая форма была DefaultForm — очищается значение DefaultForm
