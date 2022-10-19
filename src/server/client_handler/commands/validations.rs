use super::super::connection_info::RegistrationState;
use super::ClientHandler;
use std::io;

use super::PASS_COMMAND;
use super::USER_COMMAND;

impl ClientHandler {
    pub fn validate_pass_command(&mut self, parameters: &Vec<String>) -> io::Result<bool> {
        if parameters.len() != 1 {
            self.need_more_params_error(PASS_COMMAND)?;
            return Ok(false);
        }

        if self.connection.registration_state != RegistrationState::NotInitialized {
            self.already_registered_response()?;
            return Ok(false);
        }

        Ok(true)
    }

    pub fn validate_nick_command(&mut self, parameters: &Vec<String>) -> io::Result<bool> {
        if parameters.is_empty() {
            self.no_nickname_given_error()?;
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

        if self.connection.registration_state != RegistrationState::NicknameSent {
            self.no_nickname_error()?;
            return Ok(false);
        }

        Ok(true)
    }
}
