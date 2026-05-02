# Веб-публикация 1С

Навыки `/web-*` публикуют информационные базы 1С через Apache HTTP Server. v8-runner не покрывает Apache publication, поэтому эти навыки используют native scripts, но базовое подключение берут из `v8project.yaml` / `V8TR_CONFIG`.

## Навыки

| Навык | Назначение |
| --- | --- |
| `/web-publish` | Создать `default.vrd`, обновить `httpd.conf`, запустить Apache |
| `/web-info` | Показать статус Apache, порт, публикации и ошибки |
| `/web-stop` | Остановить Apache |
| `/web-unpublish` | Удалить одну или все публикации |

## Источники параметров

- База: `connection` из `v8project.yaml`.
- Apache: `UNICA_APACHE_PATH`, явный `-ApachePath` или `tools/apache24`.
- Пользователь/пароль: из connection string, явных параметров или ответа пользователя.
- Имя публикации: явный `-AppName` или имя базы/каталога.

## Рабочий цикл

```text
v8-runner build -> /web-publish -> браузер
                  /web-info
                  /web-stop
                  /web-unpublish
```

## Примеры

```powershell
# публикация файловой базы из connection
powershell.exe -NoProfile -File scripts/web-publish.ps1 -InfoBasePath "C:\Bases\MyDB" -UserName "Admin"

# публикация на другом порту
powershell.exe -NoProfile -File scripts/web-publish.ps1 -InfoBasePath "C:\Bases\MyDB" -AppName "mydb" -Port 9090

# статус
powershell.exe -NoProfile -File scripts/web-info.ps1

# остановка
powershell.exe -NoProfile -File scripts/web-stop.ps1
```

## Спецификации

- [web-spec.md](web-spec.md) — VRD, httpd.conf, wsap24.dll, portable Apache
- [v8project-guide.md](v8project-guide.md) — проектный конфиг v8-runner
