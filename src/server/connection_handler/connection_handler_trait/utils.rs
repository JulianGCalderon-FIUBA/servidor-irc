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

    fn send_message_to_channel(&mut self, message: &dyn Display, channel: &str) {
        let clients = self.database().get_clients_for_channel(channel);

        for client in clients {
            self.send_message_to_client(message, &client).ok();
        }
    }

    fn send_message_to_target(&mut self, message: &dyn Display, target: &str) -> io::Result<()> {
        if self.database().contains_client(target) {
            self.send_message_to_client(message, target)?
        } else {
            self.send_message_to_channel(message, target);
        }

        Ok(())
    }
}
