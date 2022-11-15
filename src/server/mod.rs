#[cfg(test)]
/// Utils for testing different functionalities.
mod testing_utils;

/// Contains structure for client handler. It's main purpose is to handle the connection established between server and clients.
mod client_handler;

/// Definition of the trait used in the project's structures.
mod client_trait;

/// Contains structure for database. A Database stores and updates information regarding clients, channels and related.
mod database;
mod listener;

use client_handler::ClientHandler;
use database::Database;
use std::io;
use std::net::TcpStream;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::{self, JoinHandle};

use self::database::DatabaseHandle;
use self::listener::ConnectionListener;

const MAX_CLIENTS: usize = 26;

pub const OPER_USERNAME: &str = "admin";
pub const OPER_PASSWORD: &str = "admin";

/// Represents a Server clients can connect to it contains a Database that stores relevant information.
pub struct Server {
    servername: String,
    database: Option<DatabaseHandle<TcpStream>>,
    online: Arc<AtomicBool>,
    threads: Vec<JoinHandle<()>>,
}

impl Server {
    /// Starts new [`Server`].
    pub fn start(servername: &str) -> Self {
        let servername = servername.to_string();
        let online = Arc::new(AtomicBool::new(true));

        let (database, database_thread) = Database::start();

        let threads = vec![database_thread];
        let database = Some(database);

        Self {
            servername,
            online,
            database,
            threads,
        }
    }

    pub fn quit(&self) {
        self.online.store(false, Ordering::Relaxed);
    }

    /// Listens for incoming clients and handles each request in a new thread.
    pub fn spawn_listener(&mut self, address: String) -> io::Result<()> {
        let database = match &self.database {
            Some(database) => database.clone(),
            None => {
                eprintln!("Already listening");
                return Ok(());
            }
        };

        let online = Arc::clone(&self.online);
        let servername = self.servername.clone();

        let connection_listener = ConnectionListener::new(servername, address, database, online)?;

        let thread = thread::spawn(|| connection_listener.listen());

        self.threads.push(thread);

        self.database.take();

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
