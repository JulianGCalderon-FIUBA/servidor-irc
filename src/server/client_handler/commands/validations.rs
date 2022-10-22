use super::super::connection_info::RegistrationState;
use super::ClientHandler;
use std::io;
// use std::sync::mpsc::channel;

use super::JOIN_COMMAND;
use super::PART_COMMAND;
use super::PASS_COMMAND;
use super::PRIVMSG_COMMAND;
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
            if self.connection.registration_state == RegistrationState::Registered {
                self.nickname_in_use_response()?;
            } else {
                self.nickname_collision_response()?;
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

        if self.connection.registration_state != RegistrationState::NicknameSent {
            self.no_nickname_error()?;
            return Ok(false);
        }

        Ok(true)
    }

    pub fn validate_privmsg_command(
        &mut self,
        parameters: &Vec<String>,
        trailing: &Option<String>,
    ) -> io::Result<bool> {
        if parameters.is_empty() {
            self.no_recipient_error(PRIVMSG_COMMAND)?;
            return Ok(false);
        }

        if trailing.is_none() {
            self.no_text_to_send_error()?;
            return Ok(false);
        }

        if !self.validate_targets(parameters)? {
            return Ok(false);
        };

        Ok(true)
    }

    pub fn validate_targets(&mut self, parameters: &[String]) -> io::Result<bool> {
        let mut valid = true;
        let targets = &parameters[0];
        for target in targets.split(',') {
            let is_client = self.database.contains_client(target);
            let is_channel = self.database.get_channels().contains(&target.to_string());

            if !(is_client || is_channel) {
                self.no_such_nick_error(target)?;
                valid = false;
            }

            if is_channel {
                let clients = self.database.get_clients(target);
                if !clients.contains(self.connection.nickname.as_ref().unwrap()) {
                    self.cannot_send_to_chan_error(target)?;
                    valid = false;
                }
            }
        }

        Ok(valid)
    }

    pub fn validate_channel_exists(&mut self, channel: &str) -> io::Result<bool> {
        let channels_database = self.database.get_channels();
        if !channels_database.contains(&channel.to_string()) {
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

    pub fn validate_list_command(&mut self) -> io::Result<bool> {
        if self.connection.registration_state != RegistrationState::Registered {
            self.unregistered_error()?;
            return Ok(false);
        }

        Ok(true)
    }
}
