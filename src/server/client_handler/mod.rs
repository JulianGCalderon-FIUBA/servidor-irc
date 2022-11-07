use std::io;

use crate::message::Message;

/// This module contains commands the ClientHandler knows.
mod commands;
/// This module contains the structure that contains current Registration.
mod registration;
/// This module contains the responses a ClientHandler can have for the different commands.
mod responses;

use responses::errors::ErrorReply;

use super::client_trait::Connection;
use super::database::DatabaseHandle;
use crate::message::{CreationError, ParsingError};
use registration::Registration;

use commands::{
    INVITE_COMMAND, JOIN_COMMAND, LIST_COMMAND, NAMES_COMMAND, NICK_COMMAND, NOTICE_COMMAND,
    OPER_COMMAND, PART_COMMAND, PASS_COMMAND, PRIVMSG_COMMAND, QUIT_COMMAND, USER_COMMAND,
    WHOIS_COMMAND, WHO_COMMAND,
};

/// A ClientHandler handles the client's request.
pub struct ClientHandler<C: Connection> {
    database: DatabaseHandle<C>,
    stream: C,
    registration: Registration<C>,
    servername: String,
}

impl<C: Connection> ClientHandler<C> {
    /// Returns new [`ClientHandler`].
    pub fn from_stream(
        database: DatabaseHandle<C>,
        stream: C,
        servername: String,
    ) -> io::Result<ClientHandler<C>> {
        let registration = Registration::with_stream(stream.try_clone()?);

        Ok(Self {
            database,
            stream,
            registration,
            servername,
        })
    }

    /// Handles the received requests with error handling.
    pub fn handle(mut self) {
        let conection_result = self.try_handle();

        let nickname = self.registration.nickname();

        if let Some(nickname) = &nickname {
            self.database.disconnect_client(nickname)
        }

        match conection_result {
            Ok(()) => println!(
                "Closing conection with client [{}]",
                nickname.unwrap_or_default()
            ),
            Err(error) => eprintln!(
                "Conection with client [{}] failed with error [{error}]",
                nickname.unwrap_or_default(),
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
                OPER_COMMAND => self.oper_command(parameters)?,
                PRIVMSG_COMMAND => self.privmsg_command(parameters, trailing)?,
                NOTICE_COMMAND => self.notice_command(parameters, trailing)?,
                JOIN_COMMAND => self.join_command(parameters)?,
                PART_COMMAND => self.part_command(parameters)?,
                INVITE_COMMAND => self.invite_command(parameters)?,
                NAMES_COMMAND => self.names_command(parameters)?,
                LIST_COMMAND => self.list_command(parameters)?,
                WHO_COMMAND => self.who_command(parameters)?,
                WHOIS_COMMAND => self.whois_command(parameters)?,
                QUIT_COMMAND => {
                    self.quit_command(trailing)?;
                    return Ok(());
                }
                _ => self.on_unknown_command(&command)?,
            };
        }
    }

    fn on_parsing_error(&mut self, _error: &ParsingError) -> io::Result<()> {
        self.send_response_for_error(ErrorReply::ParsingError)
    }

    fn on_unknown_command(&mut self, command: &str) -> io::Result<()> {
        self.send_response_for_error(ErrorReply::UnknownCommand421 {
            command: command.to_string(),
        })
    }
}
