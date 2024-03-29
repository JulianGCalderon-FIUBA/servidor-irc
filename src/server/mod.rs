#[cfg(test)]
/// Utils for testing different functionalities.
mod testing;

/// Definition of the trait used in the project's structures.
mod connection;

/// Contains structure for database. A Database stores and updates information regarding clients, channels and other servers.
mod database;

/// Contains structure for connection handler. A ConnectionHandler may be:
///     - ClientHandler
///     - ServerHandler
///     - RegistrationHandler
/// The three structures must implement asserts and commands that can be received.
pub mod connection_handler;

/// Contains structure for connection listener, this structure listens to an address and handles all clients connecting to that address.
mod listener;

/// Contains structure that handles the setup when two servers are connecting with each other.
mod server_connection_setup;

/// Contains constant values used throughout the project.
pub(crate) mod consts;

/// Contains structures used to store information:
///     - Client
///     - Channel
///     - Server
mod data_structures;

/// Contains different responses to commands that may be received:
///     - Notifications
///     - Errors
///     - Responses
mod responses;

use database::Database;
use std::io;
use std::net::TcpStream;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::{self, JoinHandle};

use self::connection_handler::{ConnectionHandler, ServerHandler};
use self::database::DatabaseHandle;
use self::listener::ConnectionListener;
use self::server_connection_setup::ServerConnectionSetup;

const MAX_CLIENTS: usize = 26;

pub const OPER_USERNAME: &str = "admin";
pub const OPER_PASSWORD: &str = "admin";

/// Represents a Server clients and other servers can connect to.
/// Contains a Database that stores relevant information.
pub struct Server {
    database: Option<DatabaseHandle<TcpStream>>,
    online: Arc<AtomicBool>,
    threads: Vec<JoinHandle<()>>,
}

impl Server {
    /// Creates new [`Server`], with a name and a description.
    pub fn start(servername: String, serverinfo: String) -> Self {
        let online = Arc::new(AtomicBool::new(true));

        let (database, database_thread) = Database::start(servername, serverinfo);

        let threads = vec![database_thread];
        let database = Some(database);

        Self {
            online,
            database,
            threads,
        }
    }

    /// Marks server as offline, closing all its threads.
    pub fn quit(&self) {
        self.online.store(false, Ordering::Relaxed);
    }

    /// Listens for incoming clients from an address and handles each request in a new thread.
    pub fn listen_to(&mut self, address: String) -> io::Result<()> {
        let online = Arc::clone(&self.online);
        let database = self
            .database
            .clone()
            .expect("DatabaseHandle should only be None when dropped");

        let connection_listener = ConnectionListener::new(address, database, online)?;

        let thread = thread::spawn(|| connection_listener.listen());

        self.threads.push(thread);

        Ok(())
    }

    /// Establishes a connection with another server, listening from address.
    pub fn connect_to(&mut self, address: &str) {
        if let Err(error) = self.try_connect_to(address) {
            eprintln!("Could not connect to {address}, with error {error:?}");
        }
    }

    fn try_connect_to(&mut self, address: &str) -> io::Result<()> {
        let stream = TcpStream::connect(address)?;
        let database = self
            .database
            .clone()
            .expect("DatabaseHandle should only be None when dropped");

        let mut registerer = ServerConnectionSetup::new(stream.try_clone()?, database.clone());

        registerer.register_outcoming()?;

        let online = Arc::clone(&self.online);
        let servername = registerer.servername();
        let server_handle =
            ServerHandler::from_connection(stream.try_clone()?, servername, database, online)?;

        let handle = thread::spawn(|| server_handle.handle());
        self.threads.push(handle);

        Ok(())
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        self.quit();

        if let Some(database) = self.database.take() {
            drop(database);
        }

        for thread in self.threads.drain(..) {
            thread.join().ok();
        }
    }
}
