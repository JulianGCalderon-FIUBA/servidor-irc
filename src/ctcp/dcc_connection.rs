use std::{
    io::{self, Write},
    net::TcpStream,
};

use crate::message::{read_line, CRLF};

pub struct DccConnection {
    stream: TcpStream,
}

impl DccConnection {
    pub fn new(stream: TcpStream) -> io::Result<Self> {
        stream.set_nonblocking(true)?;

        Ok(Self { stream })
    }

    pub fn connect(address: &str) -> io::Result<Self> {
        let stream = TcpStream::connect(address)?;

        Self::new(stream)
    }

    /// Sends message to connected stream.
    pub fn send_raw(&mut self, message: &str) -> io::Result<()> {
        let bytes = message.as_bytes();

        self.stream.write_all(bytes)?;
        self.stream.write_all(CRLF)
    }

    pub fn read_message(&mut self) -> io::Result<String> {
        let mut content = String::new();

        read_line(&mut self.stream, &mut content)?;

        if content.as_bytes().ends_with(CRLF) {
            content.pop();
            content.pop();
        } else {
            return error_no_trailing_crlf();
        }

        Ok(content)
    }
}

fn error_no_trailing_crlf() -> Result<String, io::Error> {
    Err(io::Error::new(
        io::ErrorKind::InvalidInput,
        "Message should be trailed with CRLF",
    ))
}
