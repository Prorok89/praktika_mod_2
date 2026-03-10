# common

Общий crate с утилитами и типами данных для quote_server и quote_client.

## Описание

Этот crate содержит:
- Модели данных (StockQuote)
- Функции валидации адресов (TCP/UDP)
- Парсинг файлов с тикерами
- Обработку ошибок

## Структура проекта

```
common/
├── src/
│   ├── lib.rs       # Основной модуль с функциями
│   ├── error.rs     # Типы ошибок
│   ├── model.rs     # Модели данных
│   └── test.rs      # Тесты
└── README.md
```

## Сборка

```bash
# Сборка в debug режиме
cargo build

# Сборка в release режиме
cargo build --release

# Запуск тестов
cargo test
```

## Зависимости

- `url` - Парсинг URL адресов
- `serde` - Сериализация/десериализация
- `chrono` - Работа с временными метками

## Примеры использования

### Валидация UDP адреса

```rust
use common::validate_udp_address;

let result = validate_udp_address("udp://127.0.0.1:9999");
match result {
    Ok((host, port)) => println!("Host: {}, Port: {}", host, port),
    Err(e) => println!("Error: {}", e),
}
```

### Валидация TCP адреса

```rust
use common::validate_tcp_address;

let result = validate_tcp_address("127.0.0.1:8080");
match result {
    Ok(address) => println!("Address: {}", address),
    Err(e) => println!("Error: {}", e),
}
```

### Парсинг тикеров из файла

```rust
use common::parse_file_tickers;

let tickers = parse_file_tickers("tickers.txt")?;
for ticker in tickers {
    println!("{}", ticker);
}
```

## Файлы данных

- `data/ticker.txt` - Файл с тикерами (по одной строке)
