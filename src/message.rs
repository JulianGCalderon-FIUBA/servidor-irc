use std::io::{self, BufRead, BufReader};
use std::io::{Read, Write};

pub struct Message {
    prefix: Option<String>,
    command: String,
    parameters: Vec<String>,
}

type MessageParse = (Option<String>, String, Vec<String>);

const CRLF: &[u8] = b"\r\n";

impl Message {
    pub fn new(content: String) -> Self {
        let parsing = parse(content);

        let (prefix, command, parameters) = parsing;

        Self {
            prefix,
            command,
            parameters,
        }
    }

    pub fn send_to(&self, stream: &mut dyn Write) -> io::Result<()> {
        let content = self.to_string();
        let bytes = content.as_bytes();

        stream.write_all(bytes)?;
        stream.write_all(CRLF)?;

        Ok(())
    }

    pub fn read_from(stream: &mut dyn Read) -> io::Result<Option<Self>> {
        let mut reader = BufReader::new(stream);

        let mut content = String::new();

        let size = reader.read_line(&mut content)?;
        if size == 0 {
            return Ok(None);
        }

        if content.as_bytes().ends_with(CRLF) {
            content.pop();
            content.pop();
        } else {
            return Ok(None);
        }

        Ok(Some(Self::new(content)))
    }
}

impl std::fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(prefix) = &self.prefix {
            write!(f, ":{} ", prefix)?;
        }

        write!(f, "{}", self.command)?;

        let mut parameters = self.parameters.iter();
        let last = parameters.next();
        let last = match last {
            None => "",
            Some(parameter) => parameter,
        };

        for parameter in parameters {
            write!(f, " {}", parameter)?;
        }

        write!(f, " :{}", last)
    }
}

fn parse(content: String) -> MessageParse {
    (None, content, Vec::new())
}
