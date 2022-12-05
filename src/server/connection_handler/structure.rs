use std::io;
use std::sync::atomic::Ordering;
use std::time::Duration;

use crate::message::{CreationError, Message};
use crate::server::connection::Connection;
use crate::server::consts::commands::*;
use crate::server::responses::ErrorReply;

use super::{
    ConnectionHandlerCommands, ConnectionHandlerGetters, ConnectionHandlerUtils,
    READ_FROM_STREAM_TIMEOUT_MS,
};

pub type CommandArgs = (Option<String>, Vec<String>, Option<String>);

pub trait ConnectionHandlerStructure<C: Connection>:
    ConnectionHandlerCommands<C> + ConnectionHandlerGetters<C> + ConnectionHandlerUtils<C>
{
    fn try_handle(&mut self) -> io::Result<()> {
        let timeout = Duration::from_millis(READ_FROM_STREAM_TIMEOUT_MS);
        self.stream()
            .set_read_timeout(Some(timeout))
            .expect("Duration should never be zero or None");

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
        let (prefix, command, parameters, trailing) = message.unpack();
        let arguments = (prefix, parameters, trailing);

        match &command[..] {
            PASS_COMMAND => self.pass_command(arguments),
            NICK_COMMAND => self.nick_command(arguments),
            USER_COMMAND => self.user_command(arguments),
            OPER_COMMAND => self.oper_command(arguments),
            PRIVMSG_COMMAND => self.privmsg_command(arguments),
            NOTICE_COMMAND => self.notice_command(arguments),
            JOIN_COMMAND => self.join_command(arguments),
            PART_COMMAND => self.part_command(arguments),
            INVITE_COMMAND => self.invite_command(arguments),
            NAMES_COMMAND => self.names_command(arguments),
            LIST_COMMAND => self.list_command(arguments),
            WHO_COMMAND => self.who_command(arguments),
            WHOIS_COMMAND => self.whois_command(arguments),
            AWAY_COMMAND => self.away_command(arguments),
            TOPIC_COMMAND => self.topic_command(arguments),
            KICK_COMMAND => self.kick_command(arguments),
            MODE_COMMAND => self.mode_command(arguments),
            QUIT_COMMAND => self.quit_command(arguments),
            SERVER_COMMAND => self.server_command(arguments),
            SQUIT_COMMAND => self.squit_command(arguments),
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
        self.stream().send(&"Server has shutdown")?;
        self.stream().shutdown()
    }

    fn on_timeout(&mut self) -> io::Result<()> {
        Ok(())
    }

    fn on_parsing_error(&mut self) -> io::Result<()> {
        self.stream().send(&ErrorReply::ParsingError)
    }
}
