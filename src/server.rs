use std::io;

use std::net::{TcpListener, TcpStream};

use crate::message::Message;

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
            self.handle_client(client?)?
        }

        Ok(())
    }

    fn handle_client(&self, mut client: TcpStream) -> io::Result<()> {
        let mut response_stream = client.try_clone()?;

        while let Ok(Some(message)) = Message::read_from(&mut client) {
            println!("Received: {}", message);
            message.send_to(&mut response_stream)?;
        }

        Ok(())
    }
}
