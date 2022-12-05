use std::sync::{atomic::AtomicBool, Arc};

use crate::server::{connection::Connection, database::DatabaseHandle};

use super::{
    ConnectionHandler, ConnectionHandlerCommands, ConnectionHandlerGetters,
    ConnectionHandlerStructure,
};
/// Asserts to ensure the commands the client sends are valid.
mod asserts;
/// Auxiliar functions that have a boolean answer.
mod booleans;
/// Logic for the commands a client may send.
mod logic;
/// Responses generated by the commands.
mod responses;
/// Extra functions that help with the command's logic.
mod utils;

/// Unit tests for each command.
#[cfg(test)]
mod tests;

/// A Client Handler handles the connection with an already registered client.
pub struct ClientHandler<C: Connection> {
    stream: C,
    database: DatabaseHandle<C>,
    nickname: String,
    online: Arc<AtomicBool>,
}
impl<C: Connection> ConnectionHandler<C> for ClientHandler<C> {}

impl<C: Connection> ClientHandler<C> {
    /// Starts a [`ClientHandler`] with the received information.
    pub fn from_connection(
        stream: C,
        nickname: String,
        database: DatabaseHandle<C>,
        online: Arc<AtomicBool>,
    ) -> std::io::Result<Self> {
        Ok(Self {
            stream,
            database,
            online,
            nickname,
        })
    }
}

impl<C: Connection> ConnectionHandlerGetters<C> for ClientHandler<C> {
    fn online(&self) -> &Arc<AtomicBool> {
        &self.online
    }

    fn stream(&mut self) -> &mut C {
        &mut self.stream
    }

    fn database(&self) -> &DatabaseHandle<C> {
        &self.database
    }
}

impl<C: Connection> ConnectionHandlerStructure<C> for ClientHandler<C> {
    fn on_try_handle_error(&mut self) {
        eprintln!("Connection with [{}] ended unexpectedly", self.nickname)
    }
    fn on_try_handle_success(&mut self) {
        eprintln!("Closing conection with [{}]", self.nickname)
    }
}

impl<C: Connection> ConnectionHandlerCommands<C> for ClientHandler<C> {}
