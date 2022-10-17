mod commands;
mod responses;

use super::ClientInfo;
use super::Server;
use crate::message::{CreationError, Message, ParsingError};
use std::io;
use std::net::TcpStream;

pub struct ClientHandler<'a> {
    /// en el futuro puede ser:
    ///     - Arc<Mutex<Server>>
    ///     - Arc<RwLock<Server>>
    ///         + la exclusividad solo es necesaria para la escritura, para evitar condiciones de carrera.
    ///     - Arc<Server> donde cada campo particular contenga su lock.
    ///         + tiene mejor performance, pero mas tedioso de implementar
    ///         + algunos campos podrian ser de solo lectura, por lo que seria innecesario un lock
    _server: &'a mut Server,
    client: ClientInfo,
}

impl<'a> ClientHandler<'a> {
    pub fn new(server: &'a mut Server, stream: TcpStream) -> Self {
        let client = ClientInfo::with_stream(stream);

        Self {
            _server: server,
            client,
        }
    }

    pub fn handle(mut self) {
        let conection_result = self.try_handle();

        match conection_result {
            Ok(()) => println!("Closing conection with client [{:?}]", self.client.nickname),
            Err(error) => eprint!(
                "Conection with client [{:?}] failed with error [{:?}]",
                self.client.nickname, error
            ),
        }
    }

    fn try_handle(&mut self) -> io::Result<()> {
        loop {
            let message = match Message::read_from(&mut self.client.stream) {
                Ok(message) => message,
                Err(CreationError::IoError(error)) => return Err(error),
                Err(CreationError::ParsingError(error)) => {
                    self.on_parsing_error(&error);
                    continue;
                }
            };

            let (prefix, command, parameters, trailing) = message.unpack();
            match &command[..] {
                "PASS" => self.pass_command(prefix, parameters, trailing),
                // "NICK" => self.nick_command(prefix, parameters, trailing),
                // "USER" => self.user_command(prefix, parameters, trailing),
                // "OPER" => self.oper_command(prefix, parameters, trailing),
                // "QUIT" => {
                //     self.quit_command(prefix, parameters, trailing);
                //     return Ok(());
                // }
                _ => self.on_unknown_command(&command),
            }
        }
    }

    fn on_parsing_error(&self, _error: &ParsingError) {
        // todo!()
    }

    fn on_unknown_command(&self, _command: &str) {
        // todo!()
    }
}
