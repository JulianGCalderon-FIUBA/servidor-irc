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
        let nickname_client_to_invite = &parameters[0];
        let nickname_current_client = &(self.registration.nickname().unwrap());
        let channel = &parameters[1];

        let prefix = self.registration.nickname().unwrap();

        let invitation_text: String =
            format!(":{prefix} {INVITE_COMMAND} {nickname_client_to_invite} {channel}");
        self.send_message_to_client(
            nickname_client_to_invite,
            &Message::new(&invitation_text).unwrap(),
        );

        self.send_response_for_reply(CommandResponse::Inviting341 {
            channel: channel.to_string(),
            nickname: nickname_current_client.to_string(),
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
        self.send_response_for_reply(CommandResponse::ListStart321)?;

        for channel in channels {
            if self.assert_can_list_channel(&channel) {
                self.send_response_for_reply(CommandResponse::List322 { channel })?;
            }
        }
        self.send_response_for_reply(CommandResponse::ListEnd323)
    }

    pub fn names_command(&mut self, parameters: Vec<String>) -> io::Result<()> {
        if let Some(error) = self.assert_registration_is_valid() {
            return self.send_response_for_error(error);
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

        for channel in channels {
            if self.database.contains_channel(&channel) {
                let clients = self.database.get_clients(&channel);
                self.send_response_for_reply(CommandResponse::NameReply353 {
                    channel: channel.clone(),
                    clients,
                })?;

                if !parameters.is_empty() {
                    self.send_response_for_reply(CommandResponse::EndOfNames366 { channel })?
                }
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
