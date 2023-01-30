use std::{
    io::{self, Write},
    net::{TcpListener, TcpStream},
};

use crate::client::CRLF;

pub struct DccConnection {
    stream: TcpStream,
}

impl DccConnection {
    pub fn start_listening() -> io::Result<TcpListener> {
        TcpListener::bind("0.0.0.0:0")
    }

    pub fn accept(listener: TcpListener) -> io::Result<Self> {
        let stream = listener.accept()?.0;

        Ok(Self { stream })
    }

    pub fn connect(address: String) -> io::Result<Self> {
        let stream = TcpStream::connect(address)?;

        Ok(Self { stream })
    }

    /// Sends message to connected stream.
    pub fn send_raw(&mut self, message: &str) -> io::Result<()> {
        let bytes = message.as_bytes();

        self.stream.write_all(bytes)?;
        self.stream.write_all(CRLF)
    }

    pub fn read_message(&mut self) -> io::Result<String> {
        // READ MESSAGE FROM STREAM, ¿ASYNC?
        todo!()
    }
}
