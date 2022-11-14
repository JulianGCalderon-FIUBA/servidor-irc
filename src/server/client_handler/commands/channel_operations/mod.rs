/// This module contains useful functionalities when working with channels.
mod utils;
/// This module contains validations for channel operations.
mod validations;

mod mode_utils;

use std::io;

use crate::server::client_handler::responses::errors::ErrorReply;
use crate::server::client_handler::responses::notifications::Notification;
use crate::server::client_handler::responses::replies::CommandResponse;
use crate::server::client_trait::Connection;

pub const OPER_CONFIG: char = 'o';
pub const LIMIT_CONFIG: char = 'l';
pub const BAN_CONFIG: char = 'b';
pub const SPEAKING_ABILITY_CONFIG: char = 'v';
pub const KEY_CONFIG: char = 'k';

const VALID_MODES: [char; 5] = ['s', 'i', 't', 'n', 'm'];

use self::mode_utils::parse_modes;

use super::ClientHandler;

impl<C: Connection> ClientHandler<C> {
    /// Invites client to channel.
    pub fn invite_command(&mut self, parameters: Vec<String>) -> io::Result<()> {
        if let Some(error) = self.assert_invite_is_valid(&parameters) {
            return self.send_response_for_error(error);
        }

        let invited_client = &parameters[0];
        let channel = parameters[1].to_string();
        let inviting_client = self.registration.nickname().unwrap();

        let invitation = Notification::Invite {
            inviting_client: inviting_client.clone(),
            invited_client: invited_client.clone(),
            channel: channel.clone(),
        };

        if self
            .send_message_to_client(invited_client, &invitation.to_string())
            .is_err()
        {
            self.send_response_for_error(ErrorReply::NoSuchNickname401 {
                nickname: invited_client.clone(),
            })?
        }

        self.send_response_for_reply(CommandResponse::Inviting341 {
            nickname: inviting_client,
            channel,
        })
    }

    /// Joins specific channel.
    pub fn join_command(&mut self, parameters: Vec<String>) -> io::Result<()> {
        if let Some(error) = self.assert_join_is_valid(&parameters) {
            return self.send_response_for_error(error);
        }
        let nickname = self.registration.nickname().unwrap();

        let channels = &parameters[0];
        //let keys = &parameters[1];

        for channel in channels.split(',') {
            if let Some(error) = self.assert_can_join_channel(channel, &nickname) {
                self.send_response_for_error(error)?;
                continue;
            }

            let notification = Notification::Join {
                nickname: self.registration.nickname().unwrap(),
                channel: channel.to_string(),
            };
            self.send_message_to_channel(channel, &notification.to_string());

            self.database.add_client_to_channel(&nickname, channel);

            self.send_topic_reply(channel.to_string())?;

            self.send_response_for_reply(CommandResponse::NameReply353 {
                channel: channel.to_string(),
                clients: self.database.get_clients_for_channel(channel),
            })?;
        }

        Ok(())
    }

    /// Lists all channels and their information.
    pub fn list_command(&mut self, parameters: Vec<String>) -> io::Result<()> {
        if let Some(error) = self.assert_registration_is_valid() {
            return self.send_response_for_error(error);
        }

        let channels = self.get_channels_for_query(parameters.get(0));

        self.send_response_for_reply(CommandResponse::ListStart321)?;

        for channel in channels {
            if self.can_list_channel(&channel) {
                let topic = match self.database.get_topic_for_channel(&channel) {
                    Some(topic) => topic,
                    None => "No topic set".to_string(),
                };
                self.send_response_for_reply(CommandResponse::List322 { channel, topic })?;
            }
        }
        self.send_response_for_reply(CommandResponse::ListEnd323)
    }

    /// Lists all names in specific channel.
    pub fn names_command(&mut self, parameters: Vec<String>) -> io::Result<()> {
        if let Some(error) = self.assert_registration_is_valid() {
            return self.send_response_for_error(error);
        }

        let channels = self.get_channels_for_query(parameters.get(0));

        for channel in channels {
            if !self.database.contains_channel(&channel) {
                continue;
            }

            let clients = self.database.get_clients_for_channel(&channel);
            self.send_response_for_reply(CommandResponse::NameReply353 {
                channel: channel.clone(),
                clients,
            })?;

            if !parameters.is_empty() {
                self.send_response_for_reply(CommandResponse::EndOfNames366 { channel })?
            }
        }

        if parameters.is_empty() {
            return self.send_response_for_reply(CommandResponse::EndOfNames366 {
                channel: "".to_string(),
            });
        }

        Ok(())
    }

    /// Parts specific channel.
    pub fn part_command(&mut self, parameters: Vec<String>) -> io::Result<()> {
        if let Some(error) = self.assert_part_is_valid(&parameters) {
            return self.send_response_for_error(error);
        }

        let channels = &parameters[0];
        let nickname = self.registration.nickname().unwrap();

        for channel in channels.split(',') {
            if let Some(error) = self.assert_can_part_channel(channel, &nickname) {
                self.send_response_for_error(error)?;
                continue;
            }
            let notification = Notification::Part {
                nickname: self.registration.nickname().unwrap(),
                channel: channel.to_string(),
            };
            self.send_message_to_channel(channel, &notification.to_string());
            self.database.remove_client_from_channel(&nickname, channel);
        }
        Ok(())
    }

    pub fn topic_command(&mut self, parameters: Vec<String>) -> io::Result<()> {
        if let Some(error) = self.assert_topic_is_valid(&parameters) {
            return self.send_response_for_error(error);
        }

        let channel = &parameters[0];

        if parameters.len() > 1 {
            let topic = &parameters[1];
            self.database.set_channel_topic(channel, topic);
        } else {
            self.send_topic_reply(channel.to_string())?;
        }

        Ok(())
    }

    pub fn kick_command(
        &mut self,
        parameters: Vec<String>,
        trailing: Option<String>,
    ) -> io::Result<()> {
        if let Some(error) = self.assert_kick_is_valid(&parameters) {
            return self.send_response_for_error(error);
        }

        let channel = parameters[0].split(',');
        let nickname = parameters[1].split(',');

        for (channel, nickname) in channel.zip(nickname) {
            if let Some(error) = self.assert_can_kick_from_channel(channel) {
                self.send_response_for_error(error)?;
            } else {
                self.kick_client_from_channel(nickname, channel, &trailing);
            }
        }

        Ok(())
    }

    pub fn mode_command(&mut self, parameters: Vec<String>) -> io::Result<()> {
        if let Some(error) = self.assert_mode_is_valid(&parameters) {
            return self.send_response_for_error(error);
        }
        if !self.assert_modes_starts_correctly(&parameters[1]) {
            return Ok(());
        }

        // let channel = &parameters[0];

        // if parameters.len() == 1 {
        //     let modes = self.database.get_channel_modes(channel);
        //     for mode in modes {
        //         let params: Option<Vec<String>> = match mode {
        //             OPER_CONFIG => self.database.get_channel_operators(channel),
        //             LIMIT_CONFIG => self.database.get_channel_limit(channel),
        //             BAN_CONFIG => Some(self.database.get_channel_banmask(channel)),
        //             SPEAKING_ABILITY_CONFIG => self.get_channel_speakers(channel),
        //             _ => None,
        //         };
        //         self.send_response_for_reply(CommandResponse::ChannelModeIs324 {
        //             channel: channel.to_string(),
        //             mode,
        //             mode_params: params,
        //         })?;
        //     }
        // }

        let modes: Vec<char> = parameters[1].chars().collect();

        let (add, remove) = parse_modes(modes);

        self.add_modes(add, parameters.clone())?;
        self.remove_modes(remove, parameters)?;

        Ok(())
    }
}
