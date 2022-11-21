use std::io;

use crate::message::{CreationError, Message};

use super::{
    connection::Connection,
    consts::commands::SERVER_COMMAND,
    database::{ClientInfo, DatabaseHandle, ExternalServer},
    responses::Notification,
};

pub struct Register<C: Connection> {
    stream: C,
    database: DatabaseHandle<C>,
    servername: String,
}

impl<C: Connection> Register<C> {
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
        let notification = Notification::Server {
            servername: self.servername.clone(), //self.database.get_server_name()
            hopcount: 1,
            serverinfo: "info".to_string(), //self.database.get_server_info()
        };

        let message = Message::new(&notification.to_string()).unwrap();
        message.send_to(&mut self.stream)
    }

    fn receive_server_notification(&mut self) -> io::Result<()> {
        let message = match Message::read_from(&mut self.stream) {
            Ok(message) => message,
            Err(error) => return Err(parse_creation_error(error)),
        };

        let (_, command, mut params, trail) = message.unpack();
        assert_is_valid_server_message(&command, &params, &trail)?;

        let hopcount = params.remove(1).parse::<usize>().unwrap();
        let servername = params.remove(0);
        let serverinfo = trail.unwrap();
        self.handle_server_command(servername, hopcount, serverinfo)?;

        Ok(())
    }

    fn handle_server_command(
        &mut self,
        servername: String,
        hopcount: usize,
        serverinfo: String,
    ) -> Result<(), io::Error> {
        let server = ExternalServer::new(
            self.stream.try_clone()?,
            servername.clone(),
            serverinfo,
            hopcount,
        );
        self.database.add_server(server);

        self.servername = servername;

        Ok(())
    }

    fn send_server_data(&mut self) -> io::Result<()> {
        for client in self.database.get_all_clients() {
            self.send_nick_notification(&client)?;
            self.send_user_notification(&client)?;
        }

        Ok(())
    }

    fn send_user_notification(&mut self, client: &ClientInfo) -> io::Result<()> {
        let user_notification = Notification::User {
            nickname: client.nickname.clone(),
            username: client.username.clone(),
            hostname: client.hostname.clone(),
            servername: client.servername.clone(),
            realname: client.realname.clone(),
        };

        let message = Message::new(&user_notification.to_string()).unwrap();
        message.send_to(&mut self.stream)
    }

    fn send_nick_notification(&mut self, client: &ClientInfo) -> io::Result<()> {
        let nick_notification = Notification::Nick {
            nickname: client.nickname.clone(),
            hopcount: client.hopcount + 1,
        };
        let message = Message::new(&nick_notification.to_string()).unwrap();

        message.send_to(&mut self.stream)
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
        "Did not receiver server response",
    )
}
