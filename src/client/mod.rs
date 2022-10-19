use std::{io, net::TcpStream};

use crate::message::CreationError;
use crate::message::Message;

/// Represents a client that can connect to a Server.
pub struct Client {
    server: TcpStream,
}

impl Client {
    /// Creates new client connected to received address.
    pub fn new(address: String) -> io::Result<Self> {
        let server = TcpStream::connect(address)?;

        Ok(Self { server })
    }

    /// Sends message to Server.
    pub fn send_message(&mut self, message: Message) -> io::Result<()> {
        message.send_to(&mut self.server)
    }

    /// Reads message received from Server.
    pub fn read_message(&mut self) -> Result<Message, CreationError> {
        Message::read_from(&mut self.server)
    }
}
