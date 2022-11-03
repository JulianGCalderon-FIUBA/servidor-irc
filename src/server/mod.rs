#[cfg(test)]
mod testing_utils;

mod client_handler;
mod client_trait;
mod database;

use crate::thread_pool::ThreadPool;
use client_handler::ClientHandler;
use database::Database;
use std::io;
use std::net::{TcpListener, TcpStream};

use self::database::DatabaseHandle;

pub const MAX_CLIENTS: usize = 26;

/// Represents a Server clients can connect to.
pub struct Server {
    database: DatabaseHandle<TcpStream>,
}

impl Server {
    /// Starts new Server.
    pub fn start() -> Self {
        let database = Database::start();

        Self { database }
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
        ClientHandler::<TcpStream>::from_stream(database, client?)
    }
}
