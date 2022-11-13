/// This module contains useful functionalities when working with channels.
mod utils;
/// This module contains validations for channel operations.
mod validations;

use std::io;

use crate::server::client_handler::responses::errors::ErrorReply;
use crate::server::client_handler::responses::notifications::Notification;
use crate::server::client_handler::responses::replies::CommandResponse;
use crate::server::client_trait::Connection;

use self::validations::{ADD_MODE, REMOVE_MODE};

pub const OPER_CONFIG: char = 'o';
pub const LIMIT_CONFIG: char = 'l';
pub const BAN_CONFIG: char = 'b';
pub const SPEAKING_ABILITY_CONFIG: char = 'v';
pub const KEY_CONFIG: char = 'k';

const VALID_MODES: [char; 5] = ['s', 'i', 't', 'n', 'm'];

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

    pub fn mode_command(&mut self, parameters: Vec<String>) -> io::Result<()> {
        if let Some(error) = self.assert_mode_is_valid(&parameters) {
            return self.send_response_for_error(error);
        }
        if !self.assert_modes_starts_correctly(parameters[1].clone()) {
            return Ok(());
        }
        let channel = &parameters[0];
        let modes: Vec<char> = parameters[1].chars().collect();

        let (add, remove) = parse_modes(modes);

        for mode in add {
            match mode {
                OPER_CONFIG => {
                    if let Some(error) = self.assert_enough_parameters(&parameters) {
                        self.send_response_for_error(error)?;
                        continue;
                    }
                    for nickname in parameters[2].split(',') {
                        self.database.add_channop(channel, nickname);
                    }
                }
                LIMIT_CONFIG => {
                    if let Some(error) = self.assert_enough_parameters(&parameters) {
                        self.send_response_for_error(error)?;
                        continue;
                    }

                    if let Ok(limit) = parameters[2].parse::<isize>() {
                        self.database.set_channel_limit(channel, limit);
                    }
                }
                BAN_CONFIG => {
                    if let Some(error) = self.assert_enough_parameters(&parameters) {
                        self.send_response_for_error(error)?;
                        continue;
                    }
                    for banmask in parameters[2].split(',') {
                        self.database.set_channel_banmask(channel, banmask)
                    }
                }
                SPEAKING_ABILITY_CONFIG => {
                    if let Some(error) = self.assert_enough_parameters(&parameters) {
                        self.send_response_for_error(error)?;
                        continue;
                    }
                    for nickname in parameters[2].split(',') {
                        self.database.add_speaker(channel, nickname);
                    }
                }
                KEY_CONFIG => {
                    if let Some(error) = self.assert_enough_parameters(&parameters) {
                        self.send_response_for_error(error)?;
                        continue;
                    }
                    if let Some(error) = self.assert_can_set_key(channel) {
                        self.send_response_for_error(error)?;
                        continue;
                    }
                    let key = &parameters[2];
                    self.database.set_channel_key(channel, key)
                }
                mode if VALID_MODES.contains(&mode) => {
                    self.database.set_channel_mode(channel, mode)
                }
                mode => self.send_response_for_error(ErrorReply::UnknownMode472 { mode })?,
            }
        }
        for mode in remove {
            match mode {
                OPER_CONFIG => {
                    if let Some(error) = self.assert_enough_parameters(&parameters) {
                        self.send_response_for_error(error)?;
                        continue;
                    }
                    for nickname in parameters[2].split(',') {
                        self.database.remove_channop(channel, nickname);
                    }
                }
                LIMIT_CONFIG => {}
                BAN_CONFIG => {}
                SPEAKING_ABILITY_CONFIG => {
                    if let Some(error) = self.assert_enough_parameters(&parameters) {
                        self.send_response_for_error(error)?;
                        continue;
                    }
                    for nickname in parameters[2].split(',') {
                        self.database.remove_speaker(channel, nickname);
                    }
                }
                KEY_CONFIG => {}
                mode if VALID_MODES.contains(&mode) => {
                    self.database.unset_channel_mode(channel, mode)
                }
                mode => self.send_response_for_error(ErrorReply::UnknownMode472 { mode })?,
            }
        }

        Ok(())
    }
}

fn parse_modes(modes: Vec<char>) -> (Vec<char>, Vec<char>) {
    let mut add_modes: Vec<char> = vec![];
    let mut remove_modes: Vec<char> = vec![];
    let mut add: bool = false;
    for char in modes {
        match char {
            ADD_MODE => {
                add = true;
                continue;
            }
            REMOVE_MODE => {
                add = false;
                continue;
            }
            char => {
                if add {
                    add_modes.push(char);
                } else {
                    remove_modes.push(char);
                }
            }
        }
    }
    (add_modes, remove_modes)
}
