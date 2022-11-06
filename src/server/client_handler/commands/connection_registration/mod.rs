use crate::server::client_handler::registration::RegistrationState;
use crate::server::client_handler::responses::notifications::Notification;
use crate::server::client_handler::responses::replies::CommandResponse;
use crate::server::client_trait::ClientTrait;

use super::ClientHandler;

use std::io;
/// This module contains validations for connection registration operations.
mod validations;

impl<T: ClientTrait> ClientHandler<T> {
    /// Saves password.
    pub fn pass_command(&mut self, mut parameters: Vec<String>) -> io::Result<()> {
        if let Some(error) = self.assert_pass_command_is_valid(&parameters) {
            return self.send_response_for_error(error);
        }

        let password = parameters.remove(0);
        self.registration.set_attribute("password", password);

        Ok(())
    }
    /// Saves nickname.
    pub fn nick_command(&mut self, mut parameters: Vec<String>) -> io::Result<()> {
        if let Some(error) = self.assert_nick_command_is_valid(&parameters) {
            return self.send_response_for_error(error);
        }

        let nickname = parameters.remove(0);

        if self.registration.state() == &RegistrationState::Registered {
            let prev_nickname = self.registration.nickname().unwrap();
            self.database.update_nickname(&prev_nickname, &nickname)
        }

        self.registration.set_nickname(nickname);

        Ok(())
    }
    /// Saves username, hostname, servername and realname.
    /// Finishes registration.
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

        Ok(())
    }
    /// Sets client as server operator.
    pub fn oper_command(&mut self, parameters: Vec<String>) -> io::Result<()> {
        if let Some(error) = self.assert_oper_command_is_valid(&parameters) {
            return self.send_response_for_error(error);
        }

        let nickname = self.registration.nickname().unwrap();
        self.database.set_server_operator(&nickname);

        self.send_response_for_reply(CommandResponse::YouAreOper381)
    }
    /// Quits server and ends connection.
    pub fn quit_command(&mut self, trailing: Option<String>) -> io::Result<()> {
        let content = trailing.unwrap_or_else(|| self.registration.nickname().unwrap_or_default());

        let notification = Notification::Quit { message: content };

        if let Some(nickname) = self.registration.nickname() {
            self.database.disconnect_client(&nickname);
            let channels = self.database.get_channels_for_client(&nickname);
            for channel in channels {
                self.send_message_to_channel(&channel, &notification.to_string());
            }
        }

        self.send_response(&notification.to_string())?;

        Ok(())
    }
}
