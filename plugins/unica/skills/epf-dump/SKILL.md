---
name: epf-dump
description: Разобрать EPF-файл обработки 1С (EPF/ERF) в XML-исходники. Используй когда пользователь просит разобрать, декомпилировать обработку, получить исходники из EPF/ERF файла
argument-hint: <EpfFile>
allowed-tools:
  - Bash
  - Read
  - Glob
  - Grep
---

# /epf-dump — Разборка обработки

## Usage

```
/epf-dump <EpfFile> [OutDir]
```

| Параметр | Обязательный | По умолчанию | Описание                            |
|----------|:------------:|--------------|-------------------------------------|
| EpfFile  | да           | —            | Путь к EPF-файлу                    |
| OutDir   | нет          | `src`        | Каталог для выгрузки исходников     |

## Параметры подключения

Для разборки EPF/ERF требуется информационная база с конфигурацией. Без базы ссылочные типы безвозвратно теряются.

Используй `v8project.yaml` или `V8TR_CONFIG`. Параметры базы бери из `connection`; если конфига нет, спроси строку соединения или сначала выполни `/db-list init`.

## Команда

Native script остаётся основным fallback для dump EPF/ERF, потому что v8-runner не предоставляет отдельной команды разборки внешних обработок.

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
| `-InputFile <путь>` | да | Путь к EPF/ERF-файлу |
| `-OutputDir <путь>` | да | Каталог для выгрузки исходников |
| `-Format <формат>` | нет | `Hierarchical` (по умолч.) / `Plain` |

> `*` — обязательно хотя бы одно подключение. Без базы скрипт завершится с ошибкой (dump в пустой базе безвозвратно теряет ссылочные типы)

## Примеры

```powershell
# Разборка обработки (файловая база)
powershell.exe -NoProfile -File scripts/epf-dump.ps1 -InfoBasePath "C:\Bases\MyDB" -InputFile "build/МояОбработка.epf" -OutputDir "src"

# Серверная база
powershell.exe -NoProfile -File scripts/epf-dump.ps1 -InfoBaseServer "srv01" -InfoBaseRef "MyDB" -UserName "Admin" -Password "secret" -InputFile "build/МояОбработка.epf" -OutputDir "src"
```
