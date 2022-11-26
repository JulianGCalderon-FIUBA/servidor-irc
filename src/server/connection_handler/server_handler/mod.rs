use std::{
    collections::HashMap,
    sync::{atomic::AtomicBool, Arc},
};

use crate::server::{connection::Connection, database::DatabaseHandle};

use super::connection_handler_trait::{
    ConnectionHandler, ConnectionHandlerCommands, ConnectionHandlerGetters,
    ConnectionHandlerStructure,
};

mod asserts;
mod logic;
// #[cfg(test)]
// mod tests;
mod utils;

pub struct ServerHandler<C: Connection> {
    stream: C,
    servername: String,
    database: DatabaseHandle<C>,
    online: Arc<AtomicBool>,
    hopcounts: HashMap<String, usize>,
    // falta info del server propio
}

impl<C: Connection> ConnectionHandler<C> for ServerHandler<C> {}

impl<C: Connection> ServerHandler<C> {
    pub fn from_connection(
        stream: C,
        servername: String,
        database: DatabaseHandle<C>,
        online: Arc<AtomicBool>,
    ) -> std::io::Result<Self> {
        Ok(Self {
            stream,
            servername,
            database,
            online,
            hopcounts: HashMap::new(),
        })
    }
}

impl<C: Connection> ConnectionHandlerGetters<C> for ServerHandler<C> {
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

impl<C: Connection> ConnectionHandlerStructure<C> for ServerHandler<C> {
    fn on_try_handle_error(&mut self) {
        eprintln!("Connection with [{}] ended unexpectedly", self.servername)
    }
    fn on_try_handle_success(&mut self) {
        eprintln!("Closing conection with [{}]", self.servername)
    }
}

impl<C: Connection> ConnectionHandlerCommands<C> for ServerHandler<C> {}
