---
name: subsystem-compile
description: Создать подсистему 1С — XML-исходники из JSON-определения. Используй когда нужно добавить подсистему (раздел) в конфигурацию
argument-hint: "[-DefinitionFile <json> | -Value <json-string>] -OutputDir <ConfigDir> [-Parent <path>]"
allowed-tools:
  - Bash
  - Read
  - Write
  - Glob
---

# /subsystem-compile — генерация подсистемы из JSON

## MCP routing

- Preferred path: use MCP `unica` tool `unica.subsystem.compile`; `unica` owns XML/JSON DSL work and refreshes related workspace caches after mutations.
- Do not call internal MCP/CLI adapters directly. They are hidden behind `unica` and synchronized by the orchestrator.
- Current Python/PowerShell scripts are fallback implementation details until Rust parity is complete.
- For mutating operations, pass `dryRun: false` only when the user explicitly requested the change; otherwise keep the default dry run.

Принимает JSON-определение подсистемы → генерирует XML + файловую структуру + регистрирует в родителе (Configuration.xml или родительская подсистема).

## Параметры и команда

| Параметр | Описание |
|----------|----------|
| `DefinitionFile` | Путь к JSON-файлу определения |
| `Value` | Инлайн JSON-строка (альтернатива DefinitionFile) |
| `OutputDir` | Корень выгрузки (где `Subsystems/`, `Configuration.xml`) |
| `Parent` | Путь к XML родительской подсистемы (для вложенных) |
| `NoValidate` | Пропустить авто-валидацию |

```powershell
powershell.exe -NoProfile -File 'scripts/subsystem-compile.ps1' -Value '<json>' -OutputDir '<ConfigDir>'
```

## JSON-определение

```json
{
  "name": "МояПодсистема",
  "synonym": "Моя подсистема",
  "comment": "",
  "includeInCommandInterface": true,
  "useOneCommand": false,
  "explanation": "Описание раздела",
  "picture": "CommonPicture.МояКартинка",
  "content": ["Catalog.Товары", "Document.Заказ"]
}
```

Минимально: только `name`. Остальное — дефолты.

## Примеры

```powershell
# Минимальная подсистема
... -Value '{"name":"Тест"}' -OutputDir config/

# С составом и картинкой
... -Value '{"name":"Продажи","content":["Catalog.Товары","Report.Продажи"],"picture":"CommonPicture.Продажи"}' -OutputDir config/

# Вложенная подсистема
... -Value '{"name":"Дочерняя"}' -OutputDir config/ -Parent config/Subsystems/Продажи.xml
```

