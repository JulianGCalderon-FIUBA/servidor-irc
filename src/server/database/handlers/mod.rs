use crate::server::{connection::Connection, data_structures::ClientInfo};

use super::Database;

mod booleans;
mod channel_configuration;
mod channels;
mod clients;
mod servers;

impl<C: Connection> Database<C> {
    pub fn get_client_info(&mut self, nickname: &str) -> Option<&mut ClientInfo> {
        if let Some(client) = self.local_clients.get_mut(nickname) {
            return Some(&mut client.info);
        }
        if let Some(client) = self.external_clients.get_mut(nickname) {
            return Some(&mut client.info);
        }
        None
    }
}
