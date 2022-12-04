mod creation_error;
mod parsing;
mod parsing_error;

#[cfg(test)]
mod tests;

pub use creation_error::CreationError;
pub use parsing_error::ParsingError;

use std::io::{self, BufRead, BufReader, Error, ErrorKind, Read, Write};
pub struct Message {
    prefix: Option<String>,
    command: String,
    parameters: Vec<String>,
    trailing: Option<String>,
}

const CRLF: &[u8] = b"\r\n";
const LF: &[u8] = b"\n";
const PREFIX_CHARACTER: u8 = b':';
const MAX_LENGTH: usize = 510;
const INVALID_CHARACTERS: [char; 3] = ['\r', '\n', '\0'];

impl Message {
    /// Creates new [`Message`] from a string
    /// Fails if message could not be parsed
    pub fn new(content: &str) -> Result<Self, ParsingError> {
        let (prefix, command, parameters, trailing) = parsing::parse(content)?;

        Ok(Self {
            prefix,
            command,
            parameters,
            trailing,
        })
    }

    /// Sends message to a stream, trailed with CRLF
    pub fn send_to(&self, stream: &mut dyn Write) -> io::Result<()> {
        let string = self.to_string();
        let bytes = string.as_bytes();

        stream.write_all(bytes)?;
        stream.write_all(CRLF)?;

        Ok(())
    }

    /// Reads message from a stream
    /// Fails on an IO error or a Parsing error.
    pub fn read_from(stream: &mut dyn Read) -> Result<Self, CreationError> {
        let mut content = String::new();

        Self::read_line(stream, &mut content)?;

        if content.as_bytes().ends_with(CRLF) {
            content.pop();
            content.pop();
        } else {
            return Err(CreationError::ParsingError(ParsingError::NoTrailingCRLF));
        }

        let message = Self::new(&content)?;

        Ok(message)
    }

    /// Reads message from a BufReader
    /// This performs better than `read_from` as a buffer performs large,
    ///  infrequent reads on the underlying Read
    ///  and maintains an in-memory buffer of the results.
    pub fn read_from_buffer<R: Read>(buffer: &mut BufReader<R>) -> Result<Self, CreationError> {
        let mut content = String::new();
        let read = buffer.read_line(&mut content)?;
        if read == 0 {
            Err(unexpected_eof_error())?;
        }

        if content.as_bytes().ends_with(CRLF) {
            content.pop();
            content.pop();
        } else {
            return Err(CreationError::ParsingError(ParsingError::NoTrailingCRLF));
        }

        let message = Self::new(&content)?;

        Ok(message)
    }

    /// Reads single line from a stream, calling a read for every byte read
    fn read_line(stream: &mut dyn Read, buffer: &mut String) -> io::Result<()> {
        let mut content = String::new();
        while !content.as_bytes().ends_with(LF) {
            let mut buffer = [0; 1];
            stream.read_exact(&mut buffer)?;
            content.push(buffer[0] as char)
        }

        buffer.push_str(&content);

        Ok(())
    }

    pub fn unpack(self) -> (Option<String>, String, Vec<String>, Option<String>) {
        (self.prefix, self.command, self.parameters, self.trailing)
    }

    pub fn get_prefix(&self) -> &Option<String> {
        &self.prefix
    }

    pub fn get_command(&self) -> &String {
        &self.command
    }

    pub fn get_parameters(&self) -> &Vec<String> {
        &self.parameters
    }

    pub fn get_trailing(&self) -> &Option<String> {
        &self.trailing
    }
}

fn unexpected_eof_error() -> Error {
    Error::new(ErrorKind::UnexpectedEof, "")
}

impl std::fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(prefix) = &self.prefix {
            write!(f, ":{} ", prefix)?;
        }

        write!(f, "{}", self.command)?;

        for parameter in self.parameters.iter() {
            write!(f, " {}", parameter)?;
        }

        if let Some(trailing) = &self.trailing {
            write!(f, " :{}", trailing)
        } else {
            Ok(())
        }
    }
}
