use std::io;
use std::sync::atomic::Ordering;

use crate::message::{CreationError, Message};
use crate::server::connection::Connection;
use crate::server::connection_handler::consts::commands::*;
use crate::server::connection_handler::responses::ErrorReply;

use super::{ConnectionHandlerCommands, ConnectionHandlerGetters, ConnectionHandlerUtils};

// const READ_FROM_STREAM_TIMEOUT_MS: u64 = 100;

pub trait ConnectionHandlerStructure<C: Connection>:
    ConnectionHandlerCommands<C> + ConnectionHandlerGetters<C> + ConnectionHandlerUtils<C>
{
    fn try_handle(&mut self) -> io::Result<()> {
        loop {
            if self.server_shutdown() {
                return self.on_server_shutdown();
            }

            if self.timeout() {
                return self.on_timeout();
            }

            let message = Message::read_from(self.stream());

            let message = match message {
                Ok(message) => message,
                Err(CreationError::IoError(error)) => match error.kind() {
                    io::ErrorKind::WouldBlock => continue,
                    _ => return Err(error),
                },
                Err(CreationError::ParsingError(_)) => {
                    self.on_parsing_error()?;
                    continue;
                }
            };

            if !self.handle_message(message)? {
                return Ok(());
            }
        }
    }

    fn handle_message(&mut self, message: Message) -> io::Result<bool> {
        let (_prefix, command, parameters, trailing) = message.unpack();

        match &command[..] {
            PASS_COMMAND => self.pass_command(parameters),
            NICK_COMMAND => self.nick_command(parameters),
            USER_COMMAND => self.user_command(parameters, trailing),
            OPER_COMMAND => self.oper_command(parameters),
            PRIVMSG_COMMAND => self.privmsg_command(parameters, trailing),
            NOTICE_COMMAND => self.notice_command(parameters, trailing),
            JOIN_COMMAND => self.join_command(parameters),
            PART_COMMAND => self.part_command(parameters),
            INVITE_COMMAND => self.invite_command(parameters),
            NAMES_COMMAND => self.names_command(parameters),
            LIST_COMMAND => self.list_command(parameters),
            WHO_COMMAND => self.who_command(parameters),
            WHOIS_COMMAND => self.whois_command(parameters),
            AWAY_COMMAND => self.away_command(trailing),
            TOPIC_COMMAND => self.topic_command(parameters),
            KICK_COMMAND => self.kick_command(parameters, trailing),
            MODE_COMMAND => self.mode_command(parameters),
            QUIT_COMMAND => self.quit_command(trailing),
            _ => self.on_unknown_command(command),
        }
    }

    fn on_try_handle_error(&mut self);
    fn on_try_handle_success(&mut self);

    fn server_shutdown(&mut self) -> bool {
        !self.online().load(Ordering::Relaxed)
    }

    fn timeout(&mut self) -> bool {
        false
    }

    fn on_server_shutdown(&mut self) -> io::Result<()> {
        self.send_response(&"Server has shutdown")
    }

    fn on_timeout(&mut self) -> io::Result<()> {
        Ok(())
    }

    fn on_parsing_error(&mut self) -> io::Result<()> {
        self.send_response(&ErrorReply::ParsingError)
    }
}
