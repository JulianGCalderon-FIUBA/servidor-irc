mod commands;
mod connection_info;

use commands::JOIN_COMMAND;
use commands::NICK_COMMAND;
use commands::PART_COMMAND;
use commands::PASS_COMMAND;
use commands::QUIT_COMMAND;
use commands::USER_COMMAND;
use commands::NAMES_COMMAND;
use commands::LIST_COMMAND;

use std::io;
use std::net::TcpStream;
use std::sync::Arc;

use super::database::Database;
use crate::message::{CreationError, Message, ParsingError};
use connection_info::ConnectionInfo;

/// A ClientHandler handles the client's request.
pub struct ClientHandler {
    database: Arc<Database>,
    stream: TcpStream,
    connection: ConnectionInfo,
}

impl ClientHandler {
    /// Returns new clientHandler.
    pub fn new(database: Arc<Database>, stream: TcpStream) -> io::Result<Self> {
        let connection = ConnectionInfo::new_with_stream(stream.try_clone()?);

        Ok(Self {
            database,
            stream,
            connection,
        })
    }

    /// Handles the received requests with error handling
    pub fn handle(mut self) {
        let conection_result = self.try_handle();

        match conection_result {
            Ok(()) => println!(
                "Closing conection with client [{}]",
                self.connection.nickname.unwrap_or_default()
            ),
            Err(error) => eprintln!(
                "Conection with client [{}] failed with error [{}]",
                self.connection.nickname.unwrap_or_default(),
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
            let message = Message::read_from(&mut self.stream);

            let message = match message {
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
                PART_COMMAND => self.part_command(parameters)?,
                JOIN_COMMAND => self.join_command(parameters)?,
                QUIT_COMMAND => {
                    self.quit_command(trailing)?;
                    return Ok(());
                },
                NAMES_COMMAND => self.names_command(parameters)?,
                LIST_COMMAND => self.list_command()?,
                _ => self.unknown_command_error(&command)?,
            };
        }
    }

    fn on_parsing_error(&mut self, _error: &ParsingError) -> io::Result<()> {
        self.send_response("300 :parsing error")
    }
}
