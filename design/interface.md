# Описание интерфейса между `Клиент`ом и `Сервер`ом

## Запрос `DeviceInfo`

- Запрос от `Клиент`а `Сервер`у

```json
    {
        "cot": "Req",
        "req": "DeviceInfo"
        "data": {
            "id": 111
        }
    }
```

- Ответ с данными от `Сервер`а `Клиент`у

```json
    {
        "cot": "RecCon",
        "data": {
            // Device Info data
        },
    }
```

- Ответ с ошибкой от `Сервер`а `Клиент`у

```json
    {
        "cot": "RecErr",
        "err": {
            "message": "Error message"
        }
    }
```

## Запрос `DeviceDoc`

- Запрос от `Клиент`а `Сервер`у

```json
    {
        "cot": "Req",
        "req": "DeviceDoc",
        "data": {
            "id": 111
        }
    }
```

- Ответ с данными от `Сервер`а `Клиент`у

```json
    {
        "cot": "RecCon",
        "data": {
            // Device Doc data
        },
    }
```

- Ответ с ошибкой от `Сервер`а `Клиент`у

```json
    {
        "cot": "RecErr",
        "err": {
            "message": "Error message"
        }
    }
```
