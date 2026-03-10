#[cfg(test)]
mod tests {
    use crate::{validate_udp_address, validate_tcp_address, parse_file_tickers};

    #[test]
    fn test_validate_udp_address_valid() {
        let result = validate_udp_address("udp://127.0.0.1:9999");
        assert!(result.is_ok());
        let (host, port) = result.unwrap();
        assert_eq!(host, "127.0.0.1");
        assert_eq!(port, 9999);
    }

    #[test]
    fn test_validate_udp_address_invalid_scheme() {
        let result = validate_udp_address("tcp://127.0.0.1:9999");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_udp_address_missing_port() {
        let result = validate_udp_address("udp://127.0.0.1");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_udp_address_invalid_format() {
        let result = validate_udp_address("not-a-url");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_tcp_address_valid() {
        let result = validate_tcp_address("127.0.0.1:12345");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_tcp_address_invalid() {
        let result = validate_tcp_address("invalid-address");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_file_tickers() {
        // Создаем временный файл для теста
        let temp_dir = std::env::temp_dir();
        let test_file = temp_dir.join("test_tickers.txt");

        std::fs::write(&test_file, "AAPL\nMSFT\nTSLA\n").unwrap();

        let result = parse_file_tickers(test_file.to_str().unwrap());
        assert!(result.is_ok());
        let tickers = result.unwrap();
        assert_eq!(tickers.len(), 3);
        assert_eq!(tickers[0], "AAPL");
        assert_eq!(tickers[1], "MSFT");
        assert_eq!(tickers[2], "TSLA");

        // Очистка
        let _ = std::fs::remove_file(&test_file);
    }

    #[test]
    fn test_parse_file_tickers_nonexistent() {
        let result = parse_file_tickers("/nonexistent/file.txt");
        assert!(result.is_err());
    }
}
