use super::connection_info::RegistrationState;
use super::ClientHandler;

use crate::server::database::ClientInfo;
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
        self.client.password = Some(password);

        self.ok_reply()
    }

    pub fn nick_command(&mut self, mut parameters: Vec<String>) -> io::Result<()> {
        if !self.validate_nick_command(&parameters)? {
            return Ok(());
        }

        let nickname = parameters.pop().unwrap();
        self.client.nickname = Some(nickname);

        if self.client.registration_state == RegistrationState::NotInitialized {
            self.client.advance_state();
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

        self.client.username = Some(username);
        self.client.hostname = Some(hostname);
        self.client.servername = Some(servername);
        self.client.realname = Some(realname);

        self.client.advance_state();

        self.add_client();

        self.ok_reply()
    }

    fn add_client(&mut self) {
        let mut database_arc_lock = self.database.write().unwrap();

        let client = ClientInfo::new_with_stream(
            self.client.stream.try_clone().unwrap(),
            self.client.password.clone().unwrap(),
            self.client.nickname.clone().unwrap(),
            self.client.username.clone().unwrap(),
            self.client.hostname.clone().unwrap(),
            self.client.servername.clone().unwrap(),
            self.client.realname.clone().unwrap(),
        );

        database_arc_lock.save_client(client);
    }

    pub fn quit_command(&mut self, trailing: Option<String>) -> io::Result<()> {
        if let Some(trailing) = trailing {
            return self.quit_reply(&trailing);
        }

        let nickname = self.client.nickname.clone().unwrap_or_default();
        self.quit_reply(&nickname)
    }
}
