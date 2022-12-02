use std::fmt::Display;

use crate::{
    macros::ok_or_return,
    server::{
        connection::Connection,
        connection_handler::connection_handler_trait::ConnectionHandlerUtils,
    },
};

use super::ServerHandler;

impl<C: Connection> ConnectionHandlerUtils<C> for ServerHandler<C> {
    fn send_message_to_channel(&mut self, message: &dyn Display, channel: &str) {
        let clients = ok_or_return!(self.database.get_channel_clients(channel));

        let mut servers = vec![];

        for client in clients {
            if self.database.is_local_client(&client) {
                self.send_message_to_client(message, &client).ok();
            } else if let Ok(server) = self.database.get_immediate_server(&client) {
                if !servers.contains(&server) && server != self.servername {
                    servers.push(server);
                }
            }
        }

        for server in servers {
            self.send_message_to_server(message, &server).ok();
        }
    }
}

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
