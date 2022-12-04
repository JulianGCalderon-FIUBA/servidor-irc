use crate::server::{connection::Connection, data_structures::ClientInfo};

use super::{database_error::DatabaseError, Database};

/// Contains requests that have a boolean response.
mod booleans;
/// Contains requests related to a channel's configuration.
mod channel_configuration;
/// Contains requests related to channels.
mod channels;
/// Contains requests related to clients.
mod clients;
/// Contains requests related to servers.
mod servers;

impl<C: Connection> Database<C> {
    pub fn get_client_info(&mut self, nickname: &str) -> Result<&mut ClientInfo, DatabaseError> {
        if let Some(client) = self.local_clients.get_mut(nickname) {
            return Ok(client.info_mut());
        }
        if let Some(client) = self.external_clients.get_mut(nickname) {
            return Ok(client.info_mut());
        }
        Err(DatabaseError::NoSuchClient)
    }
}
