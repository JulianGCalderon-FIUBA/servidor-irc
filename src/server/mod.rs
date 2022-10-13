use std::{
    io,
    net::TcpListener,
    sync::{Arc, RwLock},
};
/// This module contains the functionality to handle a client's request.
pub mod client_handler;

/// This module contains a client's information.
pub mod client_info;

use crate::thread_pool::ThreadPool;
use client_handler::ClientHandler;

pub const MAX_CLIENTS: usize = 5;

/// Represents a Server clients can connect to.
pub struct Server {}

impl Server {
    /// Starts new Server.
    pub fn start() -> Self {
        Self {}
    }

    /// Listens for incoming clients and handles each request in a new thread.
    pub fn listen_to(self, address: String) -> io::Result<()> {
        let listener = TcpListener::bind(address)?;

        let server_lock = Arc::new(RwLock::new(self));

        let pool = ThreadPool::create();

        for client in listener.incoming() {
            let server_lock_clone = Arc::clone(&server_lock);

            if let Ok(client) = client {
                pool.execute(|| {
                    let handler = ClientHandler::new(server_lock_clone, client);
                    handler.handle();
                })
            }
        }

        Ok(())
    }
}
