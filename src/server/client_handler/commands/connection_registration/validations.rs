use crate::server::client_handler::registration::RegistrationState;
use crate::server::client_trait::ClientTrait;

use super::ClientHandler;
use std::io;

use super::OPER_COMMAND;
use super::PASS_COMMAND;
use super::USER_COMMAND;

impl<T: ClientTrait> ClientHandler<T> {
    pub fn validate_pass_command(&mut self, parameters: &Vec<String>) -> io::Result<bool> {
        if parameters.len() != 1 {
            self.need_more_params_error(PASS_COMMAND)?;
            return Ok(false);
        }

        if self.registration.state() != &RegistrationState::NotInitialized {
            self.already_registered_reply()?;
            return Ok(false);
        }

        Ok(true)
    }

    pub fn validate_nick_command(&mut self, parameters: &Vec<String>) -> io::Result<bool> {
        if parameters.is_empty() {
            self.no_nickname_given_error()?;
            return Ok(false);
        }

        let nickname = &parameters[0];

        if self.database.contains_client(nickname) {
            if self.registration.state() == &RegistrationState::Registered {
                self.nickname_in_use_reply()?;
            } else {
                self.nickname_collision_reply()?;
            }
            return Ok(false);
        }

        Ok(true)
    }

    pub fn validate_user_command(
        &mut self,
        parameters: &Vec<String>,
        trailing: &Option<String>,
    ) -> io::Result<bool> {
        if parameters.len() != 3 || trailing.is_none() {
            self.need_more_params_error(USER_COMMAND)?;
            return Ok(false);
        }

        if self.registration.state() != &RegistrationState::NicknameSent {
            self.no_nickname_error()?;
            return Ok(false);
        }

        Ok(true)
    }

    pub fn validate_oper_command(&mut self, parameters: &Vec<String>) -> io::Result<bool> {
        if parameters.len() != 2 {
            self.need_more_params_error(OPER_COMMAND)?;
            return Ok(false);
        }

        if self.registration.state() != &RegistrationState::Registered {
            self.unregistered_error()?;
            return Ok(false);
        }

        Ok(true)
    }
}
