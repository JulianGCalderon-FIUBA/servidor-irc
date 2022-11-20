#[cfg(test)]
/// Utils for testing different functionalities.
mod testing_utils;

/// Contains structure for client handler. It's main purpose is to handle the connection established between server and clients.
pub mod client_handler;

/// Definition of the trait used in the project's structures.
mod client_trait;

/// Contains structure for database. A Database stores and updates information regarding clients, channels and related.
mod database;

use crate::thread_pool::ThreadPool;
use client_handler::ClientHandler;
use database::Database;
use std::io;
use std::net::{TcpListener, TcpStream};

use self::database::DatabaseHandle;

const MAX_CLIENTS: usize = 26;

pub const OPER_USERNAME: &str = "admin";
pub const OPER_PASSWORD: &str = "admin";

/// Represents a Server clients can connect to it contains a Database that stores relevant information.
pub struct Server {
    servername: String,
    database: DatabaseHandle<TcpStream>,
}

impl Server {
    /// Starts new [`Server`].
    pub fn start(servername: &str) -> Self {
        let database = Database::start();

        let servername = servername.to_string();

        Self {
            database,
            servername,
        }
    }

    /// Listens for incoming clients and handles each request in a new thread.
    pub fn listen_to(self, address: String) -> io::Result<()> {
        let listener = TcpListener::bind(address)?;

        let pool = ThreadPool::create(MAX_CLIENTS);

        for client in listener.incoming() {
            match self.handler(client) {
                Ok(handler) => pool.execute(|| handler.handle()),
                Err(error) => eprintln!("Could not create handler {error:?}"),
            }
        }

        Ok(())
    }

    fn handler(&self, client: io::Result<TcpStream>) -> io::Result<ClientHandler<TcpStream>> {
        let database = self.database.clone();
        ClientHandler::<TcpStream>::from_stream(database, client?, self.servername.clone())
    }
}
