
### Маршрутизация запросов в Connection, 

# IDM-Server - Описание функций
## Ключевые компоненты
### TCP-сервер (Server)
**Функционал**
- Открытие сокета на указанном порту из заданной конфигурации
- Прослушивание входящих подключений
- Обработка команд от клиента
- Создание для каждого подключения соединения `Connection`

**Жизненный цикл**
1. Создание объекта сервера
2. Запуск сервера:
    1. Открытие сетевого порта
    2. Ожидание подключнения клиента
    3. Создание `Connection` для подключённого клиента
    4. Передача в `Connection` настройки обработки:
        - команд
        - запросов
    5. Сохранение соединения в список
3. Ожидание завершения всех соединений
4. Остановка сервера


### Connection
**Функционал**
- Парсинг данные в структурированное сообщение `TcpMessage`
- Обработка сообщений
- Формирование и отправка ответа в формате JSON
- Управление временем жизни потока: завершение, выход, таймаут

**Жизненный цикл**
1. Создание соединения
2. Запуск соединения:
    1. Запуск двух потоков:
        - чтение
        - запись
    2. Чтение из сокета:
        1. Считывание данных
        2. Парсинг байтов в структурированное сообщение `TcpMessage`
        3. Сериализация результата в JSON
        4. Отправка результата
    3. Запись в сокет:
        1. Формерование ответа
        2. Отправка ответа в поток
3. Закрытие соединения, завершение потоков

## Функции
- конфигурация:
    - **Conf** - конфигурация сервера
- информация:
    - **DeviceInfo** - 
- domain:
    - **Eval** - 
    - **Sync** -
        - **Hub** - 
        - **LinkSend** - 
        - **Link** - 
- сервер:
    - **SelectAct** -
        - **Command** -
        - **DevConf** -
        - **DevStreamConf** -
        - **DevStream** -
    - **SelectReq** -
        - **Context** -
        - **Request** -
        - **SelectDevDoc** -
        - **SelectDevInfo** -
    - **ConnectionConf** -
    - **Connection** -
    - **Cot** -
    - **SelectCot** -
    - **ServerConf** -

## Поведенческая диаграмма

```mermaid
sequenceDiagram 
    User->>Server: TCP подключение
    Server->>Connection: Создание соединения
    
    loop Обработка сообщений
        User->>Connection: JSON сообщение
        Connection->>SelectCot: Парсинг сообщения
        
				alt Request
					SelectCot->>SelectReq: Обработка запроса 
					SelectReq->>SelectReq: Парсинг поля "req"
					alt DeviceInfo 
						SelectReq->>SelectDevInfo: Обработка DeviceInfo 
						SelectDevInfo-->>SelectReq: JSON результат 
					else DeviceDoc 
						SelectReq->>SelectDevDoc: Обработка DeviceDoc
						SelectDevDoc-->>SelectReq: JSON результат	
					end
					SelectReq-->>SelectCot: JSON результат
					
				else Activation
					SelectCot->>SelectAct: Обработка активации 
					SelectAct->>SelectAct: Парсинг поля "act"
					alt DeviceStream 
						SelectAct->>SelectDevStream: Обработка DeviceStream 
						SelectDevStream->>SelectDevStream: Извлечение данных из "data" 
						SelectDevStream-->>SelectAct: JSON результат
					end
					SelectAct-->>SelectCot: JSON результат
					
				else Другие типы Cot 
					SelectCot->>SelectCot: Error
				end
				
        SelectCot-->>Connection: Результат
        Connection-->>User: JSON ответ
    end
    
    User->>Connection: Отключение
    Connection-->>Server: Закрытие соединения
```

## Диаграмма классов

```mermaid
classDiagram
	class Server {
		+address: String
		+connections: List~Connection~
		+run() Запустить сервер
		+exit() Остановить сервер
	}
	class Connection {
		+stream: TcpStream
		+ctx: SelectCot
		+run()
		+exit()
	}
	class SelectCot {
		-select: IndexMap~Cot, Box~
		+eval()
		+new()
	}
	class SelectReq {
		-select: IndexMap~Request, Box~
		+eval()
		+new()
	}
	class SelectAct {
		-select: IndexMap~Command, Box~
		+eval()
		+new()
	}
	class SelectDevInfo {
		-ctx: Box~Eval~
		+eval()
		+new()
	}
	class SelectDevDoc {
		+eval()
		+new()
	}
	class SelectDevStream {
		+eval()
		+new()
	}
	
	Server --> Connection : creates
	Connection --> SelectCot : uses
	SelectCot --> SelectReq : delegates to
	SelectCot --> SelectAct : delegates to
	SelectReq --> SelectDevInfo: contains
	SelectReq --> SelectDevDoc: contains
	SelectAct --> SelectDevStream: contains
```

references

- [Requirenments](https://blog.bit.ai/software-requirements-document/)
- https://blog.bit.ai/software-design-document/
- https://www.geeksforgeeks.org/design-documentation-in-software-engineering/

