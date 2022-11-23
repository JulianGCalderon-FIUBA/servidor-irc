#[cfg(test)]
/// Utils for testing different functionalities.
mod testing;

/// Definition of the trait used in the project's structures.
mod connection;

/// Contains structure for database. A Database stores and updates information regarding clients, channels and related.
mod database;

mod connection_handler;
/// Contains structure for connection listener, this structure listens to an address and handles all clients connecting to that address
mod listener;
mod registerer;

mod consts;
mod data_structures;
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
use self::registerer::Register;

const MAX_CLIENTS: usize = 26;

pub const OPER_USERNAME: &str = "admin";
pub const OPER_PASSWORD: &str = "admin";

/// Represents a Server clients can connect to it contains a Database that stores relevant information.
pub struct Server {
    database: Option<DatabaseHandle<TcpStream>>,
    online: Arc<AtomicBool>,
    threads: Vec<JoinHandle<()>>,
}

impl Server {
    /// Starts new [`Server`].
    pub fn start(servername: &str) -> Self {
        let servername = servername.to_string();
        let online = Arc::new(AtomicBool::new(true));

        let (database, database_thread) = Database::start(servername.clone(), servername);

        let threads = vec![database_thread];
        let database = Some(database);

        Self {
            online,
            database,
            threads,
        }
    }

    pub fn quit(&self) {
        self.online.store(false, Ordering::Relaxed);
    }

    /// Listens for incoming clients and handles each request in a new thread.
    pub fn listen_to(&mut self, address: String) -> io::Result<()> {
        let online = Arc::clone(&self.online);
        let database = self.database.clone().unwrap();

        let connection_listener = ConnectionListener::new(address, database, online)?;

        let thread = thread::spawn(|| connection_listener.listen());

        self.threads.push(thread);

        Ok(())
    }

    fn try_connect_to(&mut self, address: &str) -> io::Result<()> {
        let stream = TcpStream::connect(address)?;
        let database = self.database.as_ref().unwrap().clone();

        let mut registerer = Register::new(stream.try_clone()?, database.clone());
        registerer.register_outcoming()?;

        let online = Arc::clone(&self.online);
        let servername = registerer.servername();
        let server_handle =
            ServerHandler::from_connection(stream.try_clone()?, servername, database, online)?;

        let handle = thread::spawn(|| server_handle.handle());
        self.threads.push(handle);

        Ok(())
    }

    pub fn connect_to(&mut self, address: &str) {
        if let Err(error) = self.try_connect_to(address) {
            eprintln!("Could not connect to {address}, with error {error:?}");
        }
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

#[cfg(debug_assertions)]
macro_rules! debug_print {
    ($( $args:expr ),*) => { println!( $( $args ),* ); }
}

#[cfg(not(debug_assertions))]
macro_rules! debug_print {
    ($( $args:expr ),*) => {};
}

pub(crate) use debug_print;
