use super::ParsingError;

use std::fmt;
use std::io;

#[derive(Debug)]
pub enum CreationError {
    IoError(io::Error),
    ParsingError(ParsingError),
}

impl fmt::Display for CreationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IoError(err) => write!(f, "IoError: {err}"),
            Self::ParsingError(err) => write!(f, "ParseError: {err}"),
        }
    }
}

impl From<io::Error> for CreationError {
    fn from(error: io::Error) -> Self {
        Self::IoError(error)
    }
}

impl From<ParsingError> for CreationError {
    fn from(error: ParsingError) -> Self {
        Self::ParsingError(error)
    }
}
