---
name: cfe-diff
description: Анализ расширения конфигурации 1С (CFE) — состав, заимствованные объекты, перехватчики, проверка переноса. Используй когда нужно понять что содержит расширение или проверить перенесены ли вставки в конфигурацию
argument-hint: -ExtensionPath <path> -ConfigPath <path> [-Mode A|B]
allowed-tools:
  - Bash
  - Read
  - Glob
---

# /cfe-diff — Анализ расширения конфигурации

## MCP routing

- Preferred path: use MCP `unica` tool `unica.cfe.diff`; `unica` owns XML/JSON DSL work and refreshes related workspace caches after mutations.
- Do not call internal MCP/CLI adapters directly. They are hidden behind `unica` and synchronized by the orchestrator.
- Current Python/PowerShell scripts are fallback implementation details until Rust parity is complete.
- For mutating operations, pass `dryRun: false` only when the user explicitly requested the change; otherwise keep the default dry run.

Анализирует расширение в двух режимах: обзор изменений (Mode A) или проверка переноса (Mode B).

## Параметры

| Параметр | Описание | По умолчанию |
|----------|----------|--------------|
| `ExtensionPath` | Путь к расширению (обязат.) | — |
| `ConfigPath` | Путь к конфигурации (обязат.) | — |
| `Mode` | `A` (обзор) / `B` (проверка переноса) | `A` |

## Команда

```powershell
powershell.exe -NoProfile -File scripts/cfe-diff.ps1 -ExtensionPath src -ConfigPath C:\cfsrc\erp -Mode A
```

## Mode A — обзор расширения

Для каждого объекта показывает:
- `[BORROWED]` — заимствованный: перехватчики (`&Перед`, `&После`, `&ИзменениеИКонтроль`, `&Вместо`), собственные реквизиты/ТЧ/формы
- `[OWN]` — собственный: количество реквизитов, ТЧ, форм

Для каждой формы заимствованного объекта показывается:
- `(borrowed)` / `(own)` — заимствованная или собственная форма
- callType-события формы и элементов
- callType на командах

## Mode B — проверка переноса

Для каждого `&ИзменениеИКонтроль` извлекает блоки `#Вставка`/`#КонецВставки` из расширения и ищет их в соответствующем модуле конфигурации.

Статусы:
- `[TRANSFERRED]` — код найден в конфигурации
- `[NOT_TRANSFERRED]` — код не найден
- `[NEEDS_REVIEW]` — нет блоков `#Вставка` или модуль конфигурации не найден

## Примеры

```powershell
# Обзор — что изменено в расширении
... -ExtensionPath src -ConfigPath C:\cfsrc\erp -Mode A

# Проверка переноса — все ли #Вставка перенесены
... -ExtensionPath src -ConfigPath C:\cfsrc\erp -Mode B
```
