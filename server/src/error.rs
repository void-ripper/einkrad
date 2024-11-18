use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct ServerError {
    msg: String,
}

impl Error for ServerError {}

impl Display for ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<std::io::Error> for ServerError {
    fn from(value: std::io::Error) -> Self {
        ServerError {
            msg: value.to_string(),
        }
    }
}

impl From<bincode::Error> for ServerError {
    fn from(value: bincode::Error) -> Self {
        ServerError {
            msg: value.to_string(),
        }
    }
}
