mod client_handler;
mod database;

#[cfg(test)]
mod mock_stream;

use std::io::{self};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;

use crate::thread_pool::ThreadPool;
use client_handler::ClientHandler;
use database::Database;

pub const MAX_CLIENTS: usize = 26;

/// Represents a Server clients can connect to.
pub struct Server {
    database: Arc<Database<TcpStream>>,
}

impl Server {
    /// Starts new Server.
    pub fn start() -> Self {
        Self {
            database: Arc::new(Database::new()),
        }
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

            let database_clone: Arc<Database<TcpStream>> = Arc::clone(&self.database);
            let handler: ClientHandler<TcpStream> =
                match ClientHandler::<TcpStream>::new_from_stream(database_clone, client) {
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
