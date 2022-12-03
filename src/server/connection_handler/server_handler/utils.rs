use std::fmt::Display;

use crate::server::{connection::Connection, connection_handler::ConnectionHandlerUtils};

use super::ServerHandler;

impl<C: Connection> ConnectionHandlerUtils<C> for ServerHandler<C> {
    fn send_message_to_channel(&self, message: &dyn Display, channel: &str) {
        self.send_message_to_local_clients_on_channel(message, channel);

        let mut servers = self.get_channel_immediate_servers(channel);

        servers
            .iter()
            .position(|s| s == &self.servername)
            .map(|index| servers.remove(index));

        self.send_message_to_servers(servers, message);
    }
}

impl<C: Connection> ServerHandler<C> {
    pub fn send_message_to_all_other_servers(&self, message: &dyn Display) {
        let mut servers = self.database.get_all_servers();

        if let Some(index) = servers.iter().position(|x| x == &self.servername) {
            servers.remove(index);
        }

        self.send_message_to_servers(servers, message);
    }
}
