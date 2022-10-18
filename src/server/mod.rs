use std::io;
use std::net::TcpListener;

use client_handler::ClientHandler;
use client_info::ClientInfo;

mod client_handler;
mod client_info;

pub struct Server {
    _clients: Vec<ClientInfo>,
}

impl Server {
    pub fn start() -> Self {
        Self { _clients: vec![] }
    }

    pub fn listen_to(mut self, address: String) -> io::Result<()> {
        let listener = TcpListener::bind(address)?;

        for client in listener.incoming() {
            let handler = ClientHandler::new(&mut self, client?);
            handler.handle();
        }

        Ok(())
    }

    pub fn backup() {
        todo!()
    }
}
