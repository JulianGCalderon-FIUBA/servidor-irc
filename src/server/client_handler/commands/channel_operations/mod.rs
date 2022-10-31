mod validations;

use crate::{
    message::Message,
    server::{client_handler::responses::replies::CommandResponse, client_trait::ClientTrait},
};

use super::ClientHandler;

use std::io;

pub const INVITE_COMMAND: &str = "INVITE";
pub const JOIN_COMMAND: &str = "JOIN";
pub const LIST_COMMAND: &str = "LIST";
pub const NAMES_COMMAND: &str = "NAMES";
pub const PART_COMMAND: &str = "PART";

impl<T: ClientTrait> ClientHandler<T> {
    pub fn invite_command(&mut self, parameters: Vec<String>) -> io::Result<()> {
        if let Some(error) = self.assert_invite_is_valid(&parameters) {
            return self.send_response_for_error(error);
        }

        let invited_client = &parameters[0];
        let inviting_client = self.registration.nickname().unwrap();
        let channel = parameters[1].to_string();

        let prefix = self.registration.nickname().unwrap();

        let invitation_text = format!(":{prefix} {INVITE_COMMAND} {invited_client} {channel}");
        let message = Message::new(&invitation_text).unwrap();
        self.send_message_to_client(invited_client, &message);

        self.send_response_for_reply(CommandResponse::Inviting341 {
            nickname: inviting_client,
            channel,
        })
    }

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

            self.send_response_for_reply(CommandResponse::NoTopic331 {
                channel: channel.to_string(),
            })?;
            self.send_response_for_reply(CommandResponse::NameReply353 {
                channel: channel.to_string(),
                clients: self.database.get_clients(channel),
            })?;
        }

        Ok(())
    }

    pub fn list_command(&mut self, parameters: Vec<String>) -> io::Result<()> {
        if let Some(error) = self.assert_registration_is_valid() {
            return self.send_response_for_error(error);
        }

        let channels = self.get_channels_for_query(parameters.get(0));

        self.send_response_for_reply(CommandResponse::ListStart321)?;

        for channel in channels {
            if self.can_list_channel(&channel) {
                self.send_response_for_reply(CommandResponse::List322 { channel })?;
            }
        }
        self.send_response_for_reply(CommandResponse::ListEnd323)
    }

    fn get_channels_for_query(&mut self, channels: Option<&String>) -> Vec<String> {
        if channels.is_none() {
            let mut channels = self.database.get_channels();
            channels.sort();
            return channels;
        }

        channels
            .unwrap()
            .split(',')
            .map(|string| string.to_string())
            .collect()
    }

    pub fn names_command(&mut self, parameters: Vec<String>) -> io::Result<()> {
        if let Some(error) = self.assert_registration_is_valid() {
            return self.send_response_for_error(error);
        }

        let channels = self.get_channels_for_query(parameters.get(0));

        for channel in channels {
            if !self.database.contains_channel(&channel) {
                continue;
            }

            let clients = self.database.get_clients(&channel);
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
            self.database.remove_client_of_channel(&nickname, channel);
            self.send_response_for_reply(CommandResponse::Ok200)?
        }
        Ok(())
    }
}
