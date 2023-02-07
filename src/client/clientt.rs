use std::{
    io::{self, Write},
    net::TcpStream,
};

use crate::message::CRLF;

/// Represents a client that can connect to a Server.
pub struct Clientt {
    pub stream: TcpStream,
}

impl Clientt {
    pub fn new(stream: TcpStream) -> Self {
        Self { stream }
    }

    /// Creates new [`Client`] connected to received address.
    pub fn connect(address: String) -> io::Result<Self> {
        let stream = TcpStream::connect(address)?;

        Ok(Self { stream })
    }

    /// Sends message to connected stream.
    pub fn send(&mut self, message: &str) -> io::Result<()> {
        let bytes = message.as_bytes();

        self.stream.write_all(bytes)?;
        self.stream.write_all(CRLF)
    }

    pub fn try_clone(&mut self) -> io::Result<Self> {
        Ok(Self {
            stream: self.stream.try_clone()?,
        })
    }

    pub fn get_stream(&self) -> io::Result<TcpStream> {
        self.stream.try_clone()
    }
}
