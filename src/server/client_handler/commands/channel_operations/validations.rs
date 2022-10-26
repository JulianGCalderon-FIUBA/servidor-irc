use crate::server::client_handler::{
    commands::{DISTRIBUTED_CHANNEL, INVALID_CHARACTER, LOCAL_CHANNEL, MAX_CHANNELS},
    connection_info::RegistrationState,
};

use super::{ClientHandler, JOIN_COMMAND, PART_COMMAND};
use std::io;
// use std::sync::mpsc::channel;

impl ClientHandler {
    // GENERAL

    pub fn validate_channel_exists(&mut self, channel: &str) -> io::Result<bool> {
        let channels_database = self.database.get_channels();
        if !channels_database.contains(&channel.to_string()) {
            return Ok(false);
        }

        Ok(true)
    }

    pub fn validate_nickname_exits(&mut self, nickname: &str) -> io::Result<bool> {
        if !self.database.contains_client(nickname) {
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

        if self.validate_user_is_in_channel(channel, nickname)? {
            //El error ya es lanzado
            return Ok(false);
        }

        Ok(true)
    }

    pub fn validate_user_is_in_channel(
        &mut self,
        channel: &str,
        nickname: &str,
    ) -> io::Result<bool> {
        if !self
            .database
            .get_clients(channel)
            .contains(&String::from(nickname))
        {
            return Ok(false);
        }
        Ok(true)
    }

    // COMMANDS

    pub fn validate_invite_command(&mut self, parameters: &Vec<String>) -> io::Result<bool> {
        if parameters.len() != 2 {
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

    pub fn validate_list_command(&mut self) -> io::Result<bool> {
        if self.connection.registration_state != RegistrationState::Registered {
            self.unregistered_error()?;
            return Ok(false);
        }

        Ok(true)
    }

    pub fn validate_names_command(&mut self) -> io::Result<bool> {
        if self.connection.registration_state != RegistrationState::Registered {
            self.unregistered_error()?;
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
}