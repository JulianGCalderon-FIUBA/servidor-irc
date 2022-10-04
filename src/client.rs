use std::{io, net::TcpStream};

use crate::message::Message;
use crate::message::MessageCreationError;

pub struct Client {
    // nickname: String,
    // hostname: String,
    // username: String,
    // tipo?
    // canales?
    server: TcpStream,
}

impl Client {
    pub fn new(address: String) -> io::Result<Self> {
        let server = TcpStream::connect(address)?;

        Ok(Self { server })
    }

    pub fn send_message(&mut self, message: Message) -> io::Result<()> {
        message.send_to(&mut self.server)
    }

    pub fn read_message(&mut self) -> Result<Message, MessageCreationError> {
        Message::read_from(&mut self.server)
    }
}
