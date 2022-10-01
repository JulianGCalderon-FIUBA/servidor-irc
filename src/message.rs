use std::io::{self, BufRead, BufReader};
use std::io::{Read, Write};

pub struct Message {
    content: String,
}

const CRLF: &[u8] = b"\r\n";

impl Message {
    pub fn new(content: String) -> Self {
        Self { content }
    }

    pub fn send_to(&self, stream: &mut dyn Write) -> io::Result<()> {
        let bytes = self.content.as_bytes();

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

        trim_crlf(&mut content);

        Ok(Some(Self { content }))
    }
}

impl std::fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.content)
    }
}

fn trim_crlf(string: &mut String) {
    if string.as_bytes().ends_with(CRLF) {
        string.pop();
        string.pop();
    }
}
