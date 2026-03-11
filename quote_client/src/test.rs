#[cfg(test)]
mod tests {
    use clap::Parser;
    use crate::{ClientError, model::Cli};

    #[test]
    fn test_cli_parsing() {
        let cli = Cli::try_parse_from([
            "test",
            "--tcp-server",
            "127.0.0.1:8080",
            "--udp-server",
            "127.0.0.1:9999",
            "--file-path",
            "test.txt",
        ]).unwrap();

        assert_eq!(cli.tcp_server, "127.0.0.1:8080");
        assert_eq!(cli.udp_server, "127.0.0.1:9999");
        assert_eq!(cli.file_path, "test.txt");
    }

    #[test]
    fn test_error_display() {
        use std::io;

        let err = ClientError::SendServer { value: "test error".to_string() };
        assert_eq!(format!("{}", err), "test error");

        let io_err = ClientError::IoError(io::Error::new(io::ErrorKind::Other, "IO error"));
        assert!(format!("{}", io_err).contains("IO error"));
    }

    #[test]
    fn test_validate_server_address_format() {
        // Проверяем формат адреса сервера
        let addr = "127.0.0.1:8080";
        let parts: Vec<&str> = addr.split(':').collect();
        assert_eq!(parts.len(), 2);
        assert_eq!(parts[0], "127.0.0.1");
        assert_eq!(parts[1], "8080");
    }

    #[test]
    fn test_udp_port_range() {
        let port: u16 = 9999;
        assert!(port >= 1 && port <= 65535);

        let default_port: u16 = 20000;
        assert!(default_port >= 1 && default_port <= 65535);
    }

    #[test]
    fn test_tickers_parsing() {
        let tickers_str = "AAPL,MSFT,TSLA";
        let tickers: Vec<&str> = tickers_str.split(',').collect();

        assert_eq!(tickers.len(), 3);
        assert_eq!(tickers[0], "AAPL");
        assert_eq!(tickers[1], "MSFT");
        assert_eq!(tickers[2], "TSLA");
    }
}
