use std::sync::{atomic::AtomicBool, Arc};

use crate::server::{connection::Connection, database::DatabaseHandle};

use super::connection_handler_trait::{
    ConnectionHandler, ConnectionHandlerCommands, ConnectionHandlerGetters,
    ConnectionHandlerStructure,
};

mod asserts;
mod logic;
mod utils;

pub struct ServerHandler<C: Connection> {
    stream: C,                   // stream del servidor propio
    _servername: String,         // server al que está conectado (original)
    database: DatabaseHandle<C>, // handler para el servidor original
    online: Arc<AtomicBool>,     // sobre el server original
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
            _servername: servername,
            database,
            online,
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
        todo!()
    }
    fn on_try_handle_success(&mut self) {
        todo!()
    }
}

impl<C: Connection> ConnectionHandlerCommands<C> for ServerHandler<C> {}
