use crate::server::client_handler::connection::RegistrationState;

use super::ClientHandler;

use std::io::{self, Read, Write};

mod validations;

pub const NICK_COMMAND: &str = "NICK";
pub const OPER_COMMAND: &str = "OPER";
pub const PASS_COMMAND: &str = "PASS";
pub const QUIT_COMMAND: &str = "QUIT";
pub const USER_COMMAND: &str = "USER";

impl<T: Read + Write> ClientHandler<T> {
    pub fn pass_command(&mut self, mut parameters: Vec<String>) -> io::Result<()> {
        if !self.validate_pass_command(&parameters)? {
            return Ok(());
        }

        let password = parameters.pop().unwrap();
        self.connection.set_password(password);

        self.ok_reply()
    }

    pub fn nick_command(&mut self, mut parameters: Vec<String>) -> io::Result<()> {
        if !self.validate_nick_command(&parameters)? {
            return Ok(());
        }

        let nickname = parameters.pop().unwrap();
        self.connection.set_nickname(nickname);

        if self.connection.state() == &RegistrationState::NotInitialized {
            self.connection.advance_state();
        }

        self.ok_reply()
    }

    pub fn oper_command(&mut self, parameters: Vec<String>) -> io::Result<()> {
        // let user = self.database.password.clone().unwrap();
        // let password = self.database.password.clone().unwrap();
        if !self.validate_oper_command(&parameters /*, &user, &password */)? {
            return Ok(());
        }

        self.database
            .set_server_operator(&self.connection.nickname().unwrap());

        self.oper_reply()
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

        self.connection
            .set_info(username, hostname, servername, realname);

        self.connection.advance_state();

        self.database.add_client(self.connection.build().unwrap());

        self.ok_reply()
    }

    pub fn quit_command(&mut self, trailing: Option<String>) -> io::Result<()> {
        if let Some(trailing) = trailing {
            return self.quit_reply(&trailing);
        }

        let nickname = self.connection.nickname().unwrap();
        self.quit_reply(&nickname)
    }
}
