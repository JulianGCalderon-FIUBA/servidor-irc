mod commands;
mod commands_utils;

mod responses;

use self::commands::NICK_COMMAND;
use self::commands::PASS_COMMAND;
use self::commands::QUIT_COMMAND;
use self::commands::USER_COMMAND;

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

            let (_prefix, command, parameters, trailing) = message.unpack();
            match &command[..] {
                PASS_COMMAND => self.pass_command(/*prefix, */ &parameters, &trailing)?,
                NICK_COMMAND => self.nick_command(/*prefix, */ &parameters, &trailing)?,
                USER_COMMAND => self.user_command(/*prefix, */ &parameters, &trailing)?,
                // "OPER" => self.oper_command(prefix, parameters, trailing),
                QUIT_COMMAND => self.quit_command(/*prefix, */ &parameters, &trailing)?,
                _ => self.on_unknown_command(&command)?,
            };
        }
    }
    fn on_parsing_error(&self, _error: &ParsingError) {
        // todo!()
    }

    fn on_unknown_command(&mut self, _command: &str) -> io::Result<()> {
        let message = Message::new("Comando no entendido!").unwrap();
        message.send_to(&mut self.client.stream)
    }
}
