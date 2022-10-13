use std::io;
use std::net::TcpListener;

use client_handler::ClientHandler;

pub mod client_handler;
pub mod client_info;

pub struct Server {}

impl Server {
    pub fn start() -> Self {
        Self {}
    }

    pub fn listen_to(mut self, address: String) -> io::Result<()> {
        let listener = TcpListener::bind(address)?;

        for client in listener.incoming() {
            let handler = ClientHandler::new(&mut self, client?);
            handler.handle();
        }

        Ok(())
    }
}
