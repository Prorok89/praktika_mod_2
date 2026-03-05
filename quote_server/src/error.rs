use std::{fmt::Display, io};

#[derive(Debug)]
pub enum ServerError {
    SendServer {value: String},
    CommandFormat,
    ConnectClosed,
    ErrorNotKnow,
    IoError(io::Error),
    TickerNotFound(String),
}

impl Display for ServerError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServerError::CommandFormat => write!(f, "format line"),
            ServerError::ConnectClosed => write!(f, "connection closed"),
            ServerError::ErrorNotKnow => write!(f, "error not know"),
            ServerError::SendServer { value } => write!(f, "{}",value),
            ServerError::IoError(e) => write!(f, "error IO : {}", e),
            ServerError::TickerNotFound(ticker) => write!(f, "Ticker not found: {}", ticker),
        }
    }
}