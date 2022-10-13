use crate::message::{CreationError, Message};
use crate::server::client_info::ClientInfo;
use crate::server::Server;
use std::sync::{Arc, RwLock};
use std::{io, net::TcpStream};

/// A ClientHandler handles the client's request.
pub struct ClientHandler {
    _server: Arc<RwLock<Server>>,
    client: ClientInfo,
}

impl ClientHandler {
    /// Returns new clientHandler.
    pub fn new(server: Arc<RwLock<Server>>, stream: TcpStream) -> Self {
        let client = ClientInfo { stream };

        Self {
            _server: server,
            client,
        }
    }

    /// Prints error if `try_handle` fails.
    ///
    pub fn handle(self) {
        if let Err(error) = self.try_handle() {
            eprintln!("Error handling client: {:?}", error);
        }
    }

    /// Tries to handle the received request.
    ///
    /// # Panics
    ///
    /// `try_handle` fails if there is an IOError when reading the Message the client sent.
    ///
    fn try_handle(mut self) -> io::Result<()> {
        loop {
            let message = match Message::read_from(&mut self.client.stream) {
                Ok(message) => message,
                Err(CreationError::IoError(error)) => return Err(error),
                Err(CreationError::ParsingError(_)) => continue,
            };

            println!("Received: {}", message);
            message.send_to(&mut self.client.stream)?;
        }
    }
}
