use std::io;
use std::sync::mpsc::Sender;

use crate::message::Message;

mod commands;
mod registration;
mod requests;
mod responses;

use commands::channel_operations::{
    INVITE_COMMAND, JOIN_COMMAND, LIST_COMMAND, NAMES_COMMAND, PART_COMMAND,
};
use commands::connection_registration::{
    NICK_COMMAND, OPER_COMMAND, PASS_COMMAND, QUIT_COMMAND, USER_COMMAND,
};
use commands::sending_messages::{NOTICE_COMMAND, PRIVMSG_COMMAND};
use commands::user_based_queries::WHOIS_COMMAND;

use self::commands::user_based_queries::WHO_COMMAND;
use self::responses::errors::ErrorReply;

use super::client_trait::ClientTrait;
use super::database::DatabaseMessage;
use crate::message::{CreationError, ParsingError};
use registration::Registration;

/// A ClientHandler handles the client's request.
pub struct ClientHandler<T: ClientTrait> {
    database: Sender<DatabaseMessage<T>>,
    stream: T,
    registration: Registration<T>,
}

impl<T: ClientTrait> ClientHandler<T> {
    /// Returns new clientHandler.

    pub fn from_stream(
        database: Sender<DatabaseMessage<T>>,
        stream: T,
    ) -> io::Result<ClientHandler<T>> {
        let registration = Registration::with_stream(stream.try_clone()?);

        Ok(Self {
            database,
            stream,
            registration,
        })
    }

    /// Handles the received requests with error handling
    pub fn handle(mut self) {
        let conection_result = self.try_handle();

        let nickname = self.registration.nickname();

        if let Some(nickname) = nickname.as_ref() {
            self.disconnect_client(nickname)
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
        self.send_response("200 :parsing error")
    }

    fn on_unknown_command(&mut self, command: &str) -> io::Result<()> {
        self.send_response_for_error(ErrorReply::UnknownCommand421 {
            command: command.to_string(),
        })
    }
}
