mod commands;
mod connection_info;

use commands::NICK_COMMAND;
use commands::PASS_COMMAND;
use commands::QUIT_COMMAND;
use commands::USER_COMMAND;

use crate::message::{CreationError, Message, ParsingError};
use crate::server::database::Database;
use connection_info::ConnectionInfo;
use std::io;
use std::net::TcpStream;
use std::sync::Arc;

/// A ClientHandler handles the client's request.
pub struct ClientHandler {
    database: Arc<Database>,
    client: ConnectionInfo,
}

impl ClientHandler {
    /// Returns new clientHandler.
    pub fn new(database: Arc<Database>, stream: TcpStream) -> Self {
        let client = ConnectionInfo::with_stream(stream);

        Self { database, client }
    }

    ///
    pub fn handle(mut self) {
        let conection_result = self.try_handle();

        match conection_result {
            Ok(()) => println!(
                "Closing conection with client [{}]",
                self.client.nickname.unwrap_or_default()
            ),
            Err(error) => eprint!(
                "Conection with client [{}] failed with error [{}]",
                self.client.nickname.unwrap_or_default(),
                error
            ),
        }
    }

    /// Tries to handle the received request.
    ///
    /// # Panics
    ///
    /// `try_handle` fails if there is an IOError when reading the Message the client sent.
    ///
    fn try_handle(&mut self) -> io::Result<()> {
        loop {
            let message = match Message::read_from(&mut self.client.stream) {
                Ok(message) => message,
                Err(CreationError::IoError(error)) => return Err(error),
                Err(CreationError::ParsingError(error)) => {
                    self.on_parsing_error(&error)?;
                    continue;
                }
            };

            let (_prefix, command, parameters, trailing) = message.unpack();
            match &command[..] {
                PASS_COMMAND => self.pass_command(parameters)?,
                NICK_COMMAND => self.nick_command(parameters)?,
                USER_COMMAND => self.user_command(parameters, trailing)?,
                QUIT_COMMAND => {
                    self.quit_command(trailing)?;
                    return Ok(());
                }
                _ => self.unknown_command_error(&command)?,
            };
        }
    }

    fn on_parsing_error(&mut self, _error: &ParsingError) -> io::Result<()> {
        self.send_response("300 :parsing error")
    }
}
