use std::{fmt::Display, io};

#[derive(Debug)]
pub enum ClientError {
    SendServer {value: String},
    CommandFormat,
    ConnectClosed,
    ErrorNotKnow,
    IoError(io::Error),
    TickerNotFound(String),
}

impl Display for ClientError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClientError::CommandFormat => write!(f, "format line"),
            ClientError::ConnectClosed => write!(f, "connection closed"),
            ClientError::ErrorNotKnow => write!(f, "error not know"),
            ClientError::SendServer { value } => write!(f, "{}",value),
            ClientError::IoError(e) => write!(f, "error IO : {}", e),
            ClientError::TickerNotFound(ticker) => write!(f, "Ticker not found: {}", ticker),
        }
    }
}