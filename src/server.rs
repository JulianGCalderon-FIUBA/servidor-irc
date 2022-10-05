use std::io;

use std::net::{TcpListener, TcpStream};

use crate::message::{CreationError, Message};

pub struct Server {
    // id
    // base de datos
    listener: TcpListener,
}

impl Server {
    pub fn new(address: String) -> io::Result<Self> {
        let listener = TcpListener::bind(address)?;

        Ok(Self { listener })
    }

    pub fn listen(&mut self) -> io::Result<()> {
        for client in self.listener.incoming() {
            self.handle_client(client?)?;
        }

        Ok(())
    }

    fn handle_client(&self, mut client: TcpStream) -> io::Result<()> {
        loop {
            let message = match Message::read_from(&mut client) {
                Ok(message) => message,
                Err(CreationError::IoError(error)) => return Err(error),
                Err(CreationError::ParsingError(_)) => continue,
            };

            println!("Received: {}", message);
            message.send_to(&mut client)?;
        }
    }
}
