use std::io;

use crate::macros::ok_or_return;
use crate::server::connection_handler::client_handler::booleans::is_distributed_channel;
use crate::server::connection_handler::mode_requests::{ChannelModeRequest, UserModeRequest};
use crate::server::consts::channel_flag::ChannelFlag;
use crate::server::consts::modes::{
    SET_BANMASK, SET_KEY, SET_OPERATOR, SET_SPEAKER, SET_USER_LIMIT,
};
use crate::server::consts::user_flag::UserFlag;
use crate::server::responses::{CommandResponse, Notification};
use crate::server::{
    connection::Connection,
    connection_handler::{client_handler::ClientHandler, ConnectionHandlerUtils},
};

use crate::server::data_structures::*;

impl<C: Connection> ClientHandler<C> {
    pub(super) fn send_join_response(&mut self, channel: &str) -> io::Result<()> {
        self.send_topic_response(channel)?;

        let clients = ok_or_return!(self.database.get_channel_clients(channel), Ok(()));

        self.stream
            .send(&CommandResponse::name_reply(channel, &clients))
    }

    pub fn send_oper_notification(&mut self) {
        let notification = Notification::mode(&self.nickname, &self.nickname, "+o");
        self.send_message_to_all_servers(&notification);
    }

    pub(super) fn send_whois_response(&mut self, client_info: ClientInfo) -> io::Result<()> {
        self.stream
            .send(&CommandResponse::whois_user(&client_info))?;

        let nickname = &client_info.nickname;
        let servername = &client_info.servername;

        let serverinfo = if servername != &self.database.get_server_name() {
            ok_or_return!(self.database.get_server_info(servername), Ok(())).serverinfo
        } else {
            self.database.get_own_server_info()
        };

        self.stream.send(&CommandResponse::whois_server(
            nickname,
            servername,
            &serverinfo,
        ))?;

        self.send_whois_operator_response(nickname)?;
        self.send_whois_channels_response(nickname)?;

        if let Some(message) = client_info.away {
            self.stream
                .send(&CommandResponse::away(nickname, &message))?;
        }

        self.stream.send(&CommandResponse::end_of_whois(nickname))?;

        Ok(())
    }

    fn send_whois_channels_response(&mut self, nickname: &str) -> Result<(), io::Error> {
        let mut channels = ok_or_return!(self.database.get_channels_for_client(nickname), Ok(()));
        if !channels.is_empty() {
            for channel in &mut channels {
                if let Some(role) = self.get_client_role_in_channel(channel, nickname) {
                    channel.insert(0, role);
                }
            }
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
        let banmasks = ok_or_return!(self.database.get_channel_banmask(channel), Ok(()));
        for banmask in banmasks {
            self.stream
                .send(&CommandResponse::banlist(channel, &banmask))?;
        }

        self.stream.send(&CommandResponse::end_of_banlist(channel))
    }

    pub(super) fn send_topic_response(&mut self, channel: &str) -> io::Result<()> {
        let topic = ok_or_return!(self.database.get_channel_topic(channel), Ok(()));
        match &topic {
            Some(topic) => self.stream.send(&CommandResponse::topic(channel, topic)),
            None => self.stream.send(&CommandResponse::no_topic(channel)),
        }
    }

    pub(super) fn send_whoreply_response(&mut self, client_info: ClientInfo) -> io::Result<()> {
        let channel = ok_or_return!(
            self.database.get_channels_for_client(&client_info.nickname),
            Ok(())
        )
        .get(0)
        .map(|string| string.to_owned());

        self.stream
            .send(&CommandResponse::whoreply(&channel, &client_info))
    }

    pub(super) fn send_list_response(&mut self, channel: String) -> io::Result<()> {
        let topic = ok_or_return!(self.database.get_channel_topic(&channel), Ok(()))
            .unwrap_or_else(|| "No topic set".to_string());

        let prv = self
            .database
            .channel_has_flag(&channel, ChannelFlag::Private)
            && !self.is_in_channel(&channel);

        self.stream
            .send(&CommandResponse::list(channel, topic, prv))
    }

    pub(super) fn send_name_response_for_remaining_clients(&mut self) -> Result<(), io::Error> {
        let remaining_clients: Vec<String> = self
            .clients_in_no_channel()
            .iter()
            .map(|client| client.nickname.clone())
            .collect();

        if !remaining_clients.is_empty() {
            self.stream
                .send(&CommandResponse::name_reply("*", &remaining_clients))?;
        }

        self.stream.send(&CommandResponse::end_of_names(""))?;
        Ok(())
    }

    pub(super) fn send_names_response(&mut self, channel: &str) -> Result<(), io::Error> {
        let mut clients = ok_or_return!(self.database.get_channel_clients(channel), Ok(()));
        for client in &mut clients {
            if let Some(role) = self.get_client_role_in_channel(channel, client) {
                client.insert(0, role);
            }
        }
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
        invited_client: &str,
        channel: &str,
    ) -> Result<(), io::Error> {
        let invitation = Notification::invite(&self.nickname, invited_client, channel);
        self.send_message_to_client(&invitation, invited_client)
    }

    pub(super) fn send_quit_notification(&mut self, nickname: &str, message: &str) {
        let quit_notification = Notification::quit(nickname, message);
        let channels = ok_or_return!(self.database.get_channels_for_client(nickname));
        for channel in channels {
            self.send_message_to_local_clients_on_channel(&quit_notification, &channel);
        }
        self.send_message_to_all_servers(&quit_notification);
    }

    pub(super) fn send_squit_notification(&mut self, servername: &str, comment: Option<String>) {
        let notification = Notification::squit(&self.nickname, servername, comment);
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

    pub(super) fn send_privmsg_notification(&mut self, target: &str, content: &str) {
        let notification = Notification::privmsg(&self.nickname, target, content);
        self.send_message_to_target(&notification, target);
    }

    pub(super) fn send_notice_notification(&mut self, target: &str, content: &str) {
        let notification = Notification::notice(&self.nickname, target, content);
        self.send_message_to_target(&notification, target)
    }

    pub(super) fn send_channel_mode_is_response(&mut self, channel: &str) -> io::Result<()> {
        let config = ok_or_return!(self.database.get_channel_config(channel), Ok(()));

        let flags = config.flags;
        let limit = config.user_limit;
        let operators = config.operators;
        let banmasks = config.banmasks;
        let speakers = config.speakers;
        let key = config.key;

        self.send_channel_flags_response(flags, channel)?;
        self.send_channel_limit_response(limit, channel)?;
        self.send_channel_key_response(key, channel)?;
        self.send_channel_operators_response(operators, channel)?;
        self.send_channel_banmasks_response(banmasks, channel)?;
        self.send_channel_speakers_response(speakers, channel)?;

        Ok(())
    }

    fn send_channel_speakers_response(
        &mut self,
        speakers: Vec<String>,
        channel: &str,
    ) -> Result<(), io::Error> {
        if !speakers.is_empty() {
            let reply = CommandResponse::channel_mode_is(channel, SET_SPEAKER, Some(speakers));
            self.stream.send(&reply)?;
        };
        Ok(())
    }

    fn send_channel_banmasks_response(
        &mut self,
        banmasks: Vec<String>,
        channel: &str,
    ) -> Result<(), io::Error> {
        if !banmasks.is_empty() {
            let reply = CommandResponse::channel_mode_is(channel, SET_BANMASK, Some(banmasks));
            self.stream.send(&reply)?;
        };
        Ok(())
    }

    fn send_channel_operators_response(
        &mut self,
        operators: Vec<String>,
        channel: &str,
    ) -> Result<(), io::Error> {
        if !operators.is_empty() {
            let reply = CommandResponse::channel_mode_is(channel, SET_OPERATOR, Some(operators));
            self.stream.send(&reply)?;
        };
        Ok(())
    }

    fn send_channel_key_response(
        &mut self,
        key: Option<String>,
        channel: &str,
    ) -> Result<(), io::Error> {
        if key.is_some() {
            let params = vec![key.expect("Verified in if condition")];
            let reply = CommandResponse::channel_mode_is(channel, SET_KEY, Some(params));
            self.stream.send(&reply)?;
        };
        Ok(())
    }

    fn send_channel_limit_response(
        &mut self,
        limit: Option<usize>,
        channel: &str,
    ) -> Result<(), io::Error> {
        if limit.is_some() {
            let params = vec![limit.expect("Verified in if condition").to_string()];
            let reply = CommandResponse::channel_mode_is(channel, SET_USER_LIMIT, Some(params));
            self.stream.send(&reply)?;
        };
        Ok(())
    }

    fn send_channel_flags_response(
        &mut self,
        flags: Vec<ChannelFlag>,
        channel: &str,
    ) -> Result<(), io::Error> {
        for flag in flags {
            let mode = flag.to_char();
            let reply = CommandResponse::channel_mode_is(channel, mode, None);
            self.stream.send(&reply)?;
        }
        Ok(())
    }

    pub(super) fn send_user_mode_is_response(&mut self, user: &str) -> io::Result<()> {
        let client_info = self.database.get_client_info(user).expect("User exists");
        let user_modes = client_info.flags.keys().map(UserFlag::to_char).collect();

        let response = CommandResponse::UserModeIs221 { user_modes };
        self.stream.send(&response)
    }

    pub(super) fn send_channel_mode_request_notification(
        &mut self,
        channel: &str,
        request: ChannelModeRequest,
    ) {
        let request = request.to_string();
        let notification = Notification::mode(&self.nickname, channel, &request);
        self.send_message_to_local_clients_on_channel(&notification, channel);
        self.send_message_to_all_servers(&notification);
    }

    pub(super) fn send_user_mode_request_notification(
        &mut self,
        request: UserModeRequest,
        user: &str,
    ) {
        let request = request.to_string();
        let notification = Notification::mode(&self.nickname, user, &request);
        self.send_message_to_all_servers(&notification);
    }
}
