# Конфигурация проекта v8project.yaml

`v8project.yaml` — единый проектный конфиг Unica и v8-runner. Для нестандартного расположения используй `V8TR_CONFIG`.

## Создание

```sh
plugins/unica/scripts/run-v8-runner.sh config init --connection '/F/Users/me/1c-bases/dev'
plugins/unica/scripts/run-v8-runner.sh config init --connection '/Sserver/ref'
```

## Минимальный пример

```yaml
basePath: '.'
workPath: 'build'
format: DESIGNER
builder: DESIGNER
connection: '/F/Users/me/1c-bases/dev'
source-set:
  - name: main
    type: CONFIGURATION
    path: 'src'
build:
  partialLoadThreshold: 20
```

## Основные поля

| Поле | Назначение |
| --- | --- |
| `basePath` | Корень проекта для относительных путей |
| `workPath` | Рабочий каталог v8-runner |
| `format` | Формат исходников, например `DESIGNER` или `EDT` |
| `builder` | Backend сборки, например `DESIGNER` или `IBCMD` |
| `connection` | Строка подключения к базе 1С |
| `source-set` | Набор исходников конфигурации, расширения или внешнего артефакта |
| `build` | Настройки загрузки/сборки |

## Правила Unica

- `V8TR_CONFIG` имеет приоритет над `./v8project.yaml`.
- Все db-skills используют v8-runner поверх этого файла.
- Для source path используй `source-set[].path`, а не отдельный project registry.
- Для web helpers используй `connection` из этого файла, но Apache path задавай `UNICA_APACHE_PATH`, `-ApachePath` или `tools/apache24`.
- Для записи видео используй `FFMPEG_PATH` или явный `ffmpegPath` в вызове `startRecording()`.

## Команды

| Задача | Команда |
| --- | --- |
| Инициализация базы/workspace | `v8-runner init` |
| Загрузка исходников | `v8-runner build` |
| Полная пересборка | `v8-runner build --full-rebuild` |
| Выгрузка XML | `v8-runner dump --mode full|incremental|partial` |
| Экспорт CF/CFE | `v8-runner make --output <file>` |
| Загрузка CF/CFE | `v8-runner load --path <file> --mode load|merge|update` |
| Запуск 1С | `v8-runner launch thin|thick|designer|ordinary` |
| Синтаксическая проверка | `v8-runner syntax designer-config` или `designer-modules` |

Более короткая внутренняя памятка: `plugins/unica/references/v8project.md`.
