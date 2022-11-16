use std::{
    collections::HashMap,
    io,
    sync::{atomic::AtomicBool, Arc},
    time::Instant,
};

use crate::server::{connection::Connection, database::DatabaseHandle};

use super::connection_handler_trait::{
    ConnectionHandler, ConnectionHandlerCommands, ConnectionHandlerGetters,
    ConnectionHandlerStructure, ConnectionHandlerUtils,
};

mod asserts;
mod logic;
mod utils;

const REGISTRATION_TIMELIMIT_SECS: u64 = 60;

pub struct RegistrationHandler<C: Connection> {
    stream: C,
    stream_for_database: Option<C>,
    database: DatabaseHandle<C>,
    servername: String,
    online: Arc<AtomicBool>,
    attributes: HashMap<&'static str, String>,
    timestamp: Instant,
}

impl<C: Connection> ConnectionHandler<C> for RegistrationHandler<C> {
    fn from_connection(
        stream: C,
        servername: String,
        database: DatabaseHandle<C>,
        online: Arc<AtomicBool>,
    ) -> std::io::Result<Self> {
        let stream_for_database = Some(stream.try_clone()?);

        Ok(Self {
            stream,
            stream_for_database,
            database,
            servername,
            online,
            attributes: HashMap::new(),
            timestamp: Instant::now(),
        })
    }
}

impl<C: Connection> ConnectionHandlerGetters<C> for RegistrationHandler<C> {
    fn online(&self) -> &Arc<AtomicBool> {
        &self.online
    }

    fn connection(&mut self) -> &mut C {
        &mut self.stream
    }

    fn database(&self) -> &DatabaseHandle<C> {
        &self.database
    }
}

impl<C: Connection> ConnectionHandlerStructure<C> for RegistrationHandler<C> {
    fn on_try_handle_error(&mut self) {
        eprintln!("Connection with unregistered client ended unexpectedly")
    }
    fn on_try_handle_success(&mut self) {
        eprintln!("Closing conection with unregistered client")
    }

    fn timeout(&mut self) -> bool {
        let elapsed_time = Instant::now().duration_since(self.timestamp);
        let elapsed_time_secs = elapsed_time.as_secs();

        elapsed_time_secs > REGISTRATION_TIMELIMIT_SECS
    }

    fn on_timeout(&mut self) -> io::Result<()> {
        self.send_response(&"Registration timeout")
    }
}

impl<C: Connection> ConnectionHandlerCommands<C> for RegistrationHandler<C> {}
