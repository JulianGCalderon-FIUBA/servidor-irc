use std::{
    collections::HashMap,
    io,
    sync::{atomic::AtomicBool, Arc},
    time::Instant,
};

use crate::server::{connection::Connection, database::DatabaseHandle};

use self::connection_type::ConnectionType;

use super::{
    client_handler::ClientHandler, ConnectionHandler, ConnectionHandlerCommands,
    ConnectionHandlerGetters, ConnectionHandlerStructure, ServerHandler,
};
/// Asserts to ensure the commands received are valid.
mod asserts;
/// Contains enum with different connection types.
mod connection_type;
/// Logic for the commands a connection may send.
mod logic;
/// Extra functions that help with the command's logic.
mod utils;

/// Unit tests for each command.
#[cfg(test)]
mod tests;

const REGISTRATION_TIMELIMIT_SECS: u64 = 60;

/// A Registration Handler handles a new connection.
/// It must save all new information in order to start corresponding handler later.
pub struct RegistrationHandler<C: Connection> {
    stream: C,
    stream_for_database: Option<C>,
    database: DatabaseHandle<C>,
    online: Arc<AtomicBool>,
    attributes: HashMap<&'static str, String>,
    timestamp: Instant,
    connection_type: ConnectionType,
}

impl<C: Connection> ConnectionHandler<C> for RegistrationHandler<C> {}

impl<C: Connection> RegistrationHandler<C> {
    /// Starts a [`RegistrationHandler`] with the received information.
    pub fn from_connection(
        stream: C,
        database: DatabaseHandle<C>,
        online: Arc<AtomicBool>,
    ) -> std::io::Result<Self> {
        let stream_for_database = Some(stream.try_clone()?);

        Ok(Self {
            stream,
            stream_for_database,
            database,
            online,
            attributes: HashMap::new(),
            timestamp: Instant::now(),
            connection_type: ConnectionType::Undefined,
        })
    }

    fn spawn_client_handler(&mut self) {
        let client_handler = match self.build_client_handler() {
            Ok(client_handler) => client_handler,
            Err(error) => return eprintln!("Could not initiate client handler, {error:?}"),
        };

        client_handler.handle();
    }

    fn build_client_handler(&mut self) -> io::Result<ClientHandler<C>> {
        ClientHandler::from_connection(
            self.stream().try_clone()?,
            self.attributes
                .remove("nickname")
                .expect("Client's nickname should be saved in attributes"),
            self.database().clone(),
            Arc::clone(self.online()),
        )
    }

    fn spawn_server_handler(&mut self) {
        let server_handler = match self.build_server_handler() {
            Ok(server_handler) => server_handler,
            Err(error) => return eprintln!("Could not initiate server handler, {error:?}"),
        };
        server_handler.handle();
    }

    fn build_server_handler(&mut self) -> io::Result<ServerHandler<C>> {
        ServerHandler::from_connection(
            self.stream().try_clone()?,
            self.attributes
                .remove("servername")
                .expect("Server's name should be saved in attributes"),
            self.database().clone(),
            Arc::clone(self.online()),
        )
    }
}

impl<C: Connection> ConnectionHandlerGetters<C> for RegistrationHandler<C> {
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

impl<C: Connection> ConnectionHandlerStructure<C> for RegistrationHandler<C> {
    fn on_try_handle_error(&mut self) {
        println!("Connection with unregistered client ended unexpectedly")
    }
    /// Spawns corresponding handler according to Connection Type.
    fn on_try_handle_success(&mut self) {
        match self.connection_type {
            ConnectionType::Undefined => println!("Closing connection with unregistered client"),
            ConnectionType::Server => self.spawn_server_handler(),
            ConnectionType::Client => self.spawn_client_handler(),
        }
    }

    fn timeout(&mut self) -> bool {
        let elapsed_time = Instant::now().duration_since(self.timestamp);
        let elapsed_time_secs = elapsed_time.as_secs();

        elapsed_time_secs > REGISTRATION_TIMELIMIT_SECS
    }

    fn on_timeout(&mut self) -> io::Result<()> {
        self.stream.send(&"Registration timeout")
    }
}

impl<C: Connection> ConnectionHandlerCommands<C> for RegistrationHandler<C> {}
