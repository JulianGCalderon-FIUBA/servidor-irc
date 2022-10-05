use std::{io, net::TcpStream};

use crate::message::CreationError;
use crate::message::Message;

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

    pub fn read_message(&mut self) -> Result<Message, CreationError> {
        Message::read_from(&mut self.server)
    }
}
