# Структуры



## Структура приложения

```mermaid
flowchart LR
    Idm-Client1["Idm-Client 1"] <-- "TCP Socket Messages" ----> Idm-Server
    Idm-Client_["Idm-Client .."] <-- "TCP Socket Messages" ----> Idm-Server
    Idm-Clientn["Idm-Client n"] <-- "TCP Socket Messages" ----> Idm-Server
```

## Диаграмма классов

### Сообщения

- `Request` - Входящее сообщение, содержит причину передачи для перенаправления вложенного запроса соответствующему обработчику
- `Cot` - Причина передачи, служит классифицирует сообщения по направлении передачи и типу запроса

    Cot     | Направление      | Тип запроса                  | Ответ
    --------|------------------| ---                          | ---
    Inf     | Client -> Server | Информационное сообщение     | -
    Act     | Client -> Server | Комманда                     | Опионально
    ActCon  | Client -> Server | Положительный ответ          | -
    ActErr  | Client -> Server | Отрицательный ответ / ошибка | -
    Req     | Client -> Server | Запрос                       | Обязательно
    ReqCon  | Client -> Server | Ответ                        | -
    ReqErr  | Client -> Server | Ошибка                       | -


    **Диаграмма `Request` и `Cot`**

    ```mermaid
    classDiagram
        class Request {
            +Cot
            +Content
        }
        class Cot {
            +Inf
            +Act    // Request.Cmd
            +ActCon // Optional confirmation
            +ActErr
            +Req    // Request.Req
            +ReqCon // Required reply
            +ReqErr
        } 
    ```

- `Request.Cmd` - Если входящее сообщение, содержит причину Cot.Act, то в теле такого сообщения содержится команда
- `Request.Req` - Если входящее сообщение, содержит причину Cot.Req, то в теле такого сообщения содержится запрос

```mermaid
classDiagram
    class Request.Cmd {
        Kind
        content
    }
    class Request.Req {
        Kind
        content
    }
```

### Обработка сообщений в бэкенде приложениия

```mermaid
classDiagram
    Server *--"*" Connection : TcpStream
    Server: +new(Conf, Schedule) Result
    Server: +run() Result
    Server: +exit()

    class Connection {
        +new(Conf, Schedule) Result
        +run() Result
        +exit()
    }
    Connection *-- SelectCot : Request

    class SelectCot {
        +eval() Reply
    }
    SelectCot *-- SelectAct : Cot-Act [ Cmd ]
    SelectCot *-- SelectReq : Cot-Req [ Req ]
    SelectCot o.. Request.Cot : selects by
    
    class SelectAct {
        +eval() Reply
    }
    SelectAct *-- SelectDevStream : Cmd.content
    SelectAct o.. Cmd.Kind : selects by

    class SelectReq {
        +eval() Reply
    }
    SelectReq *-- SelectDevInfo : Req.content
    SelectReq *-- SelectDevDoc : Req.content
    SelectReq o.. Req.Kind : selects by

```

## Конфигурация приложения

Конфигурация в приложении представлена структурой `Conf`, загружается из `yaml` файла.

**Пример конфигурации `config.yaml`:**

```yaml 
server:
    # Server socket address "IP:Port"
    address: "127.0.0.1:8080"
    # Configuration to Connection spawned by the server
    connection:
        # timeout for waiting socket, ms
        timeout: 100
```

