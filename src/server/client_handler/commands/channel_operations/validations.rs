use crate::server::client_handler::commands::{
    DISTRIBUTED_CHANNEL, INVITE_COMMAND, JOIN_COMMAND, KICK_COMMAND, MODE_COMMAND, PART_COMMAND,
    TOPIC_COMMAND,
};
use crate::server::client_handler::commands::{INVALID_CHARACTER, LOCAL_CHANNEL, MAX_CHANNELS};
use crate::server::client_handler::registration::RegistrationState;
use crate::server::client_handler::responses::errors::ErrorReply;
use crate::server::client_trait::Connection;

pub const ADD_MODE: char = '+';
pub const REMOVE_MODE: char = '-';

use super::ClientHandler;

impl<C: Connection> ClientHandler<C> {
    fn channel_name_is_valid(&self, channel: &str) -> bool {
        return ((channel.as_bytes()[0] == LOCAL_CHANNEL)
            || (channel.as_bytes()[0] == DISTRIBUTED_CHANNEL))
            && !channel.contains(INVALID_CHARACTER);
    }
    pub fn assert_modes_starts_correctly(&mut self, modes: &String) -> bool {
        modes.as_bytes()[0] == (ADD_MODE as u8) || modes.as_bytes()[0] == (REMOVE_MODE as u8)
    }

    pub fn assert_can_set_key(&mut self, channel: &str) -> Option<ErrorReply> {
        if self.database.get_channel_key(channel).is_some() {
            return Some(ErrorReply::KeySet467 {
                channel: channel.to_string(),
            });
        }
        None
    }

    /// Asserts client can join channel.
    /// Possible errors:
    ///     - Client is in too many channels.
    ///     - Channel name does not exist.
    ///     - User is already on channel.
    pub fn assert_can_join_channel(&self, channel: &str, nickname: &str) -> Option<ErrorReply> {
        let nickname = nickname.to_string();
        let channel = channel.to_string();

        let channels_for_nickname = self.database.get_channels_for_client(&nickname);
        if channels_for_nickname.len() == MAX_CHANNELS {
            return Some(ErrorReply::TooManyChannels405 { channel });
        }

        if !self.channel_name_is_valid(&channel) {
            return Some(ErrorReply::NoSuchChannel403 { channel });
        }

        if self.database.is_client_in_channel(&nickname, &channel) {
            return Some(ErrorReply::UserOnChannel443 { nickname, channel });
        }

        None
    }
    /// Asserts client can part channel.
    /// Possible errors:
    ///     - Channel name does not exist.
    ///     - User is not on channel.
    pub fn assert_can_part_channel(&self, channel: &str, nickname: &str) -> Option<ErrorReply> {
        let channel = channel.to_string();

        if !self.database.contains_channel(&channel) || !self.channel_name_is_valid(&channel) {
            return Some(ErrorReply::NoSuchChannel403 { channel });
        }

        if !self.database.is_client_in_channel(nickname, &channel) {
            return Some(ErrorReply::NotOnChannel442 { channel });
        }

        None
    }
    /// Asserts channel can be listed.
    pub fn can_list_channel(&self, channel: &str) -> bool {
        if self.database.channel_has_mode(channel, 's')
            && !self
                .database
                .is_client_in_channel(&self.registration.nickname().unwrap(), channel)
        {
            return false;
        }

        if self.database.contains_channel(channel) {
            return true;
        }

        false
    }

    /// Asserts invite command can be executed.
    /// Possible errors:
    ///     - Not enough parameters.
    ///     - Client is not registered.
    ///     - Invited client does not exist.
    ///     - Invited client is already on channel.
    ///     - Inviting client is not on channel.
    pub fn assert_invite_is_valid(&self, parameters: &Vec<String>) -> Option<ErrorReply> {
        if parameters.len() != 2 {
            let command = INVITE_COMMAND.to_string();
            return Some(ErrorReply::NeedMoreParameters461 { command });
        }

        if self.registration.state() != &RegistrationState::Registered {
            return Some(ErrorReply::UnregisteredClient);
        }

        let invited_client = parameters[0].to_string();
        let inviting_client = self.registration.nickname().unwrap();
        let channel = parameters[1].to_string();

        if !self.database.contains_client(&invited_client) {
            return Some(ErrorReply::NoSuchNickname401 {
                nickname: invited_client,
            });
        }

        if self.database.contains_channel(&channel) {
            if !self
                .database
                .is_client_in_channel(&inviting_client, &channel)
            {
                return Some(ErrorReply::NotOnChannel442 { channel });
            }
            if self
                .database
                .is_client_in_channel(&invited_client, &channel)
            {
                return Some(ErrorReply::UserOnChannel443 {
                    nickname: invited_client,
                    channel,
                });
            }
        }

        if self.database.channel_has_mode(&channel, 'i')
            && !self
                .database
                .is_channel_operator(&channel, &inviting_client)
        {
            return Some(ErrorReply::ChanOPrivIsNeeded482 { channel });
        }

        None
    }

    /// Asserts join command can be executed.
    /// Possible errors:
    ///     - Not enough parameters.
    ///     - Client is not registered.
    pub fn assert_join_is_valid(&self, parameters: &Vec<String>) -> Option<ErrorReply> {
        if parameters.is_empty() {
            let command = JOIN_COMMAND.to_string();
            return Some(ErrorReply::NeedMoreParameters461 { command });
        }

        self.assert_registration_is_valid()
    }
    /// Asserts part command can be executed.
    /// Possible errors:
    ///     - Not enough parameters.
    ///     - Client is not registered.
    pub fn assert_part_is_valid(&self, parameters: &Vec<String>) -> Option<ErrorReply> {
        if parameters.is_empty() {
            let command = PART_COMMAND.to_string();
            return Some(ErrorReply::NeedMoreParameters461 { command });
        }

        self.assert_registration_is_valid()
    }
    /// Asserts client is registered.
    pub fn assert_registration_is_valid(&self) -> Option<ErrorReply> {
        if self.registration.state() != &RegistrationState::Registered {
            return Some(ErrorReply::UnregisteredClient);
        }
        None
    }

    pub fn assert_topic_is_valid(&self, parameters: &Vec<String>) -> Option<ErrorReply> {
        if parameters.is_empty() {
            let command = TOPIC_COMMAND.to_string();
            return Some(ErrorReply::NeedMoreParameters461 { command });
        }
        if self.registration.state() != &RegistrationState::Registered {
            return Some(ErrorReply::UnregisteredClient);
        }

        let nickname = self.registration.nickname().unwrap();
        let channel = parameters[0].to_string();

        if !self.database.is_client_in_channel(&nickname, &channel) {
            return Some(ErrorReply::NotOnChannel442 { channel });
        }

        if self.database.channel_has_mode(&channel, 't')
            && !self
                .database
                .is_channel_operator(&channel, &self.registration.nickname().unwrap())
        {
            return Some(ErrorReply::ChanOPrivIsNeeded482 { channel });
        }

        None
    }

    pub fn assert_kick_is_valid(&self, parameters: &[String]) -> Option<ErrorReply> {
        if parameters.len() < 2 {
            let command = KICK_COMMAND.to_string();
            return Some(ErrorReply::NeedMoreParameters461 { command });
        }

        if self.registration.state() != &RegistrationState::Registered {
            return Some(ErrorReply::UnregisteredClient);
        }

        None
    }

    pub fn assert_can_kick_from_channel(&self, channel: &str) -> Option<ErrorReply> {
        if !self.database.contains_channel(channel) {
            let channel = channel.to_string();
            return Some(ErrorReply::NoSuchChannel403 { channel });
        }

        if !self
            .database
            .is_client_in_channel(&self.registration.nickname().unwrap(), channel)
        {
            let channel = channel.to_string();
            return Some(ErrorReply::NotOnChannel442 { channel });
        }

        if !self
            .database
            .is_channel_operator(channel, &self.registration.nickname().unwrap())
        {
            let channel = channel.to_string();
            return Some(ErrorReply::ChanopPrivilegesNeeded482 { channel });
        }

        None
    }

    pub fn assert_mode_is_valid(&self, parameters: &Vec<String>) -> Option<ErrorReply> {
        if parameters.is_empty() {
            let command = MODE_COMMAND.to_string();
            return Some(ErrorReply::NeedMoreParameters461 { command });
        }
        if self.registration.state() != &RegistrationState::Registered {
            return Some(ErrorReply::UnregisteredClient);
        }

        let channel = &parameters[0];
        let nickname = self.registration.nickname().unwrap();

        if !self.database.contains_channel(channel) {
            return Some(ErrorReply::NoSuchChannel403 {
                channel: channel.to_string(),
            });
        }

        if !self.database.is_client_in_channel(&nickname, channel) {
            return Some(ErrorReply::NotOnChannel442 {
                channel: channel.to_string(),
            });
        }

        // if !self.database.is_channel_operator(channel, &nickname) && parameters.len() > 1{
        //     return Some(ErrorReply::ChanOPrivIsNeeded482 {
        //         channel: channel.to_string(),
        //     });
        // }

        None
    }
}
