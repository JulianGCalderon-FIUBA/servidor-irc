use std::fmt;

#[derive(Debug)]
pub enum ParsingError {
    NoTrailingCRLF,
    EmptyMessage,
    EmptyPrefix,
    NoCommand,
    // InvalidCharacter,
}

impl fmt::Display for ParsingError {
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
