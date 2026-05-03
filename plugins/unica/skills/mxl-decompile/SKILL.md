---
name: mxl-decompile
description: Декомпиляция табличного документа (MXL) в JSON-определение. Используй когда нужно получить редактируемое описание существующего макета
argument-hint: <TemplatePath> [OutputPath]
allowed-tools:
  - Bash
  - Read
  - Write
  - Glob
---

# /mxl-decompile — Декомпилятор макета в DSL

## MCP routing

- Preferred path: use MCP `unica` tool `unica.mxl.decompile`; `unica` owns XML/JSON DSL work and refreshes related workspace caches after mutations.
- Do not call internal MCP/CLI adapters directly. They are hidden behind `unica` and synchronized by the orchestrator.
- Current Python/PowerShell scripts are fallback implementation details until Rust parity is complete.
- For mutating operations, pass `dryRun: false` only when the user explicitly requested the change; otherwise keep the default dry run.

Принимает Template.xml табличного документа 1С и генерирует компактное JSON-определение (DSL). Обратная операция к `/mxl-compile`.

## Использование

```
/mxl-decompile <TemplatePath> [OutputPath]
```

## Параметры

| Параметр     | Обязательный | Описание                                |
|--------------|:------------:|-----------------------------------------|
| TemplatePath | да           | Путь к Template.xml                     |
| OutputPath   | нет          | Путь для JSON (если не указан — stdout) |

## Команда

```powershell
powershell.exe -NoProfile -File scripts/mxl-decompile.ps1 -TemplatePath "<путь>/Template.xml" [-OutputPath "<путь>.json"]
```

## Рабочий процесс

Декомпиляция существующего макета для анализа или доработки:

1. Claude вызывает `/mxl-decompile` для получения JSON из Template.xml
2. Claude анализирует или модифицирует JSON (добавляет области, меняет стили)
3. Claude вызывает `/mxl-compile` для генерации нового Template.xml
4. Claude вызывает `/mxl-validate` для проверки

## JSON-схема DSL

Полная спецификация формата: **`docs/mxl-dsl-spec.md`** (прочитать через Read tool).

## Генерация имён

Скрипт автоматически генерирует осмысленные имена:

- **Шрифты**: `default`, `bold`, `header`, `small`, `italic` — или описательные имена по свойствам
- **Стили**: `bordered`, `bordered-center`, `bold-right`, `border-top` и т.д. — по комбинации свойств

## Детектирование `rowStyle`

Если в строке есть пустые ячейки (без параметров/текста) и все они имеют одинаковый формат — этот формат распознаётся как `rowStyle`, а пустые ячейки исключаются из вывода.
