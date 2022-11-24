use std::fmt::Display;
use std::io;

use crate::server::connection::Connection;

use super::ConnectionHandlerGetters;

pub trait ConnectionHandlerUtils<C: Connection>: ConnectionHandlerGetters<C> {
    fn send_message_to_client(&mut self, message: &dyn Display, nickname: &str) -> io::Result<()> {
        if let Some(stream) = self.database().get_local_stream(nickname) {
            stream?.send(&message)?;
        } else {
            let server = self.database().get_immediate_server(nickname).unwrap();
            self.send_message_to_server(message, &server).ok();
        }
        Ok(())
    }

    fn send_message_to_channel(&mut self, message: &dyn Display, channel: &str) {
        let clients = self.database().get_local_clients_for_channel(channel);

        let mut servers = vec![];

        for client in clients {
            if self.database().is_local_client(&client) {
                self.send_message_to_client(message, &client).ok();
            } else {
                let server = self.database().get_immediate_server(&client).unwrap();
                if !servers.contains(&server) {
                    servers.push(server);
                }
            }
        }

        for server in servers {
            self.send_message_to_server(message, &server).ok();
        }
    }

    fn send_message_to_local_clients_on_channel(&mut self, message: &dyn Display, channel: &str) {
        let clients = self.database().get_clients_for_channel(channel);

        for client in clients {
            self.send_message_to_client(message, &client).ok();
        }
    }

    fn send_message_to_server(&mut self, message: &dyn Display, server: &str) -> io::Result<()> {
        if let Some(stream) = self.database().get_server_stream(server) {
            stream?.send(&message)?;
        }
        Ok(())
    }

    fn send_message_to_all_servers(&mut self, message: &dyn Display) {
        let servers = self.database().get_all_servers();

        for server in servers {
            self.send_message_to_server(message, &server).ok();
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
