добавь
- активация DevStream, комманда без ответа
- DeviceInfo, запрос с ответом
- DeviceDoc, запрос с ответом
сюда
# Описание интерфейса между `Клиент`ом и `Сервер`ом

## Команда `DevStream`

```json
    {
        "cot": "Act",
        "name": "DeviceStream",
        "content": {
            "dev-id": 111
        }
    }
```

## DeviceInfo, запрос с ответом

## DeviceDoc, запрос с ответом



## Запрос `DeviceInfo`

- Запрос от `Клиент`а `Сервер`у

```json
    {
        "cot": "Req",
        "name": "DeviceInfo",
        "content": {
            "dev-id": 111
        }
    }
```

- Ответ с данными от `Сервер`а `Клиент`у

```json
    {
        "cot": "RecCon",
        "name": "DeviceInfo",
        "content": {
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
        "name": "DeviceDoc",
        "content": {
            "dev-id": 111
        }
    }
```

- Ответ с данными от `Сервер`а `Клиент`у

```json
    {
        "cot": "RecCon",
        "name": "DeviceDoc",
        "content": {
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
