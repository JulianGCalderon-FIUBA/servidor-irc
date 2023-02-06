use std::{
    io::BufReader,
    net::TcpStream,
    sync::mpsc::{self, SendError, Sender},
    thread::{self, JoinHandle},
};

use crate::message::{CreationError, Message};

type MessageReadingThreadHandle = JoinHandle<Result<(), SendError<Result<Message, CreationError>>>>;

/// Represents a client that can connect to a Server.
pub struct AsyncReader {
    thread: Option<MessageReadingThreadHandle>,
}

impl AsyncReader {
    /// Creates new [`Client`] connected to received address.
    pub fn new(stream: TcpStream, sender: Sender<Result<Message, CreationError>>) -> Self {
        let handle = thread::spawn(|| async_send(stream, sender));

        Self {
            thread: Some(handle),
        }
    }
}

fn async_send(
    stream: TcpStream,
    sender: Sender<Result<Message, CreationError>>,
) -> Result<(), mpsc::SendError<Result<Message, CreationError>>> {
    let mut reader = BufReader::new(stream);
    loop {
        let message = Message::read_from_buffer(&mut reader);
        if let Err(CreationError::IoError(_)) = message {
            return Ok(());
        }
        sender.send(message)?;
    }
}

impl Drop for AsyncReader {
    fn drop(&mut self) {
        if let Some(handler) = self.thread.take() {
            if let Err(error) = handler.join() {
                eprintln!("Error: {error:?}");
            }
        }
    }
}
