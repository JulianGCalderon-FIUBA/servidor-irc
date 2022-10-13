use std::{
    io,
    net::TcpListener,
    sync::{Arc, RwLock},
};

use client_handler::ClientHandler;

use crate::thread_pool::ThreadPool;

pub mod client_handler;
pub mod client_info;

pub const MAX_CLIENTS: usize = 5;

pub struct Server {}

impl Server {
    pub fn start() -> Self {
        Self {}
    }

    pub fn listen_to(self, address: String) -> io::Result<()> {
        let listener = TcpListener::bind(address)?;

        let server_lock = Arc::new(RwLock::new(self));

        let pool = ThreadPool::create();

        for client in listener.incoming().take(MAX_CLIENTS) {
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
