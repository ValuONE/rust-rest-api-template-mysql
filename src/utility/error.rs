use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum ApiError {
    TodoNotFound,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ApiError::TodoNotFound => {
                write!(f, "No todo was found")
            }
        }
    }
}

impl Error for ApiError {}