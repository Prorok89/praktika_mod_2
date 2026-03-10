use std::{fmt::Display, io};

#[derive(Debug)]
pub enum ClientError {
    SendServer {value: String},
    IoError(io::Error),
}

impl Display for ClientError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClientError::SendServer { value } => write!(f, "{}",value),
            ClientError::IoError(e) => write!(f, "error IO : {}", e),
        }
    }
}