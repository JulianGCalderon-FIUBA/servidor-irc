use std::io;

use crate::server::consts::modes::{
    ChannelFlag, SET_BANMASK, SET_KEY, SET_OPERATOR, SET_SPEAKER, SET_USER_LIMIT,
};
use crate::server::responses::{CommandResponse, Notification};
use crate::server::{
    connection::Connection,
    connection_handler::{
        client_handler::ClientHandler, connection_handler_trait::ConnectionHandlerUtils,
    },
};

use crate::server::data_structures::*;

use super::booleans::is_distributed_channel;

impl<C: Connection> ClientHandler<C> {
    pub(super) fn send_join_response(&mut self, channel: &str) -> io::Result<()> {
        self.send_topic_response(channel)?;

        let clients = self.database.get_clients_for_channel(channel);
        self.stream
            .send(&CommandResponse::name_reply(channel, &clients))
    }

    pub(super) fn send_whois_response(&mut self, client_info: ClientInfo) -> io::Result<()> {
        let nickname = &client_info.nickname();
        let servername = &client_info.servername;
        let serverinfo = "serverinfo"; // todo

        self.stream
            .send(&CommandResponse::whois_user(&client_info))?;

        self.stream.send(&CommandResponse::whois_server(
            nickname, servername, serverinfo,
        ))?;

        self.send_whois_operator_response(nickname)?;
        self.send_whois_channels_response(nickname)?;

        self.stream.send(&CommandResponse::end_of_whois(nickname))?;

        Ok(())
    }

    fn send_whois_channels_response(&mut self, nickname: &str) -> Result<(), io::Error> {
        let mut channels = self.database.get_channels_for_client(nickname);
        if !channels.is_empty() {
            self.append_channel_role(&mut channels, nickname);
            self.stream
                .send(&CommandResponse::whois_channel(nickname, &channels))?;
        };

        Ok(())
    }

    fn send_whois_operator_response(&mut self, nickname: &str) -> Result<(), io::Error> {
        if self.database.is_server_operator(nickname) {
            self.stream
                .send(&CommandResponse::whois_operator(nickname))?;
        };
        Ok(())
    }

    pub(super) fn send_banlist_response(&mut self, channel: &str) -> io::Result<()> {
        let banmasks = self.database.get_channel_banmask(channel);
        for banmask in banmasks {
            self.stream
                .send(&CommandResponse::banlist(channel, &banmask))?;
        }

        self.stream.send(&CommandResponse::end_of_banlist(channel))
    }

    pub(super) fn send_topic_response(&mut self, channel: &str) -> io::Result<()> {
        match &self.database.get_topic_for_channel(channel) {
            Some(topic) => self.stream.send(&CommandResponse::topic(channel, topic)),
            None => self.stream.send(&CommandResponse::no_topic(channel)),
        }
    }

    pub(super) fn send_whoreply_response(&mut self, client_info: ClientInfo) -> io::Result<()> {
        let channel = self
            .database
            .get_channels_for_client(&client_info.nickname())
            .get(0)
            .map(|string| string.to_owned());

        self.stream
            .send(&CommandResponse::whoreply(&channel, &client_info))
    }

    pub(super) fn send_list_response(&mut self, channel: String) -> io::Result<()> {
        let topic = self
            .database
            .get_topic_for_channel(&channel)
            .unwrap_or_else(|| "No topic set".to_string());

        let prv = self
            .database
            .channel_has_mode(&channel, &ChannelFlag::Private)
            && !self.is_in_channel(&channel);

        self.stream
            .send(&CommandResponse::list(channel, topic, prv))
    }

    pub(super) fn send_names_response(&mut self, channel: &str) -> Result<(), io::Error> {
        let clients = self.database.get_clients_for_channel(channel);
        self.stream
            .send(&CommandResponse::name_reply(channel, &clients))
    }

    pub(super) fn send_join_notification(&mut self, channel: &str) {
        let notification = Notification::join(&self.nickname, channel);
        self.send_message_to_local_clients_on_channel(&notification, channel);

        if is_distributed_channel(channel) {
            self.send_message_to_all_servers(&notification);
        }
    }

    pub(super) fn send_part_notification(&mut self, channel: &str) {
        let notification = Notification::part(&self.nickname, channel);
        self.send_message_to_local_clients_on_channel(&notification, channel);

        if is_distributed_channel(channel) {
            self.send_message_to_all_servers(&notification);
        }
    }

    pub(super) fn send_topic_notification(&mut self, channel: &str, topic: &str) {
        let notification = Notification::topic(&self.nickname, channel, topic);
        self.send_message_to_local_clients_on_channel(&notification, channel);

        if is_distributed_channel(channel) {
            self.send_message_to_all_servers(&notification);
        }
    }

    pub(super) fn send_invite_notification(
        &mut self,
        invited_client: String,
        channel: &str,
    ) -> Result<(), io::Error> {
        let invitation = Notification::invite(&self.nickname, &invited_client, channel);
        self.send_message_to_client(&invitation, &invited_client)
    }

    pub(super) fn send_quit_notification(&mut self, message: &str) {
        let notification = Notification::quit(&self.nickname, message);

        let channels = self.database.get_channels_for_client(&self.nickname);
        for channel in channels {
            self.send_message_to_local_clients_on_channel(&notification, &channel);
        }

        self.send_message_to_all_servers(&notification);
    }

    pub(super) fn send_kick_notification(
        &mut self,
        channel: &str,
        nickname: &str,
        comment: &Option<String>,
    ) {
        let notification = Notification::kick(&self.nickname, channel, nickname, comment);
        self.send_message_to_local_clients_on_channel(&notification, channel);

        if is_distributed_channel(channel) {
            self.send_message_to_all_servers(&notification);
        }
    }

    pub(super) fn send_privmsg_notification(
        &mut self,
        target: &str,
        content: &str,
    ) -> Result<(), io::Error> {
        let notification = Notification::privmsg(&self.nickname, target, content);
        self.send_message_to_target(&notification, target)
    }

    pub(super) fn send_notice_notification(
        &mut self,
        target: &str,
        content: &str,
    ) -> Result<(), io::Error> {
        let notification = Notification::notice(&self.nickname, target, content);

        self.send_message_to_target(&notification, target)
    }

    pub(super) fn send_mode_response(&mut self, channel: &str) -> io::Result<()> {
        if let Some(config) = self.database.get_channel_config(channel) {
            let flags = config.flags;
            let limit = config.user_limit;
            let operators = config.operators;
            let banmasks = config.banmasks;
            let speakers = config.speakers;
            let key = config.key;

            for flag in flags {
                let mode = flag.to_char();
                let reply = CommandResponse::channel_mode_is(channel, mode, None);
                self.stream.send(&reply)?;
            }

            if limit.is_some() {
                let params = vec![limit.unwrap().to_string()];
                let reply = CommandResponse::channel_mode_is(channel, SET_USER_LIMIT, Some(params));
                self.stream.send(&reply)?;
            }

            if key.is_some() {
                let params = vec![key.unwrap()];
                let reply = CommandResponse::channel_mode_is(channel, SET_KEY, Some(params));
                self.stream.send(&reply)?;
            }

            if !operators.is_empty() {
                let reply =
                    CommandResponse::channel_mode_is(channel, SET_OPERATOR, Some(operators));
                self.stream.send(&reply)?;
            }

            if !banmasks.is_empty() {
                let reply = CommandResponse::channel_mode_is(channel, SET_BANMASK, Some(banmasks));
                self.stream.send(&reply)?;
            }

            if !speakers.is_empty() {
                let reply = CommandResponse::channel_mode_is(channel, SET_SPEAKER, Some(speakers));
                self.stream.send(&reply)?;
            }
        }
        Ok(())
    }
}
