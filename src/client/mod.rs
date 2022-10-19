use std::io::{self, BufReader};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::thread;

use crate::message::{CreationError, Message};

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

    pub fn async_read<F>(&mut self, on_message: F)
    where
        F: Fn(Message) + Send + 'static,
    {
        let on_message = Box::new(on_message);

        let read_stream = self.read_stream.take();
        if let Some(mut read_stream) = read_stream {
            thread::spawn(move || {
                let mut reader: BufReader<&mut dyn Read> = BufReader::new(&mut read_stream);
                loop {
                    let message = Message::read_from_buffer(&mut reader);
                    if let Ok(message) = message {
                        on_message(message);
                    } else if let Err(CreationError::IoError(_)) = message {
                        return;
                    }
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
