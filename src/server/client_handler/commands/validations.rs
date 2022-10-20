use super::super::connection_info::RegistrationState;
use super::ClientHandler;
use std::io;

use super::PART_COMMAND;
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

        let nickname = &parameters[0];

        if self.database.contains_client(nickname) {
            self.nickname_collision_response()?;
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

    pub fn _validate_part_command(
        &mut self,
        parameters: &Vec<String>,
        _nickname: &str,
    ) -> io::Result<bool> {
        if parameters.is_empty() {
            self.need_more_params_error(PART_COMMAND)?;
            return Ok(false);
        }
        // let channels = self.database._get_channels();
        // for (i, channel) in parameters.iter().enumerate() {
        //     if !channels.contains(&channel) {
        //         self.no_such_channel_error(&channel)?;
        //         parameters.remove(i);
        //     }

        //     let clients = self.database._get_clients(&channel);
        //     if !clients.contains(&nickname.to_string()) {
        //         self.not_on_channel_error(&channel)?;
        //         parameters.remove(i);
        //     }
        // }
        // if parameters.is_empty() {
        //     return Ok(false);
        // }
        Ok(true)
    }
}
