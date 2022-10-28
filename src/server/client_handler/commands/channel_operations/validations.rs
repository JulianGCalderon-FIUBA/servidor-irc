use crate::server::{
    client_handler::{
        commands::{DISTRIBUTED_CHANNEL, INVALID_CHARACTER, LOCAL_CHANNEL, MAX_CHANNELS},
        registration::RegistrationState,
        responses::errors::ErrorReply,
    },
    client_trait::ClientTrait,
};

use super::{ClientHandler, INVITE_COMMAND, JOIN_COMMAND, PART_COMMAND};
use std::io;
// use std::sync::mpsc::channel;

impl<T: ClientTrait> ClientHandler<T> {
    // GENERAL
    pub fn validate_nickname_exits(&mut self, nickname: &str) -> io::Result<bool> {
        if !self.database.contains_client(nickname) {
            self.no_such_nickname_error(nickname)?;
            return Ok(false);
        }

        Ok(true)
    }

    fn channel_name_is_valid(&mut self, channel: &str) -> bool {
        return ((channel.as_bytes()[0] == LOCAL_CHANNEL)
            || (channel.as_bytes()[0] == DISTRIBUTED_CHANNEL))
            && !channel.contains(INVALID_CHARACTER);
    }

    pub fn assert_can_join_channel(&mut self, channel: &str, nickname: &str) -> Option<ErrorReply> {
        let channels_for_nickname = self.database.get_channels_for_client(nickname);

        if channels_for_nickname.len() == MAX_CHANNELS {
            return Some(ErrorReply::TooManyChannels405 {
                channel: channel.to_string(),
            });
        }
        if !self.channel_name_is_valid(channel) {
            return Some(ErrorReply::NoSuchChannel403 {
                channel: channel.to_string(),
            });
        }
        if self.user_is_in_channel(channel, nickname) {
            return Some(ErrorReply::UserOnChannel443 {
                nickname: nickname.to_string(),
                channel: channel.to_string(),
            });
        }
        None
    }

    pub fn assert_can_part_channel(&mut self, channel: &str, nickname: &str) -> Option<ErrorReply> {
        if !self.database.contains_channel(channel) || !self.channel_name_is_valid(channel) {
            return Some(ErrorReply::NoSuchChannel403 {
                channel: channel.to_string(),
            });
        }
        let clients = self.database.get_clients(channel);
        if !clients.contains(&nickname.to_string()) {
            return Some(ErrorReply::NotOnChannel442 {
                channel: channel.to_string(),
            });
        }
        None
    }

    pub fn assert_can_list_channel(&mut self, channel: &str) -> bool {
        if !self.database.contains_channel(channel) || !self.channel_name_is_valid(channel) {
            return false;
        }
        true
    }

    pub fn user_is_in_channel(&mut self, channel: &str, nickname: &str) -> bool {
        self.database
            .get_clients(channel)
            .contains(&String::from(nickname))
    }

    // COMMANDS

    pub fn validate_invite_command(&mut self, parameters: &Vec<String>) -> io::Result<bool> {
        if parameters.len() != 2 {
            self.need_more_params_error(INVITE_COMMAND)?;
            return Ok(false);
        }
        if self.registration.state() != &RegistrationState::Registered {
            self.unregistered_error()?;
            return Ok(false);
        }
        Ok(true)
    }

    pub fn assert_join_is_valid(&mut self, parameters: &Vec<String>) -> Option<ErrorReply> {
        if parameters.is_empty() {
            return Some(ErrorReply::NeedMoreParameters461 {
                command: JOIN_COMMAND.to_string(),
            });
        }
        self.assert_registration_is_valid()
    }

    pub fn assert_part_is_valid(&mut self, parameters: &Vec<String>) -> Option<ErrorReply> {
        if parameters.is_empty() {
            return Some(ErrorReply::NeedMoreParameters461 {
                command: PART_COMMAND.to_string(),
            });
        }
        self.assert_registration_is_valid()
    }

    pub fn assert_registration_is_valid(&mut self) -> Option<ErrorReply> {
        if self.registration.state() != &RegistrationState::Registered {
            return Some(ErrorReply::UnregisteredClient);
        }
        None
    }
}
