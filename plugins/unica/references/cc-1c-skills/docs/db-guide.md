# Базы данных 1С

Группа `/db-*` работает через `v8project.yaml` и v8-runner. Старые native Designer scripts остаются только fallback для режимов, которых нет в v8-runner.

## Навыки

| Навык | Основной backend |
| --- | --- |
| `/db-list` | `v8-runner config init`, чтение/правка `v8project.yaml` |
| `/db-create` | `v8-runner config init` + `v8-runner init` |
| `/db-dump-cf` | `v8-runner make --output` |
| `/db-load-cf` | `v8-runner load --path` |
| `/db-dump-xml` | `v8-runner dump --mode ...` |
| `/db-load-xml` | `v8-runner build` |
| `/db-update` | `v8-runner build` или `v8-runner load --mode update` |
| `/db-run` | `v8-runner launch` |
| `/db-load-git` | `v8-runner build` |

## Рабочий цикл

```text
v8project.yaml -> /db-create -> /db-load-xml -> /db-update -> /db-run
                                 ^             |
                                 |             v
                             правки XML <- /db-dump-xml
```

## Примеры

```sh
# создать конфиг и базу/workspace
plugins/unica/scripts/run-v8-runner.sh config init --connection '/F/Users/me/1c-bases/dev'
plugins/unica/scripts/run-v8-runner.sh init

# загрузить исходники
plugins/unica/scripts/run-v8-runner.sh build

# выгрузить исходники
plugins/unica/scripts/run-v8-runner.sh dump --mode full

# экспортировать CF
plugins/unica/scripts/run-v8-runner.sh make --output build/config.cf

# загрузить CF и применить обновление
plugins/unica/scripts/run-v8-runner.sh load --path build/config.cf --mode update

# запустить клиент
plugins/unica/scripts/run-v8-runner.sh launch thin
```

## Спецификации

- [v8project-guide.md](v8project-guide.md) — формат `v8project.yaml`
- [build-spec.md](build-spec.md) — пакетный режим платформы 1С
