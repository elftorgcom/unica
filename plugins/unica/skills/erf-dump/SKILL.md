---
name: erf-dump
description: Разобрать ERF-файл отчёта 1С в XML-исходники. Используй когда пользователь просит разобрать, декомпилировать отчёт, получить исходники из ERF файла
argument-hint: <ErfFile>
allowed-tools:
  - Bash
  - Read
  - Glob
  - Grep
---

# /erf-dump — Разборка отчёта

## Usage

```
/erf-dump <ErfFile> [OutDir]
```

| Параметр | Обязательный | По умолчанию | Описание                            |
|----------|:------------:|--------------|-------------------------------------|
| ErfFile  | да           | —            | Путь к ERF-файлу                    |
| OutDir   | нет          | `src`        | Каталог для выгрузки исходников     |

## Параметры подключения

Для разборки EPF/ERF требуется информационная база с конфигурацией. Без базы ссылочные типы безвозвратно теряются.

Используй `v8project.yaml` или `V8TR_CONFIG`. Параметры базы бери из `connection`; если конфига нет, спроси строку соединения или сначала выполни `/db-list init`.

## Команда

Native script остаётся основным fallback для dump EPF/ERF, потому что v8-runner не предоставляет отдельной команды разборки внешних отчётов. Используй общий скрипт из epf-dump:

```powershell
powershell.exe -NoProfile -File scripts/epf-dump.ps1 <параметры>
```

### Параметры скрипта

| Параметр | Обязательный | Описание |
|----------|:------------:|----------|
| `-V8Path <путь>` | нет | Каталог bin платформы (или полный путь к 1cv8.exe) |
| `-InfoBasePath <путь>` | * | Файловая база |
| `-InfoBaseServer <сервер>` | * | Сервер 1С (для серверной базы) |
| `-InfoBaseRef <имя>` | * | Имя базы на сервере |
| `-UserName <имя>` | нет | Имя пользователя |
| `-Password <пароль>` | нет | Пароль |
| `-InputFile <путь>` | да | Путь к ERF-файлу |
| `-OutputDir <путь>` | да | Каталог для выгрузки исходников |
| `-Format <формат>` | нет | `Hierarchical` (по умолч.) / `Plain` |

> `*` — обязательно хотя бы одно подключение. Без базы скрипт завершится с ошибкой (dump в пустой базе безвозвратно теряет ссылочные типы)

## Примеры

```powershell
# Разборка отчёта (файловая база)
powershell.exe -NoProfile -File scripts/epf-dump.ps1 -InfoBasePath "C:\Bases\MyDB" -InputFile "build/МойОтчёт.erf" -OutputDir "src"

# Серверная база
powershell.exe -NoProfile -File scripts/epf-dump.ps1 -InfoBaseServer "srv01" -InfoBaseRef "MyDB" -UserName "Admin" -Password "secret" -InputFile "build/МойОтчёт.erf" -OutputDir "src"
```
