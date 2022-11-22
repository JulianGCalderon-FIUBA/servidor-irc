use std::{
    fmt::Display,
    io::{self, Write},
    net::TcpStream,
};

use crate::message::Message;

pub trait ConnectionResponses: Write + Sized {
    fn send(&mut self, message: &dyn Display) -> io::Result<()> {
        if let Ok(message) = Message::new(&message.to_string()) {
            return message.send_to(self);
        }

        Ok(())
    }
}

impl ConnectionResponses for TcpStream {}
