use std::{fmt::Display, io};

#[derive(Debug)]
pub enum ServerError {
    SendServer {value: String},
    ConnectClosed,
    IoError(io::Error),
    TickerNotFound(String),
}

impl Display for ServerError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServerError::ConnectClosed => write!(f, "connection closed"),
            ServerError::SendServer { value } => write!(f, "{}",value),
            ServerError::IoError(e) => write!(f, "error IO : {}", e),
            ServerError::TickerNotFound(ticker) => write!(f, "Ticker not found: {}", ticker),
        }
    }
}