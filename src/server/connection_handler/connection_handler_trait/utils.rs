use std::fmt::Display;
use std::io;

use crate::message::Message;
use crate::server::connection::Connection;

use super::ConnectionHandlerGetters;

pub trait ConnectionHandlerUtils<C: Connection>: ConnectionHandlerGetters<C> {
    fn send_response(&mut self, response: &dyn Display) -> io::Result<()> {
        let response = Message::new(&response.to_string()).unwrap();
        response.send_to(self.connection())
    }

    fn send_message_to_client(&mut self, message: &dyn Display, nickname: &str) -> io::Result<()> {
        let message = Message::new(&message.to_string()).unwrap();
        let mut stream = self.database().get_stream(nickname)?;
        message.send_to(&mut stream)
    }
}
