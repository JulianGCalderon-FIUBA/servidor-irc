use crate::server::client_handler::responses::replies::CommandResponse;
use crate::server::client_trait::ClientTrait;

use super::ClientHandler;

use std::io;
mod validations;

impl<T: ClientTrait> ClientHandler<T> {
    pub fn pass_command(&mut self, mut parameters: Vec<String>) -> io::Result<()> {
        if let Some(error) = self.assert_pass_command_is_valid(&parameters) {
            return self.send_response_for_error(error);
        }

        let password = parameters.remove(0);
        self.registration.set_attribute("password", password);

        self.send_response_for_reply(CommandResponse::Ok)
    }

    pub fn nick_command(&mut self, mut parameters: Vec<String>) -> io::Result<()> {
        if let Some(error) = self.assert_nick_command_is_valid(&parameters) {
            return self.send_response_for_error(error);
        }

        let nickname = parameters.remove(0);

        // if self.registration.state() == &RegistrationState::Registered {
        //     let prev_nickname = self.registration.nickname().unwrap();
        //     self.database.update_nickname(&prev_nickname, &nickname)
        // }

        self.registration.set_nickname(nickname);

        self.send_response_for_reply(CommandResponse::Ok)
    }

    pub fn user_command(
        &mut self,
        mut parameters: Vec<String>,
        trailing: Option<String>,
    ) -> io::Result<()> {
        if let Some(error) = self.assert_user_command_is_valid(&parameters, &trailing) {
            return self.send_response_for_error(error);
        }

        let realname = trailing.unwrap();
        let servername = parameters.pop().unwrap();
        let hostname = parameters.pop().unwrap();
        let username = parameters.pop().unwrap();

        self.registration.set_attribute("username", username);
        self.registration.set_attribute("hostname", hostname);
        self.registration.set_attribute("servername", servername);
        self.registration.set_attribute("realname", realname);

        let client = self.registration.build().unwrap();
        self.database.add_client(client);

        self.send_response_for_reply(CommandResponse::Ok)
    }

    pub fn oper_command(&mut self, parameters: Vec<String>) -> io::Result<()> {
        if let Some(error) = self.assert_oper_command_is_valid(&parameters) {
            return self.send_response_for_error(error);
        }

        let nickname = self.registration.nickname().unwrap();
        self.database.set_server_operator(&nickname);

        self.send_response_for_reply(CommandResponse::YouAreOper381)
    }

    pub fn quit_command(&mut self, trailing: Option<String>) -> io::Result<()> {
        let message = trailing.unwrap_or_else(|| self.registration.nickname().unwrap_or_default());

        self.send_response_for_reply(CommandResponse::Quit { message })
    }
}
