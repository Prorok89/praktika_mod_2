use std::fmt::Display;

pub enum ServerError {
    CommandFormat
}

impl Display for ServerError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServerError::CommandFormat => write!(f, "format line")
        }
    }
}