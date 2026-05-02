---
name: erf-build
description: Собрать внешний отчёт 1С (ERF) из XML-исходников. Используй когда пользователь просит собрать, скомпилировать отчёт или получить ERF файл из исходников
argument-hint: <ReportName>
allowed-tools:
  - Bash
  - Read
  - Glob
  - Grep
---

# /erf-build — Сборка отчёта

## Usage

```
/erf-build <ReportName> [SrcDir] [OutDir]
```

| Параметр   | Обязательный | По умолчанию | Описание                             |
|------------|:------------:|--------------|--------------------------------------|
| ReportName | да           | —            | Имя отчёта (имя корневого XML)       |
| SrcDir     | нет          | `src`        | Каталог исходников                   |
| OutDir     | нет          | `build`      | Каталог для результата               |

## Параметры подключения

Используй `v8project.yaml` или `V8TR_CONFIG`. Если external report описан отдельным `source-set`, сначала попробуй собрать артефакт через v8-runner:

```sh
../../scripts/run-v8-runner.sh make --source-set <name> --output build/МойОтчёт.erf
```

Если v8-runner не покрывает конкретную ERF-сборку, используй native script ниже. Параметры базы бери из `connection` в `v8project.yaml`; если конфига нет, скрипт может создать временную базу со stub-метаданными.

## Команда

Fallback использует общий скрипт из epf-build:

```powershell
powershell.exe -NoProfile -File scripts/epf-build.ps1 <параметры>
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
| `-SourceFile <путь>` | да | Путь к корневому XML-файлу исходников |
| `-OutputFile <путь>` | да | Путь к выходному ERF-файлу |

> `*` — опционально. Если не указано — автоматически создаётся временная база со заглушками метаданных

## Примеры

```powershell
# Сборка отчёта (файловая база)
powershell.exe -NoProfile -File scripts/epf-build.ps1 -InfoBasePath "C:\Bases\MyDB" -SourceFile "src/МойОтчёт.xml" -OutputFile "build/МойОтчёт.erf"

# Серверная база
powershell.exe -NoProfile -File scripts/epf-build.ps1 -InfoBaseServer "srv01" -InfoBaseRef "MyDB" -UserName "Admin" -Password "secret" -SourceFile "src/МойОтчёт.xml" -OutputFile "build/МойОтчёт.erf"
```
