use std::io;
use std::net::TcpListener;

use client_handler::ClientHandler;

pub mod client_handler;

pub struct Server {}

impl Server {
    pub fn listening_to(address: String) -> io::Result<()> {
        let mut server = Server::new();

        let listener = TcpListener::bind(address)?;

        for client in listener.incoming() {
            let handler = ClientHandler::new(&mut server, client?);
            handler.handle();
        }

        Ok(())
    }

    fn new() -> Self {
        Self {}
    }
}
