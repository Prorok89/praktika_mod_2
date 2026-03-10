# quote_client

Клиент для получения котировок акций от quote_server.

## Описание

Клиент подключается к серверу по TCP, отправляет команду STREAM и получает котировки через UDP.

## Параметры запуска

| Параметр | Описание | По умолчанию |
|----------|----------|--------------|
| `--server-addr, -s` | Адрес сервера | требуется |
| `--udp-port, -u` | UDP порт для получения данных | 20000 |
| `--file-path, -f` | Путь к файлу с тикерами | требуется |

## Сборка

```bash
# Сборка в debug режиме
cargo build

# Сборка в release режиме
cargo build --release

# Запуск тестов
cargo test
```

## Примеры запуска

### Запуск в debug режиме с параметрами

```bash
# Windows
target\debug\quote_client.exe --server-addr 127.0.0.1:12345 --udp-port 10001 --file-path quote_client/data/test1.txt

# Linux/Mac
./target/debug/quote_client --server-addr 127.0.0.1:12345 --udp-port 10001 --file-path quote_client/data/test1.txt
```

### Запуск через cargo

```bash
cargo run -- --server-addr 127.0.0.1:12345 --udp-port 10001 --file-path quote_client/data/test1.txt
```

## Формат файла с тикерами

Каждый тикер на отдельной строке:
```
AAPL
MSFT
```

## Ожидается сервер

Сервер должен быть запущен и слушать указанный порт:

```bash
# Сначала запустите сервер
quote_server.exe --port 12345 --interval 5 --file-path quote_server/data/ticker.txt
```

## Ошибки

| Ошибка | Описание |
|--------|----------|
| Connection refused | Сервер не запущен |
| Address invalid | Неверный формат адреса |
| File not found | Файл с тикерами не найден |

## Логирование

```bash
RUST_LOG=info cargo run -- --server-addr 127.0.0.1:12345 --udp-port 10001 --file-path quote_client/data/test1.txt
```

## PING/PONG

Клиент автоматически отвечает на PING запросы от сервера для поддержания соединения.
