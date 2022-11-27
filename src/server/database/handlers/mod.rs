use crate::server::{connection::Connection, data_structures::ClientInfo};

use super::{database_error::DatabaseError, Database};

mod booleans;
mod channel_configuration;
mod channels;
mod clients;
mod servers;

impl<C: Connection> Database<C> {
    pub fn get_client_info(&mut self, nickname: &str) -> Result<&mut ClientInfo, DatabaseError> {
        if let Some(client) = self.local_clients.get_mut(nickname) {
            return Ok(&mut client.info);
        }
        if let Some(client) = self.external_clients.get_mut(nickname) {
            return Ok(&mut client.info);
        }
        Err(DatabaseError::NoSuchClient)
    }
}
