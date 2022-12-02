use std::fmt::Display;
use std::io;

use crate::macros::ok_or_return;
use crate::server::connection::Connection;

use super::ConnectionHandlerGetters;

pub trait ConnectionHandlerUtils<C: Connection>: ConnectionHandlerGetters<C> {
    fn send_message_to_client(&self, message: &dyn Display, nickname: &str) -> io::Result<()> {
        if let Ok(mut stream) = self.database().get_local_stream(nickname) {
            return stream.send(&message);
        }

        if let Ok(server) = self.database().get_immediate_server(nickname) {
            self.send_message_to_server(message, &server).ok();
        }

        Ok(())
    }

    fn send_message_to_channel(&self, message: &dyn Display, channel: &str) {
        let clients = ok_or_return!(self.database().get_channel_clients(channel));

        let mut servers = vec![];

        for client in clients {
            if self.database().is_local_client(&client) {
                self.send_message_to_client(message, &client).ok();
            } else if let Ok(server) = self.database().get_immediate_server(&client) {
                if !servers.contains(&server) {
                    servers.push(server);
                }
            }
        }

        for server in servers {
            self.send_message_to_server(message, &server).ok();
        }
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
        if let Ok(mut stream) = self.database().get_server_stream(server) {
            stream.send(&message)?;
        }

        Ok(())
    }

    fn send_message_to_all_servers(&self, message: &dyn Display) {
        let servers = self.database().get_all_servers();

        for server in servers {
            self.send_message_to_server(message, &server).ok();
        }
    }

    fn send_message_to_target(&self, message: &dyn Display, target: &str) -> io::Result<()> {
        if self.database().contains_client(target) {
            self.send_message_to_client(message, target)?
        } else {
            self.send_message_to_channel(message, target);
        }

        Ok(())
    }
}
