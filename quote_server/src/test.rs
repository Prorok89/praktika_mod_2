#[cfg(test)]
mod tests {
    use std::sync::{Arc, RwLock};
    use clap::Parser;
    use crate::{Cli, Client, ServerError};

    #[test]
    fn test_cli_default_values() {
        let cli = Cli::try_parse_from(["test", "--port", "8080", "--file-path", "test.txt"]).unwrap();
        assert_eq!(cli.port, 8080);
        assert_eq!(cli.file_path, "test.txt");
        assert_eq!(cli.interval, 1);
    }

    #[test]
    fn test_cli_with_interval() {
        let cli = Cli::try_parse_from(["test", "--port", "8080", "--interval", "5", "--file-path", "test.txt"]).unwrap();
        assert_eq!(cli.port, 8080);
        assert_eq!(cli.interval, 5);
    }

    #[test]
    fn test_client_new() {
        let client = Client::new();
        assert_eq!(client.adress, "127.0.0.1");
        assert_eq!(client.port, 9999);
        assert!(client.ticker.is_empty());
        assert!(!client.alive);
    }

    #[test]
    fn test_client_clone() {
        let mut client = Client::new();
        client.ticker.push("AAPL".to_string());
        client.alive = true;

        let client_clone = client.clone();
        assert_eq!(client_clone.ticker, vec!["AAPL"]);
        assert!(client_clone.alive);
    }

    #[test]
    fn test_error_display() {
        let err = ServerError::ConnectClosed;
        assert_eq!(format!("{}", err), "connection closed");

        let err = ServerError::SendServer { value: "test error".to_string() };
        assert_eq!(format!("{}", err), "test error");

        let err = ServerError::TickerNotFound("AAPL".to_string());
        assert_eq!(format!("{}", err), "Ticker not found: AAPL");
    }

    #[test]
    fn test_tickers_validation() {
        let tickers = vec!["AAPL".to_string(), "MSFT".to_string()];
        let request_tickers = vec!["AAPL".to_string()];

        let all_valid = request_tickers.iter().all(|t| tickers.contains(t));
        assert!(all_valid);

        let invalid_request = vec!["UNKNOWN".to_string()];
        let has_invalid = invalid_request.iter().any(|t| !tickers.contains(t));
        assert!(has_invalid);
    }
}
