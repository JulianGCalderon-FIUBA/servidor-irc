pub mod dcc_chat_receiver;
pub mod dcc_chat_sender;

use std::{
    io::{self, Write},
    net::{SocketAddr, TcpStream},
    sync::mpsc::{self, Receiver},
    thread,
};

use crate::message::{read_line, CRLF};

pub struct DccChat {
    pub stream: TcpStream,
    pub read_stream: Option<TcpStream>,
}

impl DccChat {
    pub fn new(stream: TcpStream) -> io::Result<Self> {
        Ok(Self {
            read_stream: Some(stream.try_clone()?),
            stream,
        })
    }

    pub fn connect(address: SocketAddr) -> io::Result<Self> {
        let stream = TcpStream::connect(address)?;

        Self::new(stream)
    }

    /// Sends message to connected stream.
    pub fn send_raw(&mut self, message: &str) -> io::Result<()> {
        let bytes = message.as_bytes();

        self.stream.write_all(bytes)?;
        self.stream.write_all(CRLF)
    }

    pub fn async_read_message(&mut self) -> Receiver<String> {
        let (sender, receiver) = mpsc::channel();

        let mut stream = self.read_stream.take().unwrap();
        thread::spawn(move || loop {
            while let Ok(message) = read_message(&mut stream) {
                sender.send(message).unwrap();
            }
        });

        receiver
    }
}

fn read_message(stream: &mut TcpStream) -> io::Result<String> {
    let mut content = String::new();

    read_line(stream, &mut content)?;

    if content.as_bytes().ends_with(CRLF) {
        content.pop();
        content.pop();
    } else {
        return error_no_trailing_crlf();
    }

    Ok(content)
}

fn error_no_trailing_crlf() -> Result<String, io::Error> {
    Err(io::Error::new(
        io::ErrorKind::InvalidInput,
        "Message should be trailed with CRLF",
    ))
}
