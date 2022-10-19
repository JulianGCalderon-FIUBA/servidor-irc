use std::io::{self, Write};
use std::net::TcpStream;
use std::thread;

use crate::message::Message;

/// Represents a client that can connect to a Server.
pub struct Client {
    write_stream: TcpStream,
    read_stream: Option<TcpStream>,
}

const CRLF: &[u8; 2] = b"\r\n";

impl Client {
    /// Creates new client connected to received address.
    pub fn new(address: String) -> io::Result<Self> {
        let write_stream = TcpStream::connect(address)?;
        let read_stream = Some(write_stream.try_clone()?);

        Ok(Self {
            write_stream,
            read_stream,
        })
    }

    pub fn async_read(&mut self) {
        let read_stream = self.read_stream.take();
        if let Some(mut read_stream) = read_stream {
            thread::spawn(move || loop {
                let message = Message::read_from(&mut read_stream);
                match message {
                    Ok(message) => println!("{}", message),
                    Err(error) => eprintln!("Error reading message: {}", error),
                }
            });
        }
    }

    /// Sends message to Server.
    pub fn send_raw(&mut self, message: &str) -> io::Result<()> {
        let bytes = message.as_bytes();

        self.write_stream.write_all(bytes)?;
        self.write_stream.write_all(CRLF)
    }
}
