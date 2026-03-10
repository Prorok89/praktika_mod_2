# quote_server

Сервер генерации и рассылки котировок акций.

## Описание

Сервер принимает TCP подключения от клиентов, обрабатывает команду STREAM и отправляет котировки через UDP.

## Параметры запуска

| Параметр | Описание | По умолчанию |
|----------|----------|--------------|
| `--port, -p` | Порт для TCP соединений | 10000 |
| `--file-path, -f` | Путь к файлу с тикерами | требуется |
| `--interval, -i` | Интервал генерации данных (сек) | 1 |

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
target\debug\quote_server.exe --port 12345 --interval 5 --file-path quote_server/data/ticker.txt

# Linux/Mac
./target/debug/quote_server --port 12345 --interval 5 --file-path quote_server/data/ticker.txt
```

### Запуск через cargo

```bash
cargo run -- --port 12345 --interval 5 --file-path quote_server/data/ticker.txt
```

## Формат файла с тикерами

Каждый тикер на отдельной строке:
```
AAPL
MSFT
TSLA
```

## Пример команды STREAM

```
STREAM udp://127.0.0.1:9999 AAPL,MSFT
```

## Ошибки

| Ошибка | Описание |
|--------|----------|
| Address in use | Порт уже занят |
| Ticker not found | Тикер отсутствует в файле |
| Invalid command | Неправильный формат команды |

## Логирование

```bash
RUST_LOG=info cargo run -- --port 12345 --interval 5 --file-path quote_server/data/ticker.txt
```