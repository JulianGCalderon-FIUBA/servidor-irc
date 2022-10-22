use super::super::connection_info::RegistrationState;
use super::ClientHandler;
use std::io;

use super::JOIN_COMMAND;
use super::PART_COMMAND;
use super::PASS_COMMAND;
use super::USER_COMMAND;

const MAX_CHANNELS: usize = 10;
const INVALID_CHARACTER: char = '\'';

const LOCAL_CHANNEL: u8 = b'&';
const DISTRIBUTED_CHANNEL: u8 = b'#';

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

    pub fn validate_channel_exists(&mut self, channel: &str) -> io::Result<bool> {
        let channels_database = self.database._get_channels();
        if !channels_database.contains(&channel.to_string()) {
            return Ok(false);
        }

        Ok(true)
    }

    pub fn validate_part_command(
        &mut self,
        parameters: &Vec<String>,
        _nickname: &str,
    ) -> io::Result<bool> {
        if parameters.is_empty() {
            self.need_more_params_error(PART_COMMAND)?;
            return Ok(false);
        }
        Ok(true)
    }

    pub fn validate_join_command(&mut self, parameters: &Vec<String>) -> io::Result<bool> {
        if parameters.is_empty() {
            self.need_more_params_error(JOIN_COMMAND)?;
            return Ok(false);
        }
        Ok(true)
    }

    fn validate_channel_name(&mut self, channel: &str) -> io::Result<bool> {
        if ((channel.as_bytes()[0] == LOCAL_CHANNEL)
            || (channel.as_bytes()[0] == DISTRIBUTED_CHANNEL))
            && !channel.contains(INVALID_CHARACTER)
        {
            return Ok(true);
        }
        Ok(false)
    }

    pub fn validate_can_join_channel(&mut self, channel: &str, nickname: &str) -> io::Result<bool> {
        let channels_for_nickname = self.database.get_channels_for_client(nickname);
        if channels_for_nickname.len() == MAX_CHANNELS {
            self.too_many_channels_error(channel)?;
            return Ok(false);
        }

        if !self.validate_channel_name(channel)? {
            self.no_such_channel_error(channel)?;
            return Ok(false);
        }

        Ok(true)
    }
}
