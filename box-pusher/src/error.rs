use std::error;
use std::fmt;
use std::io;
use std::result;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    InvalidJson(String),
    Io(String),
    Duku(String),
    InvalidLevel(String),
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Self::Io(format!("{}", e))
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Self::InvalidJson(format!("{}", e))
    }
}

impl From<duku::Error> for Error {
    fn from(e: duku::Error) -> Self {
        Self::Duku(format!("{}", e))
    }
}

impl From<&str> for Error {
    fn from(e: &str) -> Self {
        Self::InvalidLevel(e.to_string())
    }
}
