#[cfg(test)]
mod testing_utils;

mod client_handler;
mod client_trait;
mod database;

use std::io;
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc;

use crate::thread_pool::ThreadPool;
use client_handler::ClientHandler;
use database::Database;

use self::database::DatabaseMessage;

pub const MAX_CLIENTS: usize = 26;

/// Represents a Server clients can connect to.
pub struct Server {
    database: mpsc::Sender<DatabaseMessage<TcpStream>>,
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
            let client = match client {
                Ok(client) => client,
                Err(error) => {
                    eprintln!("Could not establish connection with client, with error: {error:?}");
                    continue;
                }
            };

            let database = mpsc::Sender::clone(&self.database);
            let handler: ClientHandler<TcpStream> =
                match ClientHandler::<TcpStream>::from_stream(database, client) {
                    Ok(handler) => handler,
                    Err(error) => {
                        eprintln!("Could not create handler for client, with error: {error:?}");
                        continue;
                    }
                };

            pool.execute(|| {
                handler.handle();
            })
        }

        Ok(())
    }
}
