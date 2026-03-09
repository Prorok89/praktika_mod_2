use std::fmt::Display;

pub enum CommonError {
    CommonError(String)
}

impl Display for CommonError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CommonError::CommonError(value) => write!(f, "{}", value)
        }
    }
}