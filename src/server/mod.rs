use std::io;
use std::net::TcpListener;

use self::client_handler::ClientHandler;

pub mod client_handler;

pub struct Server {
    listener: TcpListener,
}

impl Server {
    pub fn new(address: String) -> io::Result<Self> {
        let listener = TcpListener::bind(address)?;

        Ok(Self { listener })
    }

    pub fn listen(self) -> io::Result<()> {
        for client in self.listener.incoming() {
            let mut handler = ClientHandler::new(&self, client?);
            handler.handle();
        }

        Ok(())
    }
}
