use super::connection_info::RegistrationState;
use super::ClientHandler;

use crate::server::database::ClientInfoBuilder;
use std::io;

mod responses;
mod validations;

pub const PASS_COMMAND: &str = "PASS";
pub const NICK_COMMAND: &str = "NICK";
pub const USER_COMMAND: &str = "USER";
pub const QUIT_COMMAND: &str = "QUIT";

impl ClientHandler {
    pub fn pass_command(&mut self, mut parameters: Vec<String>) -> io::Result<()> {
        if !self.validate_pass_command(&parameters)? {
            return Ok(());
        }

        let password = parameters.pop().unwrap();
        self.connection.password = Some(password);

        self.ok_reply()
    }

    pub fn nick_command(&mut self, mut parameters: Vec<String>) -> io::Result<()> {
        if !self.validate_nick_command(&parameters)? {
            return Ok(());
        }

        let nickname = parameters.pop().unwrap();
        self.connection.nickname = Some(nickname);

        if self.connection.registration_state == RegistrationState::NotInitialized {
            self.connection.advance_state();
        }

        self.ok_reply()
    }

    pub fn user_command(
        &mut self,
        mut parameters: Vec<String>,
        trailing: Option<String>,
    ) -> io::Result<()> {
        if !self.validate_user_command(&parameters, &trailing)? {
            return Ok(());
        }

        let realname = trailing.unwrap();
        let username = parameters.pop().unwrap();
        let hostname = parameters.pop().unwrap();
        let servername = parameters.pop().unwrap();

        self.connection.username = Some(username);
        self.connection.hostname = Some(hostname);
        self.connection.servername = Some(servername);
        self.connection.realname = Some(realname);

        self.connection.advance_state();

        self.add_client_to_database()?;

        self.ok_reply()
    }

    fn add_client_to_database(&mut self) -> io::Result<()> {
        let mut client_builder = ClientInfoBuilder::new_with(
            self.connection.nickname.clone().unwrap(),
            self.connection.username.clone().unwrap(),
            self.connection.hostname.clone().unwrap(),
            self.connection.servername.clone().unwrap(),
            self.connection.realname.clone().unwrap(),
        );

        client_builder.with_stream(self.connection.stream.try_clone()?);

        if let Some(password) = self.connection.password.clone() {
            client_builder.with_password(password);
        }

        self.database.add_client(client_builder.build());

        Ok(())
    }

    pub fn quit_command(&mut self, trailing: Option<String>) -> io::Result<()> {
        if let Some(trailing) = trailing {
            return self.quit_reply(&trailing);
        }

        let nickname = self.connection.nickname.clone().unwrap_or_default();
        self.quit_reply(&nickname)
    }
}
