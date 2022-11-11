use std::io;
use std::io::BufReader;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, Receiver, RecvTimeoutError, Sender};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

use crate::message::Message;

/// This module contains commands the ClientHandler knows.
mod commands;
/// This module contains the structure that contains current Registration.
mod registration;
/// This module contains the responses a ClientHandler can have for the different commands.
mod responses;

use responses::errors::ErrorReply;

use self::commands::AWAY_COMMAND;
use self::registration::RegistrationState;

use super::client_trait::Connection;
use super::database::DatabaseHandle;
use crate::message::{CreationError, ParsingError};
use registration::Registration;

use commands::{
    INVITE_COMMAND, JOIN_COMMAND, KICK_COMMAND, LIST_COMMAND, NAMES_COMMAND, NICK_COMMAND,
    NOTICE_COMMAND, OPER_COMMAND, PART_COMMAND, PASS_COMMAND, PRIVMSG_COMMAND, QUIT_COMMAND,
    TOPIC_COMMAND, USER_COMMAND, WHOIS_COMMAND, WHO_COMMAND,
};

const REGISTRATION_TIMELIMIT_MS: u128 = 60 * 1000;
const READ_FROM_STREAM_TIMEOUT_MS: u64 = 100;

/// A ClientHandler handles the client's request.
pub struct ClientHandler<C: Connection> {
    database: DatabaseHandle<C>,
    stream: C,
    registration: Registration<C>,
    servername: String,
    online: Arc<AtomicBool>,
}

impl<C: Connection> ClientHandler<C> {
    /// Returns new [`ClientHandler`].
    pub fn from_stream(
        database: DatabaseHandle<C>,
        stream: C,
        servername: String,
        online: Arc<AtomicBool>,
    ) -> io::Result<ClientHandler<C>> {
        let registration = Registration::with_stream(stream.try_clone()?);

        Ok(Self {
            database,
            stream,
            registration,
            servername,
            online,
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

        self.stream.shutdown().ok();
    }

    /// Tries to handle the received request.
    ///
    /// # Panics
    ///
    /// `try_handle` fails if there is an IOError when reading the Message the client sent.
    ///
    fn try_handle(&mut self) -> io::Result<()> {
        let receiver = self.start_async_read_stream()?;

        loop {
            if self.server_shutdown() {
                self.on_shutdown();
                return Ok(());
            }

            if self.registration_timeout() {
                self.on_registration_timeout();
                return Ok(());
            }

            let timeout = Duration::from_millis(READ_FROM_STREAM_TIMEOUT_MS);
            let message = match receiver.recv_timeout(timeout) {
                Ok(message) => message,
                Err(RecvTimeoutError::Timeout) => continue,
                Err(RecvTimeoutError::Disconnected) => panic!(),
            };

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
                AWAY_COMMAND => self.away_command(trailing)?,
                TOPIC_COMMAND => self.topic_command(parameters)?,
                KICK_COMMAND => self.kick_command(parameters, trailing)?,
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

    fn on_shutdown(&mut self) {
        self.send_response("Server has shutdown").ok();
    }

    fn on_registration_timeout(&mut self) {
        self.send_response("Registration timeout").ok();
    }

    fn registration_timeout(&self) -> bool {
        if self.registration.state() == &RegistrationState::Registered {
            return false;
        }

        let elapsed_time = Instant::now().duration_since(self.registration.instant());
        let elapsed_time_ms = elapsed_time.as_millis();

        elapsed_time_ms > REGISTRATION_TIMELIMIT_MS
    }

    fn server_shutdown(&mut self) -> bool {
        !self.online.load(Ordering::Relaxed)
    }

    fn start_async_read_stream(&self) -> io::Result<Receiver<Result<Message, CreationError>>> {
        let (sender, receiver) = mpsc::channel();

        let stream = self.stream.try_clone()?;
        thread::spawn(|| async_read_stream(stream, sender));

        Ok(receiver)
    }
}

fn async_read_stream<C: Connection>(stream: C, sender: Sender<Result<Message, CreationError>>) {
    let mut reader = BufReader::new(stream);

    loop {
        let message = Message::read_from_buffer(&mut reader);

        if let Err(CreationError::IoError(_)) = message {
            if sender.send(message).is_err() {
                return;
            };
            break;
        }

        if sender.send(message).is_err() {
            return;
        };
    }
}
