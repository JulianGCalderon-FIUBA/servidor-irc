use crate::message::{CreationError, Message};
use crate::server::Server;
use std::{io, net::TcpStream};

pub struct ClientHandler<'a> {
    //en el futuro tiene que ser un Arc<Mutex<Server>>
    _server: &'a Server,
    //client: ClientInfo,
    client: TcpStream,
}

impl<'a> ClientHandler<'a> {
    pub fn new(server: &'a Server, client: TcpStream) -> ClientHandler<'a> {
        Self {
            _server: server,
            client,
        }
    }

    pub fn handle(&mut self) {
        match self.handle_client() {
            Ok(_) => (),
            Err(_) => eprintln!("Error handling client"),
        };
    }

    fn handle_client(&mut self) -> io::Result<()> {
        loop {
            let message = match Message::read_from(&mut self.client) {
                Ok(message) => message,
                Err(CreationError::IoError(error)) => return Err(error),
                Err(CreationError::ParsingError(_)) => continue,
            };

            println!("Received: {}", message);
            message.send_to(&mut self.client)?;
        }
    }
}
