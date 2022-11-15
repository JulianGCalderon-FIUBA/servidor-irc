use std::io::{BufReader, Write};
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

    pub fn start_async_read<F>(&mut self, on_message: F)
    where
        F: Fn(Result<Message, CreationError>) + Send + 'static,
    {
        let on_message = Box::new(on_message);

        let read_stream = match self.read_stream.take() {
            Some(read_stream) => read_stream,
            None => return,
        };

        let handle = thread::spawn(|| async_read(read_stream, on_message));
        self.read_thread = Some(handle);
    }

    pub fn async_print(&mut self) {
        self.start_async_read(print_message);
    }

    /// Sends message to Server.
    pub fn send_raw(&mut self, message: &str) -> io::Result<()> {
        let bytes = message.as_bytes();

        self.write_stream.write_all(bytes)?;
        self.write_stream.write_all(CRLF)
    }

    pub fn finished_asnyc_read(&self) -> bool {
        if let Some(join_handle) = &self.read_thread {
            return join_handle.is_finished();
        }

        true
    }
}

fn async_read<F>(read_stream: TcpStream, on_message: Box<F>)
where
    F: Fn(Result<Message, CreationError>) + Send + 'static,
{
    let mut reader = BufReader::new(read_stream);
    loop {
        let message = Message::read_from_buffer(&mut reader);
        if let Err(CreationError::IoError(_)) = message {
            return;
        }
        on_message(message);
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

fn print_message(message: Result<Message, CreationError>) {
    match message {
        Ok(message) => println!("{message}"),
        Err(error) => eprintln!("{error:?}"),
    }
}
