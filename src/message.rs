use std::io::{self, BufRead, BufReader, Error, ErrorKind};
use std::io::{Read, Write};

pub struct Message {
    prefix: Option<String>,
    command: String,
    parameters: Vec<String>,
}

type MessageParse = (Option<String>, String, Vec<String>);

const CRLF: &[u8] = b"\r\n";

impl Message {
    pub fn new(content: String) -> io::Result<Self> {
        let (prefix, command, parameters) = parse(content)?;

        Ok(Self {
            prefix,
            command,
            parameters,
        })
    }

    pub fn send_to(&self, stream: &mut dyn Write) -> io::Result<()> {
        let content = self.to_string();
        let bytes = content.as_bytes();

        stream.write_all(bytes)?;
        stream.write_all(CRLF)?;

        Ok(())
    }

    pub fn read_from(stream: &mut dyn Read) -> io::Result<Self> {
        let mut reader = BufReader::new(stream);

        let mut content = String::new();

        let size = reader.read_line(&mut content)?;
        if size == 0 {
            return Err(empty_message_error());
        }

        if content.as_bytes().ends_with(CRLF) {
            content.pop();
            content.pop();
        } else {
            return Err(no_trailing_crlf_in_message_error());
        }

        Self::new(content)
    }
}

impl std::fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(prefix) = &self.prefix {
            write!(f, ":{} ", prefix)?;
        }

        write!(f, "{}", self.command)?;

        Ok(())
    }
}

fn parse(content: String) -> io::Result<MessageParse> {
    Ok((None, content, Vec::new()))
}

fn empty_message_error() -> Error {
    Error::new(ErrorKind::InvalidInput, "message should not be empty")
}

fn no_trailing_crlf_in_message_error() -> Error {
    Error::new(ErrorKind::InvalidInput, "message should end in CRLF")
}
