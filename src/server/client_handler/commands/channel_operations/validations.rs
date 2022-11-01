use crate::server::{
    client_handler::{
        commands::{DISTRIBUTED_CHANNEL, INVALID_CHARACTER, LOCAL_CHANNEL, MAX_CHANNELS},
        registration::RegistrationState,
        responses::errors::ErrorReply,
    },
    client_trait::ClientTrait,
};

use super::{ClientHandler, INVITE_COMMAND, JOIN_COMMAND, PART_COMMAND};

// use std::sync::mpsc::channel;

impl<T: ClientTrait> ClientHandler<T> {
    // GENERAL

    fn channel_name_is_valid(&self, channel: &str) -> bool {
        return ((channel.as_bytes()[0] == LOCAL_CHANNEL)
            || (channel.as_bytes()[0] == DISTRIBUTED_CHANNEL))
            && !channel.contains(INVALID_CHARACTER);
    }

    pub fn assert_can_join_channel(&self, channel: &str, nickname: &str) -> Option<ErrorReply> {
        let nickname = nickname.to_string();
        let channel = channel.to_string();

        let channels_for_nickname = self.get_channels_for_client(&nickname);
        if channels_for_nickname.len() == MAX_CHANNELS {
            return Some(ErrorReply::TooManyChannels405 { channel });
        }

        if !self.channel_name_is_valid(&channel) {
            return Some(ErrorReply::NoSuchChannel403 { channel });
        }

        if self.user_is_in_channel(&channel, &nickname) {
            return Some(ErrorReply::UserOnChannel443 { nickname, channel });
        }

        None
    }

    pub fn assert_can_part_channel(&self, channel: &str, nickname: &str) -> Option<ErrorReply> {
        let channel = channel.to_string();

        if !self.contains_channel(&channel) || !self.channel_name_is_valid(&channel) {
            return Some(ErrorReply::NoSuchChannel403 { channel });
        }

        let clients = self.get_clients_for_channel(&channel);
        if !clients.contains(&nickname.to_string()) {
            return Some(ErrorReply::NotOnChannel442 { channel });
        }

        None
    }

    pub fn can_list_channel(&self, channel: &str) -> bool {
        self.contains_channel(channel) && self.channel_name_is_valid(channel)
    }

    pub fn user_is_in_channel(&self, channel: &str, nickname: &str) -> bool {
        self.get_clients_for_channel(channel)
            .contains(&String::from(nickname))
    }

    // COMMANDS

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

        if !self.contains_client(&invited_client) {
            return Some(ErrorReply::NoSuchNickname401 {
                nickname: invited_client,
            });
        }

        if self.contains_channel(&channel) {
            if !self.user_is_in_channel(&channel, &inviting_client) {
                return Some(ErrorReply::NotOnChannel442 { channel });
            }
            if self.user_is_in_channel(&channel, &invited_client) {
                return Some(ErrorReply::UserOnChannel443 {
                    nickname: invited_client,
                    channel,
                });
            }
        }
        None
    }

    pub fn assert_join_is_valid(&self, parameters: &Vec<String>) -> Option<ErrorReply> {
        if parameters.is_empty() {
            let command = JOIN_COMMAND.to_string();
            return Some(ErrorReply::NeedMoreParameters461 { command });
        }

        self.assert_registration_is_valid()
    }

    pub fn assert_part_is_valid(&self, parameters: &Vec<String>) -> Option<ErrorReply> {
        if parameters.is_empty() {
            return Some(ErrorReply::NeedMoreParameters461 {
                command: PART_COMMAND.to_string(),
            });
        }

        self.assert_registration_is_valid()
    }

    pub fn assert_registration_is_valid(&self) -> Option<ErrorReply> {
        if self.registration.state() != &RegistrationState::Registered {
            return Some(ErrorReply::UnregisteredClient);
        }
        None
    }
}
