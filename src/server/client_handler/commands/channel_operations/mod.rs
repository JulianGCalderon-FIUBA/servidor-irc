/// This module contains useful functionalities when working with channels.
mod utils;
/// This module contains validations for channel operations.
mod validations;

use crate::server::client_handler::responses::errors::ErrorReply;
use crate::server::client_handler::responses::notifications::Notification;
use crate::server::client_handler::responses::replies::CommandResponse;
use crate::server::client_trait::ClientTrait;

use super::ClientHandler;

use std::io;

impl<T: ClientTrait> ClientHandler<T> {
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

            self.send_response_for_reply(CommandResponse::NoTopic331 {
                channel: channel.to_string(),
            })?;
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
                self.send_response_for_reply(CommandResponse::List322 { channel })?;
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
}
