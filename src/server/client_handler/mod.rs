use std::io;

use crate::message::Message;

mod commands;
mod connection_info;
mod responses;

use commands::channel_operations::{
    INVITE_COMMAND, JOIN_COMMAND, LIST_COMMAND, NAMES_COMMAND, PART_COMMAND,
};
use commands::connection_registration::{
    NICK_COMMAND, OPER_COMMAND, PASS_COMMAND, QUIT_COMMAND, USER_COMMAND,
};
use commands::sending_messages::{NOTICE_COMMAND, PRIVMSG_COMMAND};

use std::net::TcpStream;
use std::sync::Arc;

use super::database::Database;
use crate::message::{CreationError, ParsingError};
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

        let nickname = self.connection.nickname;

        if let Some(nickname) = nickname.as_ref() {
            self.database.disconnect_client(nickname);
        }

        match conection_result {
            Ok(()) => println!(
                "Closing conection with client [{}]",
                nickname.unwrap_or_default()
            ),
            Err(error) => eprintln!(
                "Conection with client [{}] failed with error [{}]",
                nickname.unwrap_or_default(),
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
                PRIVMSG_COMMAND => self.privmsg_command(parameters, trailing)?,
                NOTICE_COMMAND => self.notice_command(parameters, trailing)?,
                PART_COMMAND => self.part_command(parameters)?,
                JOIN_COMMAND => self.join_command(parameters)?,
                NAMES_COMMAND => self.names_command(parameters)?,
                LIST_COMMAND => self.list_command()?,
                OPER_COMMAND => self.oper_command(parameters)?,
                INVITE_COMMAND => self.invite_command(parameters /* , trailing*/)?,
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
