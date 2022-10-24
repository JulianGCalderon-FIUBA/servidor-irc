use std::io::Write;
use std::net::TcpStream;
use std::thread;
use std::{io, thread::JoinHandle};

use crate::message::{CreationError, Message};

/// Represents a client that can connect to a Server.
pub struct Client {
    write_stream: TcpStream,
    read_stream: Option<TcpStream>,
    read_thread: Option<JoinHandle<()>>,
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
            read_thread: None,
        })
    }

    pub fn async_read<F>(&mut self, on_message: F)
    where
        F: Fn(Result<Message, CreationError>) + Send + 'static,
    {
        let on_message = Box::new(on_message);

        let read_stream = self.read_stream.take();
        if let Some(mut read_stream) = read_stream {
            let handle = thread::spawn(move || loop {
                let message = Message::read_from(&mut read_stream);
                if let Err(CreationError::IoError(_)) = message {
                    on_message(message);
                    return;
                }
                on_message(message);
            });
            self.read_thread = Some(handle);
        }
    }

    /// Sends message to Server.
    pub fn send_raw(&mut self, message: &str) -> io::Result<()> {
        let bytes = message.as_bytes();

        self.write_stream.write_all(bytes)?;
        self.write_stream.write_all(CRLF)
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        if let Some(handler) = self.read_thread.take() {
            if let Err(error) = handler.join() {
                eprintln!("Error: {:?}", error);
            }
        }
    }
}
