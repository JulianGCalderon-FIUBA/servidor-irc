use std::io;

use crate::server::connection::Connection;
use crate::server::connection_handler::connection_handler_trait::{
    ConnectionHandlerLogic, ConnectionHandlerUtils,
};
use crate::server::connection_handler::responses::Notification;

use super::connection_type::ConnectionType;
use super::RegistrationHandler;

impl<C: Connection> ConnectionHandlerLogic<C> for RegistrationHandler<C> {
    fn pass_logic(&mut self, mut params: Vec<String>) -> io::Result<bool> {
        let password = params.pop().unwrap();
        self.attributes.insert("password", password);

        Ok(true)
    }

    fn nick_logic(&mut self, mut params: Vec<String>) -> io::Result<bool> {
        let nickname = params.pop().unwrap();
        self.attributes.insert("nickname", nickname);

        Ok(true)
    }

    fn user_logic(
        &mut self,
        mut params: Vec<String>,
        trail: Option<String>,
    ) -> std::io::Result<bool> {
        let realname = trail.unwrap();
        let username = params.pop().unwrap();
        let servername = self.servername.to_string();
        let hostname = self.stream.peer_address()?.ip().to_string();

        self.attributes.insert("username", username);
        self.attributes.insert("hostname", hostname);
        self.attributes.insert("servername", servername);
        self.attributes.insert("realname", realname);

        let client = self.build_client();

        self.database.add_client(client.unwrap());

        self.connection_type = ConnectionType::Client;

        Ok(false)
    }

    fn quit_logic(&mut self, trail: Option<String>) -> io::Result<bool> {
        let message =
            trail.unwrap_or_else(|| self.attributes.remove("nickname").unwrap_or_default());

        let notification = Notification::Quit { message };

        self.send_response(&notification.to_string())?;

        Ok(false)
    }
}