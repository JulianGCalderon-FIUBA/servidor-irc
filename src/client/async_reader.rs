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
    stream: Option<TcpStream>,
    handle: Option<JoinHandle<SendThreadReturn>>,
    running: Arc<AtomicBool>,
}

impl AsyncReader {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            stream: Some(stream),
            running: Arc::new(AtomicBool::new(false)),
            handle: None,
        }
    }

    /// Creates new [`Client`] connected to received address.
    pub fn spawn(&mut self) -> mpsc::Receiver<Result<Message, CreationError>> {
        let stream = self.stream.take().expect("reader is already running");

        let (sender, receiver) = mpsc::channel();

        let running = Arc::clone(&self.running);
        thread::spawn(|| async_send(stream, sender, running));

        receiver
    }

    pub fn join(&mut self) -> std::thread::Result<SendThreadReturn> {
        self.running.store(false, Ordering::Relaxed);

        let handle = self.handle.take().expect("should be running");
        handle.join()
    }

    pub fn running(&self) -> bool {
        self.running.load(Ordering::Relaxed)
    }
}

impl Drop for AsyncReader {
    fn drop(&mut self) {
        if self.running() {
            self.join().ok();
        }
    }
}

type SendThreadReturn = Result<(), mpsc::SendError<Result<Message, CreationError>>>;

fn async_send(
    mut stream: TcpStream,
    sender: mpsc::Sender<Result<Message, CreationError>>,
    running: Arc<AtomicBool>,
) -> SendThreadReturn {
    while running.as_ref().load(Ordering::Relaxed) {
        let message = Message::read_from(&mut stream);
        if let Err(CreationError::IoError(io_error)) = &message {
            if let io::ErrorKind::WouldBlock = io_error.kind() {
                continue;
            } else {
                sender.send(message)?;
                return Ok(());
            }
        }

        sender.send(message)?;
    }

    Ok(())
}
