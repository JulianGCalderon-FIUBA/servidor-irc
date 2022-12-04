use std::io;

use crate::{
    macros::ok_or_return,
    message::{CreationError, Message},
};

use super::{
    connection::Connection,
    consts::{
        channel::DISTRIBUTED_CHANNEL,
        commands::SERVER_COMMAND,
        modes::{SET_BANMASK, SET_KEY, SET_OPERATOR, SET_SPEAKER, SET_USER_LIMIT},
    },
    database::DatabaseHandle,
    responses::{ErrorReply, Notification},
};

use crate::server::data_structures::*;

/// MethodObject
/// Manages connection between servers and information exchange.
/// It is in charge of sharing all local information to the new server and registering incoming information in database.
pub struct ServerConnectionSetup<C: Connection> {
    stream: C,
    database: DatabaseHandle<C>,
    servername: String,
}

impl<C: Connection> ServerConnectionSetup<C> {
    /// Creates a [`ServerConnectionSetup`] from a connection stream
    ///   and a database in which to register the new connection.
    pub fn new(stream: C, database: DatabaseHandle<C>) -> Self {
        Self {
            stream,
            database,
            servername: Default::default(),
        }
    }

    /// Registers server from an outcoming connection.
    pub fn register_outcoming(&mut self) -> io::Result<()> {
        self.send_server_notification()?;
        self.receive_server_notification()?;
        self.send_server_data()
    }

    /// Registers server from an incoming connection.
    pub fn register_incoming(
        &mut self,
        servername: String,
        hopcount: usize,
        serverinfo: String,
    ) -> io::Result<()> {
        self.handle_server_command(servername, hopcount, serverinfo)?;
        self.send_server_notification()?;
        self.send_server_data()
    }

    pub fn servername(&self) -> String {
        self.servername.clone()
    }

    fn send_server_notification(&mut self) -> io::Result<()> {
        let servername = self.database.get_server_name();
        let serverinfo = self.database.get_server_info();

        self.stream
            .send(&Notification::server(&servername, 1, &serverinfo))
    }

    /// Waits for server command from incoming connection and handles it.
    ///
    /// Fails if server command is not valid or there was a parsing error.
    fn receive_server_notification(&mut self) -> io::Result<()> {
        let message = match Message::read_from(&mut self.stream) {
            Ok(message) => message,
            Err(error) => return Err(parse_creation_error(error)),
        };

        let (_, command, mut params, trail) = message.unpack();
        assert_is_valid_server_message(&command, &params, &trail)?;

        let hopcount = params
            .remove(1)
            .parse::<usize>()
            .expect("Hopcount should be a number");
        let servername = params.remove(0);
        let serverinfo = trail.expect("Trail should be Some");
        self.handle_server_command(servername, hopcount, serverinfo)?;

        Ok(())
    }

    /// Registers server from incoming connection.
    ///
    /// Fails if server is already registered.
    fn handle_server_command(
        &mut self,
        servername: String,
        hopcount: usize,
        serverinfo: String,
    ) -> Result<(), io::Error> {
        let server = ImmediateServer::new(
            self.stream.try_clone()?,
            servername.clone(),
            serverinfo,
            hopcount,
        );

        self.assert_can_add_server(&server.info().servername)?;
        self.database.add_immediate_server(server);
        self.servername = servername;

        Ok(())
    }

    fn assert_can_add_server(&mut self, servername: &str) -> io::Result<()> {
        if self.database.contains_server(servername) {
            let command = SERVER_COMMAND.to_string();
            let message = "Servername already registered".to_string();
            self.stream
                .send(&ErrorReply::UnknownError400 { command, message })?;

            return Err(already_registered_error());
        }

        Ok(())
    }

    // Sends all server data to new connected server:
    //  - all clients (with nick and user)
    //  - all channels (with join)
    //  - all channel configurations (with mode)
    //  - all server operators (with mode)
    fn send_server_data(&mut self) -> io::Result<()> {
        for mut client in self.database.get_all_clients() {
            client.hopcount += 1;
            self.send_nick_notification(&client)?;
            self.send_user_notification(&client)?;
            if client.is_operator() {
                self.send_oper_notification(&client)?;
            }
        }
        for channel in self.database.get_all_channels() {
            if !channel.starts_with(DISTRIBUTED_CHANNEL) {
                continue;
            }
            let clients = ok_or_return!(self.database.get_channel_clients(&channel), Ok(()));
            clients.iter().for_each(|client| {
                self.send_join_notification(client, &channel).ok();
            });
            self.send_channel_mode_is_notification(&channel)?;
        }

        Ok(())
    }

    fn send_user_notification(&mut self, client: &ClientInfo) -> io::Result<()> {
        self.stream.send(&Notification::user(client))
    }

    fn send_nick_notification(&mut self, client: &ClientInfo) -> io::Result<()> {
        self.stream
            .send(&Notification::nick(&client.nickname(), client.hopcount))
    }

    fn send_oper_notification(&mut self, client: &ClientInfo) -> io::Result<()> {
        self.stream.send(&Notification::mode(
            &client.nickname(),
            &client.nickname(),
            "+o",
        ))
    }

    fn send_join_notification(&mut self, nickname: &str, channel: &str) -> io::Result<()> {
        self.stream.send(&Notification::join(nickname, channel))
    }

    pub(super) fn send_channel_mode_is_notification(&mut self, channel: &str) -> io::Result<()> {
        let config = ok_or_return!(self.database.get_channel_config(channel), Ok(()));

        let flags = config.flags;
        let limit = config.user_limit;
        let operators = config.operators;
        let banmasks = config.banmasks;
        let speakers = config.speakers;
        let key = config.key;
        let sender = &operators[0].clone();

        for flag in flags {
            let request = format!("+{}", flag.to_char());
            let notification = Notification::mode(sender, channel, &request);

            self.stream.send(&notification)?;
        }

        if let Some(limit) = limit {
            let request = format!("+{SET_USER_LIMIT} {limit}");
            let notification = Notification::mode(sender, channel, &request);

            self.stream.send(&notification)?;
        }

        if let Some(key) = key {
            let request = format!("+{SET_KEY} {:?}", key);
            let notification = Notification::mode(sender, channel, &request);

            self.stream.send(&notification)?;
        }

        for operator in operators {
            let request = format!("+{SET_OPERATOR} {operator}");
            let notification = Notification::mode(sender, channel, &request);

            self.stream.send(&notification)?;
        }

        for banmask in banmasks {
            let request = format!("+{SET_BANMASK} {:?}", banmask);
            let notification = Notification::mode(sender, channel, &request);

            self.stream.send(&notification)?;
        }

        for speaker in speakers {
            let request = format!("+{SET_SPEAKER} {:?}", speaker);
            let notification = Notification::mode(sender, channel, &request);

            self.stream.send(&notification)?;
        }

        Ok(())
    }
}

fn parse_creation_error(err: CreationError) -> io::Error {
    match err {
        crate::message::CreationError::IoError(error) => error,
        crate::message::CreationError::ParsingError(_) => invalid_input_error(),
    }
}

fn assert_is_valid_server_message(
    command: &str,
    params: &Vec<String>,
    trail: &Option<String>,
) -> io::Result<()> {
    if command != SERVER_COMMAND {
        return Err(invalid_input_error());
    }

    if params.len() < 2 || trail.is_none() {
        return Err(invalid_input_error());
    }

    if params[1].parse::<usize>().is_err() {
        return Err(invalid_input_error());
    }

    Ok(())
}

fn invalid_input_error() -> io::Error {
    io::Error::new(
        io::ErrorKind::InvalidInput,
        "Did not receive valid server notification",
    )
}

fn already_registered_error() -> io::Error {
    io::Error::new(io::ErrorKind::Unsupported, "Server is already registered")
}
