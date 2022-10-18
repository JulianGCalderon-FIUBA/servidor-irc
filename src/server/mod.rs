use std::{io, net::TcpListener, sync::Arc};

use crate::thread_pool::ThreadPool;
use client_handler::ClientHandler;
pub use database::Database;

pub const MAX_CLIENTS: usize = 26;

mod client_handler;
mod database;

/// Represents a Server clients can connect to.
pub struct Server {
    database: Database,
}

impl Server {
    /// Starts new Server.
    pub fn start() -> Self {
        Self {
            database: Database::new(),
        }
    }

    /// Listens for incoming clients and handles each request in a new thread.
    pub fn listen_to(self, address: String) -> io::Result<()> {
        let listener = TcpListener::bind(address)?;

        let database_arc = Arc::new(self.database);

        let pool = ThreadPool::create(MAX_CLIENTS);

        for client in listener.incoming() {
            let database_clone = Arc::clone(&database_arc);

            if let Ok(client) = client {
                pool.execute(|| {
                    let handler = ClientHandler::new(database_clone, client);
                    handler.handle();
                })
            }
        }

        Ok(())
    }

    pub fn backup() {
        todo!()
    }
}
