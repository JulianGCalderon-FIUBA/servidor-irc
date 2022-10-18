use super::ClientHandler;
use crate::server::database::RegistrationState;
use std::io;

pub const PASS_COMMAND: &str = "PASS";
pub const NICK_COMMAND: &str = "NICK";
pub const USER_COMMAND: &str = "USER";
pub const QUIT_COMMAND: &str = "QUIT";

impl ClientHandler {
    pub fn pass_command(&mut self, mut parameters: Vec<String>) -> io::Result<()> {
        if parameters.len() != 1 {
            return self.need_more_params_error(PASS_COMMAND);
        }

        if self.client.registration_state != RegistrationState::NotInitialized {
            return self.already_registered_response();
        }

        let password = parameters.pop().unwrap();
        self.client.password = Some(password);

        self.ok_reply()
    }

    pub fn nick_command(&mut self, mut parameters: Vec<String>) -> io::Result<()> {
        if parameters.is_empty() {
            return self.no_nickname_given_error();
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
        if parameters.len() != 3 || trailing.is_none() {
            return self.need_more_params_error(USER_COMMAND);
        }

        if self.client.registration_state != RegistrationState::NicknameSent {
            return self.no_nickname_error();
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

        self.ok_reply()
    }

    pub fn quit_command(&mut self, trailing: Option<String>) -> io::Result<()> {
        if let Some(trailing) = trailing {
            return self.quit_reply(&trailing);
        }

        let nickname = self.client.nickname.clone().unwrap_or_default();
        self.quit_reply(&nickname)
    }
}
