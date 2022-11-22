use std::io;

use crate::server::consts::modes::PRIVATE;
use crate::server::responses::Notification;
use crate::server::{
    connection::Connection,
    connection_handler::{
        client_handler::ClientHandler, connection_handler_trait::ConnectionHandlerUtils,
    },
};

use crate::server::data_structures::*;

impl<C: Connection> ClientHandler<C> {
    pub(super) fn send_join_response(&mut self, channel: &str) -> io::Result<()> {
        self.send_topic_response(channel.to_string())?;

        let clients = self.database.get_clients_for_channel(channel);
        self.stream.send_name_reply(channel, &clients)
    }

    pub(super) fn send_whois_response(&mut self, client_info: ClientInfo) -> io::Result<()> {
        let nickname = client_info.nickname.clone();
        let servername = client_info.servername.clone();
        let serverinfo = "todo: serverinfo";

        self.stream.send_whois_user(&client_info)?;

        self.send_whois_server_response(&nickname, &servername, serverinfo)?;
        self.send_whois_operator_response(&nickname)?;
        self.send_whois_channels_response(&nickname)?;

        self.stream.send_end_of_whois(&client_info.nickname)?;

        Ok(())
    }

    fn send_whois_channels_response(&mut self, nickname: &str) -> Result<(), io::Error> {
        let mut channels = self.database.get_channels_for_client(nickname);
        if !channels.is_empty() {
            self.append_channel_role(&mut channels, nickname);
            self.stream.send_whois_channel(nickname, &channels)?;
        };

        Ok(())
    }

    fn send_whois_operator_response(&mut self, nickname: &str) -> Result<(), io::Error> {
        if self.database.is_server_operator(nickname) {
            self.stream.send_whois_operator(nickname)?;
        };
        Ok(())
    }

    fn send_whois_server_response(
        &mut self,
        nickname: &str,
        servername: &str,
        serverinfo: &str,
    ) -> Result<(), io::Error> {
        self.stream
            .send_whois_server(nickname, servername, serverinfo)
    }

    pub(super) fn send_banlist_response(&mut self, channel: &str) -> io::Result<()> {
        let banmasks = self.database.get_channel_banmask(channel);
        for banmask in banmasks {
            self.stream.send_banlist(channel, &banmask)?;
        }

        self.stream.send_end_of_banlist(channel)
    }

    pub(super) fn send_topic_response(&mut self, channel: String) -> io::Result<()> {
        match self.database.get_topic_for_channel(&channel) {
            Some(topic) => self.stream.send_topic(&channel, &topic),
            None => self.stream.send_no_topic(&channel),
        }
    }

    pub(super) fn send_whoreply_response(&mut self, client_info: ClientInfo) -> io::Result<()> {
        let channel = self
            .database
            .get_channels_for_client(&client_info.nickname)
            .get(0)
            .map(|string| string.to_owned());

        self.stream.send_whoreply_response(&channel, &client_info)
    }

    pub(super) fn send_list_response(&mut self, channel: String) -> io::Result<()> {
        let topic = self
            .database
            .get_topic_for_channel(&channel)
            .unwrap_or_else(|| "No topic set".to_string());

        let prv =
            self.database.channel_has_mode(&channel, PRIVATE) && !self.is_in_channel(&channel);

        self.stream.send_list(channel, topic, prv)
    }

    pub(super) fn send_invite_response(
        &mut self,
        inviting_client: String,
        channel: String,
    ) -> Result<(), io::Error> {
        self.stream.send_inviting(inviting_client, channel)
    }

    pub(super) fn send_names_response(&mut self, channel: &str) -> Result<(), io::Error> {
        let clients = self.database.get_clients_for_channel(channel);
        self.stream.send_name_reply(channel, &clients)
    }

    pub(super) fn send_end_of_names_response(&mut self, channel: &str) -> Result<(), io::Error> {
        self.stream.send_end_of_names(channel)
    }
    pub(super) fn send_quit_response(&mut self, message: &str) -> io::Result<()> {
        self.stream.send_quit(message)
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

    pub(super) fn send_away_response(
        &mut self,
        client: &str,
        message: &str,
    ) -> Result<(), io::Error> {
        self.stream.send_away(client, message)
    }

    pub(super) fn send_kick_notification(
        &mut self,
        channel: &str,
        nickname: &str,
        comment: &Option<String>,
    ) {
        let notification = Notification::Kick {
            kicker: self.nickname.clone(),
            channel: channel.to_string(),
            kicked: nickname.to_string(),
            comment: comment.clone(),
        };
        self.send_message_to_channel(&notification, channel);
    }

    pub(super) fn send_privmsg_notification(
        &mut self,
        target: &str,
        content: &str,
    ) -> Result<(), io::Error> {
        let nickname = self.nickname.clone();
        let notification = Notification::Privmsg {
            sender: nickname,
            target: target.to_string(),
            message: content.to_owned(),
        };
        self.send_message_to_target(&notification, target)
    }

    pub(super) fn send_notice_notification(
        &mut self,
        target: &str,
        content: &str,
    ) -> Result<(), io::Error> {
        let nickname = self.nickname.clone();
        let notification = Notification::Notice {
            sender: nickname,
            target: target.to_string(),
            message: content.to_owned(),
        };
        self.send_message_to_target(&notification, target)
    }
}
