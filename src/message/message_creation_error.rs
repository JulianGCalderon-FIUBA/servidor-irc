use std::fmt;
use std::io;

#[derive(Debug)]
pub enum MessageCreationError {
    IoError(io::Error),
    ParseError(ParseError),
}

#[derive(Debug)]
pub enum ParseError {
    NoTrailingCRLF,
    EmptyMessage,
    EmptyPrefix,
    NoCommand,
    // InvalidCharacter,
}

impl fmt::Display for MessageCreationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IoError(err) => write!(f, "IoError: {}", err),
            Self::ParseError(err) => write!(f, "ParseError: {}", err),
        }
    }
}

impl From<io::Error> for MessageCreationError {
    fn from(error: io::Error) -> Self {
        Self::IoError(error)
    }
}

impl From<ParseError> for MessageCreationError {
    fn from(error: ParseError) -> Self {
        Self::ParseError(error)
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoTrailingCRLF => write!(f, "message should have trailing CRLF"),
            Self::EmptyMessage => write!(f, "message should not be empty"),
            Self::EmptyPrefix => write!(f, "prefix should not be empty"),
            Self::NoCommand => write!(f, "message should have a command"),
            // Self::InvalidCharacter => write!(f, "encountered ilegal character"),
        }
    }
}
