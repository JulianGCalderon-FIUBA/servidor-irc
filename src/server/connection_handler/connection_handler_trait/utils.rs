use std::fmt::Display;
use std::io;

use crate::server::connection::Connection;

use super::ConnectionHandlerGetters;

pub trait ConnectionHandlerUtils<C: Connection>: ConnectionHandlerGetters<C> {
    fn send_message_to_client(&mut self, message: &dyn Display, nickname: &str) -> io::Result<()> {
        if let Some(stream) = self.database().get_stream(nickname) {
            stream?.send(&message)?;
        }
        // cliente desconectado
        Ok(())
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
