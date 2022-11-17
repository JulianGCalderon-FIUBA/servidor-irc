use std::io;

use crate::server::{
    connection::Connection,
    connection_handler::{
        client_handler::ClientHandler,
        connection_handler_trait::ConnectionHandlerUtils,
        consts::modes::PRIVATE,
        responses::{CommandResponse, Notification},
    },
    database::ClientInfo,
};

impl<C: Connection> ClientHandler<C> {
    pub(super) fn send_join_response(&mut self, channel: &str) -> io::Result<()> {
        self.send_topic_response(channel.to_string())?;

        let name_reply = &CommandResponse::NameReply353 {
            channel: channel.to_string(),
            clients: self.database.get_clients_for_channel(channel),
        };

        self.send_response(name_reply)
    }

    pub(super) fn send_whois_response(&mut self, client_info: ClientInfo) -> io::Result<()> {
        let nickname = client_info.nickname.clone();
        let server = self.servername.to_string();

        self.send_response(&CommandResponse::WhoisUser311 { client_info })?;
        self.send_response(&CommandResponse::WhoisServer312 {
            nickname: nickname.clone(),
            server,
            server_info: "Lemon pie server".to_string(),
        })?;

        if self.database.is_server_operator(&nickname) {
            self.send_response(&CommandResponse::WhoisOperator313 {
                nickname: nickname.clone(),
            })?;
        }

        let mut channels = self.database.get_channels_for_client(&nickname);
        if !channels.is_empty() {
            self.append_channel_role(&mut channels, &nickname);
            self.send_response(&CommandResponse::WhoisChannels319 {
                nickname: nickname.clone(),
                channels,
            })?;
        }
        self.send_response(&CommandResponse::EndOfWhois318 { nickname })?;

        Ok(())
    }

    pub(super) fn send_banlist_response(&mut self, channel: &str) -> io::Result<()> {
        let bans = self.database.get_channel_banmask(channel);
        for b in bans {
            self.send_response(&CommandResponse::BanList367 {
                channel: channel.to_string(),
                banmask: b,
            })?;
        }
        self.send_response(&CommandResponse::EndOfBanList368 {
            channel: channel.to_string(),
        })?;
        Ok(())
    }

    pub(super) fn send_topic_response(&mut self, channel: String) -> io::Result<()> {
        match self.database.get_topic_for_channel(&channel) {
            Some(topic) => self.send_response(&CommandResponse::Topic332 { channel, topic })?,
            None => self.send_response(&CommandResponse::NoTopic331 { channel })?,
        };
        Ok(())
    }

    pub(super) fn send_whoreply_response(&mut self, client_info: ClientInfo) -> io::Result<()> {
        let channel = self
            .database
            .get_channels_for_client(&client_info.nickname)
            .get(0)
            .map(|string| string.to_owned());

        self.send_response(&CommandResponse::WhoReply352 {
            channel,
            client_info,
        })
    }

    pub(super) fn send_list_response(&mut self, channel: String) -> io::Result<()> {
        let topic = match self.database.get_topic_for_channel(&channel) {
            Some(topic) => topic,
            None => "No topic set".to_string(),
        };
        let prv = self.database.channel_has_mode(&channel, PRIVATE)
            && !self.database.is_client_in_channel(&self.nickname, &channel);
        self.send_response(&CommandResponse::List322 {
            channel,
            prv,
            topic,
        })?;
        Ok(())
    }
    pub(super) fn send_invite_response(
        &mut self,
        inviting_client: String,
        channel: String,
    ) -> Result<(), io::Error> {
        let invite_response = CommandResponse::Inviting341 {
            nickname: inviting_client,
            channel,
        };
        self.send_response(&invite_response)?;
        Ok(())
    }

    pub(super) fn send_names_response(&mut self, channel: &str) -> Result<(), io::Error> {
        let clients = self.database.get_clients_for_channel(channel);
        let name_reply = CommandResponse::NameReply353 {
            channel: channel.to_string(),
            clients,
        };
        self.send_response(&name_reply)?;
        Ok(())
    }

    pub(super) fn send_end_of_names_response(&mut self, channel: &str) -> Result<(), io::Error> {
        let end_of_names = CommandResponse::EndOfNames366 {
            channel: channel.to_string(),
        };
        self.send_response(&end_of_names)
    }
    pub(super) fn send_quit_response(&mut self, message: &str) -> io::Result<()> {
        let notification = Notification::Quit {
            message: message.to_string(),
        };
        self.send_response(&notification)
    }

    pub(super) fn send_join_notification(&mut self, channel: &str) {
        let notification = Notification::Join {
            nickname: self.nickname.clone(),
            channel: channel.to_string(),
        };
        self.send_message_to_channel(&notification, channel);
    }

    pub(super) fn send_part_notification(&mut self, channel: &str) {
        let notification = Notification::Part {
            nickname: self.nickname.clone(),
            channel: channel.to_string(),
        };
        self.send_message_to_channel(&notification, channel);
    }

    pub(super) fn send_invite_notification(
        &mut self,
        invited_client: String,
        channel: &str,
    ) -> Result<(), io::Error> {
        let invitation = Notification::Invite {
            inviting_client: self.nickname.clone(),
            invited_client: invited_client.clone(),
            channel: channel.to_string(),
        };
        self.send_message_to_client(&invitation, &invited_client)
    }

    pub(super) fn send_quit_notification(&mut self, message: &str) {
        let notification = Notification::Quit {
            message: message.to_string(),
        };
        let channels = self.database.get_channels_for_client(&self.nickname);
        for channel in channels {
            self.send_message_to_channel(&channel, &notification.to_string());
        }
    }
}
