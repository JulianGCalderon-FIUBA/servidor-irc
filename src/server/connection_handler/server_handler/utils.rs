use std::fmt::Display;

use crate::server::{
    connection::Connection, connection_handler::connection_handler_trait::ConnectionHandlerUtils,
};

use super::ServerHandler;

impl<C: Connection> ConnectionHandlerUtils<C> for ServerHandler<C> {}

impl<C: Connection> ServerHandler<C> {
    pub fn send_message_to_all_other_servers(&mut self, message: &dyn Display) {
        let mut servers = self.database.get_all_servers();

        if let Some(index) = servers.iter().position(|x| x == &self.servername) {
            servers.remove(index);
        }

        for server in servers {
            self.send_message_to_server(message, &server).ok();
        }
    }
}
