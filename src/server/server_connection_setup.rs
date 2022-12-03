use std::io;

use crate::message::{CreationError, Message};

use super::{
    connection::Connection,
    consts::commands::SERVER_COMMAND,
    database::DatabaseHandle,
    responses::{ErrorReply, Notification},
};

use crate::server::data_structures::*;

pub struct ServerConnectionSetup<C: Connection> {
    stream: C,
    database: DatabaseHandle<C>,
    servername: String,
}

impl<C: Connection> ServerConnectionSetup<C> {
    pub fn new(stream: C, database: DatabaseHandle<C>) -> Self {
        Self {
            stream,
            database,
            servername: Default::default(),
        }
    }

    pub fn register_outcoming(&mut self) -> io::Result<()> {
        self.send_server_notification()?;
        self.receive_server_notification()?;
        self.send_server_data()
    }

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

    fn send_server_data(&mut self) -> io::Result<()> {
        for mut client in self.database.get_all_clients() {
            client.hopcount += 1;
            self.send_nick_notification(&client)?;
            self.send_user_notification(&client)?;
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
