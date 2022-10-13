use std::io;
use std::net::TcpListener;

use client_handler::ClientHandler;

pub mod client_handler;

pub struct Server {
    listener: TcpListener,
}

impl Server {
    pub fn new(address: String) -> io::Result<Self> {
        let listener = TcpListener::bind(address)?;

        Ok(Self { listener })
    }

    pub fn listen(mut self) -> io::Result<()> {
        let listener_clone = self.listener.try_clone()?;

        for client in listener_clone.incoming() {
            let handler = ClientHandler::new(&mut self, client?);
            handler.handle();
        }

        Ok(())
    }
}
