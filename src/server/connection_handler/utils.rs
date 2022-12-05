use std::fmt::Display;
use std::io;

use crate::macros::ok_or_return;
use crate::server::connection::Connection;

use super::ConnectionHandlerGetters;

pub trait ConnectionHandlerUtils<C: Connection>: ConnectionHandlerGetters<C> {
    fn send_message_to_client(&self, message: &dyn Display, nickname: &str) -> io::Result<()> {
        if self.database().is_local_client(nickname) {
            let mut stream = ok_or_return!(self.database().get_local_stream(nickname), Ok(()));
            return stream.send(&message);
        }

        let server = ok_or_return!(self.database().get_immediate_server(nickname), Ok(()));

        self.send_message_to_server(message, &server)
    }

    fn send_message_to_channel(&self, message: &dyn Display, channel: &str) {
        self.send_message_to_local_clients_on_channel(message, channel);

        let servers = self.get_channel_immediate_servers(channel);

        self.send_message_to_servers(servers, message);
    }

    fn send_message_to_local_clients_on_channel(&self, message: &dyn Display, channel: &str) {
        let clients = ok_or_return!(self.database().get_channel_clients(channel));

        for client in clients {
            if self.database().is_local_client(&client) {
                self.send_message_to_client(message, &client).ok();
            }
        }
    }

    fn send_message_to_server(&self, message: &dyn Display, server: &str) -> io::Result<()> {
        let mut stream = ok_or_return!(self.database().get_server_stream(server), Ok(()));
        stream.send(message)
    }

    fn send_message_to_all_servers(&self, message: &dyn Display) {
        let servers = self.database().get_all_servers();

        self.send_message_to_servers(servers, message);
    }

    fn send_message_to_target(&self, message: &dyn Display, target: &str) {
        if self.database().contains_client(target) {
            self.send_message_to_client(message, target).ok();
        } else {
            self.send_message_to_channel(message, target);
        }
    }

    fn send_message_to_servers(&self, servers: Vec<String>, message: &dyn Display) {
        for server in servers {
            self.send_message_to_server(message, &server).ok();
        }
    }

    fn get_channel_immediate_servers(&self, channel: &str) -> Vec<String> {
        let clients = ok_or_return!(self.database().get_channel_clients(channel), vec![]);

        let mut servers = vec![];

        for client in clients {
            if let Ok(server) = self.database().get_immediate_server(&client) {
                if !servers.contains(&server) {
                    servers.push(server);
                }
            }
        }

        servers
    }
}
