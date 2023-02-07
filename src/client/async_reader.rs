use std::{
    io,
    net::TcpStream,
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc, Arc,
    },
    thread::{self, JoinHandle},
};

use crate::message::{CreationError, Message};

/// Represents a client that can connect to a Server.
pub struct AsyncReader {
    handle: JoinHandle<MessageSendError>,
    running: Arc<AtomicBool>,
}

type MessageReceiver = mpsc::Receiver<Result<Message, CreationError>>;

impl AsyncReader {
    pub fn spawn(stream: TcpStream) -> (Self, MessageReceiver) {
        let (sender, receiver) = mpsc::channel();

        let running = Arc::new(AtomicBool::new(true));
        let running_ = Arc::clone(&running);
        let handle = thread::spawn(|| async_send(stream, sender, running_));

        let reader = Self { handle, running };

        (reader, receiver)
    }

    pub fn join(self) -> std::thread::Result<MessageSendError> {
        self.running.store(false, Ordering::Relaxed);
        self.handle.join()
    }

    pub fn running(&self) -> bool {
        !self.handle.is_finished()
    }
}

type MessageSendError = Result<(), mpsc::SendError<Result<Message, CreationError>>>;

fn async_send(
    mut stream: TcpStream,
    sender: mpsc::Sender<Result<Message, CreationError>>,
    running: Arc<AtomicBool>,
) -> MessageSendError {
    while running.as_ref().load(Ordering::Relaxed) {
        let message = Message::read_from(&mut stream);
        if let Err(CreationError::IoError(io_error)) = &message {
            if let io::ErrorKind::WouldBlock = io_error.kind() {
                continue;
            } else {
                sender.send(message)?;
                break;
            }
        }

        sender.send(message)?;
    }
    Ok(())
}
