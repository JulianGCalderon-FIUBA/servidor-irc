mod validations;

use crate::message::Message;

use super::ClientHandler;

use std::io::{self, Read, Write};

pub const INVITE_COMMAND: &str = "INVITE";
pub const JOIN_COMMAND: &str = "JOIN";
pub const LIST_COMMAND: &str = "LIST";
pub const NAMES_COMMAND: &str = "NAMES";
pub const PART_COMMAND: &str = "PART";

impl<T: Read + Write> ClientHandler<T> {
    pub fn invite_command(&mut self, parameters: Vec<String>) -> io::Result<()> {
        if !self.validate_invite_command(&parameters)? {
            return Ok(());
        }

        let nickname_client_to_invite = &parameters[0];
        let nickname_current_client = &(self.connection.nickname());
        let channel = &parameters[1];

        if !self.validate_nickname_exits(nickname_client_to_invite)? {
            return Ok(());
        }

        if self.validate_channel_exists(channel)? {
            if !self.validate_user_is_in_channel(channel, nickname_current_client)? {
                self.not_on_channel_error(channel)?;
                return Ok(());
            }
            if !self.validate_can_join_channel(channel, nickname_client_to_invite)? {
                self.user_on_channel_error(nickname_client_to_invite, channel)?;
                return Ok(());
            }
            // si el canal es invite only: validar que el usuario es operador del canal
        }

        let prefix = self.connection.nickname();

        let invitation_text: String =
            format!("{prefix} {INVITE_COMMAND} {nickname_client_to_invite} {channel}");
        self.send_message_to_client(
            nickname_client_to_invite,
            &Message::new(&invitation_text).unwrap(),
        );

        self.invite_reply(channel, nickname_client_to_invite)
    }

    pub fn join_command(&mut self, parameters: Vec<String>) -> io::Result<()> {
        if !self.validate_join_command(&parameters)? {
            return Ok(());
        }
        let nickname = self.connection.nickname();

        let channels = &parameters[0];
        //let keys = &parameters[1];

        for channel in channels.split(',') {
            if !self.validate_can_join_channel(channel, &nickname)? {
                continue;
            }
            self.database.add_client_to_channel(&nickname, channel);
            self.no_topic_reply(channel)?;
            self.names_reply(channel, self.database.get_clients(channel))?
        }

        Ok(())
    }

    pub fn list_command(&mut self, parameters: Vec<String>) -> io::Result<()> {
        if !self.validate_list_command()? {
            return Ok(());
        }

        let mut channels: Vec<String> = if parameters.is_empty() {
            let mut list = self.database.get_channels();
            list.sort();
            list
        } else {
            parameters[0]
                .split(',')
                .map(|string| string.to_string())
                .collect()
        };

        if channels.is_empty() {
            self.list_start_reply()?;
            return self.list_end_reply();
        }

        for (i, channel) in channels.clone().iter().enumerate() {
            if !self.validate_can_list_channel(channel)? {
                channels.remove(i);
                continue;
            }
        }
        self.list_reply(channels)
    }

    pub fn names_command(&mut self, parameters: Vec<String>) -> io::Result<()> {
        if !self.validate_names_command()? {
            return Ok(());
        }

        let channels: Vec<String> = if parameters.is_empty() {
            let mut list = self.database.get_channels();
            list.sort();
            list
        } else {
            parameters[0]
                .split(',')
                .map(|string| string.to_string())
                .collect()
        };

        if channels.is_empty() {
            return self.end_of_names_reply("");
        }

        for channel in channels {
            if self.database.contains_channel(&channel) {
                let clients = self.database.get_clients(&channel);
                self.names_reply(&channel, clients)?;
                if !parameters.is_empty() {
                    self.end_of_names_reply(&channel)?;
                }
            }
        }
        if parameters.is_empty() {
            self.end_of_names_reply("")?;
        }
        Ok(())
    }

    pub fn part_command(&mut self, parameters: Vec<String>) -> io::Result<()> {
        if !self.validate_part_command(&parameters)? {
            return Ok(());
        }

        let channels = &parameters[0];
        let nickname = self.connection.nickname();

        for channel in channels.split(',') {
            if !self.validate_can_part_channel(channel, &nickname)? {
                continue;
            }
            self.database.remove_client_of_channel(&nickname, channel);
            self.ok_reply()?;
        }
        Ok(())
    }
}
