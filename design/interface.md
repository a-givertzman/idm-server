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
        "data": {
            "devId": 111
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
        "data": {
            "devId": 111
        }
    }
```

- Ответ с данными от `Сервер`а `Клиент`у

```json
    {
        "cot": "RecCon",
        "name": "DeviceInfo",
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
        "name": "DeviceDoc",
        "data": {
            "devId": 111
        }
    }
```

- Ответ с данными от `Сервер`а `Клиент`у

```json
    {
        "cot": "RecCon",
        "name": "DeviceDoc",
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
