use crate::message::{CreationError, Message};
use crate::server::client_info::ClientInfo;
use crate::server::Server;
use std::{io, net::TcpStream};

pub struct ClientHandler<'a> {
    /// en el futuro puede ser:
    ///     - Arc<Mutex<Server>>
    ///     - Arc<RwLock<Server>>
    ///         + la exclusividad solo es necesaria para la escritura, para evitar condiciones de carrera.
    ///     - Arc<Server> donde cada campo particular contenga su lock.
    ///         + tiene mejor performance, pero mas tedioso de implementar
    ///         + algunos campos podrian ser de solo lectura, por lo que seria innecesaria un lock
    _server: &'a mut Server,
    client: ClientInfo,
}

impl<'a> ClientHandler<'a> {
    pub fn new(server: &'a mut Server, stream: TcpStream) -> Self {
        let client = ClientInfo { stream };

        Self {
            _server: server,
            client,
        }
    }

    pub fn handle(self) {
        if let Err(error) = self.try_handle() {
            eprintln!("Error handling client: {:?}", error);
        }
    }

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
