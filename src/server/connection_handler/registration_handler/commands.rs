use std::io;

use crate::server::connection::Connection;
use crate::server::connection_handler::connection_handler_trait::{
    ConnectionHandlerAsserts, ConnectionHandlerCommands, ConnectionHandlerUtils,
};

use super::RegistrationHandler;

impl<C: Connection> ConnectionHandlerCommands<C> for RegistrationHandler<C> {
    fn pass_command(&mut self, mut parameters: Vec<String>) -> io::Result<()> {
        if let Err(error) = self.assert_pass_command_is_valid(&parameters) {
            self.send_response(&error)?;
        }

        let password = parameters.pop().unwrap();
        self.attributes.insert("password", password);

        Ok(())
    }

    fn nick_command(&mut self, mut parameters: Vec<String>) -> io::Result<()> {
        if let Err(error) = self.assert_nick_command_is_valid(&parameters) {
            self.send_response(&error)?;
        }

        let nickname = parameters.pop().unwrap();
        self.attributes.insert("nickname", nickname);

        Ok(())
    }

    fn user_command(
        &mut self,
        mut parameters: Vec<String>,
        trailing: Option<String>,
    ) -> io::Result<()> {
        if let Err(error) = self.assert_user_command_is_valid(&parameters, &trailing) {
            self.send_response(&error)?;
        }

        let realname = trailing.unwrap();
        let username = parameters.pop().unwrap();
        let servername = self.servername.to_string();
        let hostname = self.stream.peer_address()?.ip().to_string();

        self.attributes.insert("username", username);
        self.attributes.insert("hostname", hostname);
        self.attributes.insert("servername", servername);
        self.attributes.insert("realname", realname);

        let client = self.build_client();

        self.database.add_client(client.unwrap());

        Ok(())
    }
}
