use std::io;

use crate::server::connection::Connection;
use crate::server::connection_handler::commands::MODE_COMMAND;
use crate::server::connection_handler::connection_handler_trait::{
    ConnectionHandlerLogic, ConnectionHandlerUtils,
};
use crate::server::connection_handler::modes::*;
use crate::server::connection_handler::responses::{CommandResponse, ErrorReply, Notification};
use crate::server::database::ClientInfo;

use super::{ClientHandler, ADD_MODE, REMOVE_MODE};

impl<C: Connection> ConnectionHandlerLogic<C> for ClientHandler<C> {
    fn nick_logic(&mut self, _params: Vec<String>) -> io::Result<bool> {
        Ok(true)
    }

    fn oper_logic(&mut self, _params: Vec<String>) -> std::io::Result<bool> {
        self.database.set_server_operator(&self.nickname);

        self.send_response(&CommandResponse::YouAreOper381)?;

        Ok(true)
    }

    fn privmsg_logic(
        &mut self,
        mut params: Vec<String>,
        trail: Option<String>,
    ) -> std::io::Result<bool> {
        let content = trail.unwrap();
        let targets = params.remove(0);

        for target in targets.split(',') {
            if let Err(error) = self.assert_target_is_valid(target) {
                self.send_response(&error)?;
                continue;
            }

            self.send_privmsg_to_target(target, &content)?;
        }

        Ok(true)
    }

    fn notice_logic(
        &mut self,
        mut params: Vec<String>,
        trail: Option<String>,
    ) -> std::io::Result<bool> {
        let content = trail.unwrap();
        let targets = params.pop().unwrap();

        for target in targets.split(',') {
            if let Err(error) = self.assert_target_is_valid(target) {
                self.send_response(&error)?;
                continue;
            }

            self.send_notice_to_target(target, &content)?;
        }

        Ok(true)
    }

    fn join_logic(&mut self, params: Vec<String>) -> std::io::Result<bool> {
        let channels = params[0].split(',');

        let mut keys = get_keys_split(params.get(1)).into_iter();

        for channel in channels {
            let key = keys.next();

            if let Err(error) = self.assert_can_join_channel(channel, &self.nickname) {
                self.send_response(&error)?;
                continue;
            }

            if self.database.get_channel_key(channel) != key {
                self.send_response(&ErrorReply::BadChannelKey475 {
                    channel: channel.to_string(),
                })?;
                continue;
            }

            if let Some(limit) = self.database.get_channel_limit(channel) {
                if self.database.get_clients_for_channel(channel).len() >= limit {
                    self.send_response(&ErrorReply::ChannelIsFull471 {
                        channel: channel.to_string(),
                    })?;
                    continue;
                }
            }

            if self.client_matches_banmask(channel, &self.nickname) {
                self.send_response(&ErrorReply::BannedFromChannel474 {
                    channel: channel.to_string(),
                })?;
                continue;
            }

            let notification = Notification::Join {
                nickname: self.nickname.clone(),
                channel: channel.to_string(),
            };

            self.send_message_to_channel(&notification, channel);

            self.database.add_client_to_channel(&self.nickname, channel);

            self.send_topic_reply(channel.to_string())?;

            self.send_response(&CommandResponse::NameReply353 {
                channel: channel.to_string(),
                clients: self.database.get_clients_for_channel(channel),
            })?;
        }

        Ok(true)
    }

    fn part_logic(&mut self, mut _params: Vec<String>) -> std::io::Result<bool> {
        let channels = _params.pop().unwrap();
        let nickname = self.nickname.clone();

        for channel in channels.split(',') {
            if let Err(error) = self.assert_can_part_channel(channel) {
                self.send_response(&error)?;
                continue;
            }
            let notification = Notification::Part {
                nickname: self.nickname.clone(),
                channel: channel.to_string(),
            };
            self.send_message_to_channel(&notification, channel);
            self.database.remove_client_from_channel(&nickname, channel);
        }

        Ok(true)
    }

    fn invite_logic(&mut self, mut params: Vec<String>) -> std::io::Result<bool> {
        let channel = params.pop().unwrap();
        let invited_client = params.pop().unwrap();
        let inviting_client = self.nickname.clone();

        let invitation = Notification::Invite {
            inviting_client: inviting_client.clone(),
            invited_client: invited_client.clone(),
            channel: channel.clone(),
        };

        if self
            .send_message_to_client(&invitation, &invited_client)
            .is_err()
        {
            self.send_response(&ErrorReply::NoSuchNickname401 {
                nickname: invited_client.clone(),
            })?
        }

        self.send_response(&CommandResponse::Inviting341 {
            nickname: inviting_client,
            channel,
        })?;

        Ok(true)
    }

    fn names_logic(&mut self, params: Vec<String>) -> std::io::Result<bool> {
        let channels = self.get_channels_for_query(params.get(0));

        for channel in channels {
            if !self.database.contains_channel(&channel) {
                continue;
            }

            if !self.can_name_channel(&channel) {
                continue;
            }

            let clients = self.database.get_clients_for_channel(&channel);
            self.send_response(&CommandResponse::NameReply353 {
                channel: channel.clone(),
                clients,
            })?;

            if !params.is_empty() {
                self.send_response(&CommandResponse::EndOfNames366 { channel })?
            }
        }

        if params.is_empty() {
            self.send_response(&CommandResponse::EndOfNames366 {
                channel: "".to_string(),
            })?;
        }

        Ok(true)
    }

    fn list_logic(&mut self, _params: Vec<String>) -> std::io::Result<bool> {
        let channels = self.get_channels_for_query(_params.get(0));

        self.send_response(&CommandResponse::ListStart321)?;

        for channel in channels {
            if self.can_list_channel(&channel) {
                let topic = match self.database.get_topic_for_channel(&channel) {
                    Some(topic) => topic,
                    None => "No topic set".to_string(),
                };

                if self.database.channel_has_mode(&channel, 's')
                    && !self.database.is_client_in_channel(&self.nickname, &channel)
                {
                    continue;
                }
                let prv = self.database.channel_has_mode(&channel, 'p')
                    && !self.database.is_client_in_channel(&self.nickname, &channel);

                self.send_response(&CommandResponse::List322 {
                    channel,
                    prv,
                    topic,
                })?;
            }
        }
        self.send_response(&CommandResponse::ListEnd323)?;

        Ok(true)
    }

    fn who_logic(&mut self, mut params: Vec<String>) -> std::io::Result<bool> {
        let mask = params.pop();

        let mut clients = match &mask {
            Some(mask) => self.database.get_clients_for_mask(mask),
            None => self.filtered_clients_for_default_who_command(),
        };

        clients.sort();

        for client_info in clients {
            self.send_whoreply_for_client(client_info)?;
        }

        self.send_response(&CommandResponse::EndOfWho315 { name: mask })?;

        Ok(true)
    }

    fn whois_logic(&mut self, mut params: Vec<String>) -> std::io::Result<bool> {
        let (_server, nickmasks) = if params.len() == 2 {
            (params.get(0).map(|s| s.to_string()), params.remove(1))
        } else {
            (None, params.remove(1))
        };

        for nickmask in nickmasks.split(',') {
            let mut clients: Vec<ClientInfo> = self.database.get_clients_for_nickmask(nickmask);

            if clients.is_empty() {
                let nickname = nickmask.to_string();
                self.send_response(&ErrorReply::NoSuchNickname401 { nickname })?;
                continue;
            }

            clients.sort();

            for client in clients {
                self.send_whois_responses(client)?;
            }
        }

        Ok(true)
    }

    fn away_logic(&mut self, trail: Option<String>) -> std::io::Result<bool> {
        self.database.set_away_message(&trail, &self.nickname);

        let reply = match trail {
            Some(_) => CommandResponse::NowAway,
            None => CommandResponse::UnAway,
        };

        self.send_response(&reply)?;

        Ok(true)
    }

    fn topic_logic(&mut self, mut params: Vec<String>) -> std::io::Result<bool> {
        let channel = params.remove(0);

        if let Some(topic) = params.pop() {
            self.database.set_channel_topic(&channel, &topic);
        } else {
            self.send_topic_reply(channel)?;
        }

        Ok(true)
    }

    fn kick_logic(&mut self, params: Vec<String>, trail: Option<String>) -> std::io::Result<bool> {
        let channel = params[0].split(',');
        let nickname = params[1].split(',');

        for (channel, nickname) in channel.zip(nickname) {
            if let Err(error) = self.assert_can_kick_from_channel(channel) {
                self.send_response(&error)?;
            } else {
                self.kick_client_from_channel(nickname, channel, &trail);
            }
        }

        Ok(true)
    }

    fn mode_logic(&mut self, _params: Vec<String>) -> std::io::Result<bool> {
        let modes: Vec<char> = _params[1].chars().collect();

        let (add, remove) = parse_modes(modes);

        self.add_modes(add, _params.clone())?;
        self.remove_modes(remove, _params)?;

        Ok(true)
    }

    fn quit_logic(&mut self, trail: Option<String>) -> std::io::Result<bool> {
        let message = trail.unwrap_or_else(|| self.nickname.clone());

        let notification = Notification::Quit { message };

        self.database.disconnect_client(&self.nickname);
        let channels = self.database.get_channels_for_client(&self.nickname);
        for channel in channels {
            self.send_message_to_channel(&channel, &notification.to_string());
        }

        self.send_response(&notification.to_string())?;

        Ok(false)
    }
}

impl<C: Connection> ClientHandler<C> {
    fn send_privmsg_to_target(
        &mut self,
        target: &str,
        content: &str,
    ) -> Result<(), std::io::Error> {
        let nickname = self.nickname.clone();
        let notification = Notification::Privmsg {
            sender: nickname,
            target: target.to_string(),
            message: content.to_owned(),
        };

        self.send_message_to_target(&notification.to_string(), target)?;

        if self.database.contains_client(target) {
            if let Some(message) = self.database.get_away_message(target) {
                let nickname = target.to_string();
                let reply = CommandResponse::Away { nickname, message };
                self.send_response(&reply)?;
            }
        }

        Ok(())
    }

    fn send_notice_to_target(&mut self, target: &str, content: &str) -> Result<(), std::io::Error> {
        let nickname = self.nickname.clone();
        let notification = Notification::Notice {
            sender: nickname,
            target: target.to_string(),
            message: content.to_owned(),
        };

        self.send_message_to_target(&notification.to_string(), target)?;

        Ok(())
    }

    fn filtered_clients_for_default_who_command(&self) -> Vec<ClientInfo> {
        self.database
            .get_all_clients()
            .into_iter()
            .filter(|client_info| self.shares_channel_with_self(client_info))
            .collect()
    }

    fn shares_channel_with_self(&self, client_info: &ClientInfo) -> bool {
        let client_channels = self.database.get_channels_for_client(&client_info.nickname);
        let self_channels = self.database.get_channels_for_client(&self.nickname);

        !client_channels
            .iter()
            .any(|channel| self_channels.contains(channel))
    }

    fn send_whoreply_for_client(&mut self, client_info: ClientInfo) -> io::Result<()> {
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

    fn send_whois_responses(&mut self, client_info: ClientInfo) -> Result<(), io::Error> {
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

        let channels = self.database.get_channels_for_client(&nickname);
        if !channels.is_empty() {
            self.send_response(&CommandResponse::WhoisChannels319 {
                nickname: nickname.clone(),
                channels,
            })?;
        }
        self.send_response(&CommandResponse::EndOfWhois318 { nickname })?;

        Ok(())
    }

    fn client_matches_banmask(&self, channel: &str, nickname: &str) -> bool {
        for mask in self.database.get_channel_banmask(channel) {
            if self.database.client_matches_banmask(nickname, &mask) {
                return true;
            }
        }
        false
    }

    fn send_topic_reply(&mut self, channel: String) -> Result<(), io::Error> {
        match self.database.get_topic_for_channel(&channel) {
            Some(topic) => self.send_response(&CommandResponse::Topic332 { channel, topic })?,
            None => self.send_response(&CommandResponse::NoTopic331 { channel })?,
        };
        Ok(())
    }

    fn kick_client_from_channel(
        &mut self,
        nickname: &str,
        channel: &str,
        comment: &Option<String>,
    ) {
        let notification = Notification::Kick {
            kicker: self.nickname.clone(),
            channel: channel.to_string(),
            kicked: nickname.to_string(),
            comment: comment.clone(),
        };

        self.send_message_to_channel(&notification, channel);
        self.database.remove_client_from_channel(nickname, channel);
    }

    fn get_channels_for_query(&mut self, channels: Option<&String>) -> Vec<String> {
        if channels.is_none() {
            let mut channels = self.database.get_all_channels();
            channels.sort();
            return channels;
        }

        collect_parameters(channels.unwrap())
    }

    fn can_name_channel(&mut self, channel: &str) -> bool {
        let is_priv_or_secret = self.database.channel_has_mode(channel, 's')
            || self.database.channel_has_mode(channel, 'p');

        let is_client_in_channel = self.database.is_client_in_channel(&self.nickname, channel);

        !is_priv_or_secret || is_client_in_channel
    }

    fn can_list_channel(&self, channel: &str) -> bool {
        if self.database.channel_has_mode(channel, 's')
            && !self.database.is_client_in_channel(&self.nickname, channel)
        {
            return false;
        }

        if self.database.contains_channel(channel) {
            return true;
        }

        false
    }

    fn add_modes(&mut self, add: Vec<char>, parameters: Vec<String>) -> Result<(), io::Error> {
        let channel = &parameters[0];
        let argument = parameters.get(2);

        for mode in add {
            match mode {
                SET_OPERATOR => self.add_channop(channel, argument)?,
                SET_USER_LIMIT => {
                    self.set_limit(channel, argument)?;
                }
                SET_BANMASK => {
                    self.set_banmask(channel, argument)?;
                }
                SET_SPEAKER => {
                    self.add_speaker(channel, argument)?;
                }
                SET_KEY => {
                    self.set_key(channel, argument)?;
                }
                mode if VALID_MODES.contains(&mode) => {
                    self.database.set_channel_mode(channel, mode as char)
                }
                mode => self.send_response(&ErrorReply::UnknownMode472 { mode })?,
            }
        }

        Ok(())
    }

    fn remove_modes(
        &mut self,
        remove: Vec<char>,
        parameters: Vec<String>,
    ) -> Result<(), io::Error> {
        let channel = &parameters[0];
        let argument = parameters.get(2);

        for mode in remove {
            match mode {
                SET_OPERATOR => {
                    self.remove_channop(channel, argument)?;
                }
                SET_USER_LIMIT => self.database.set_channel_limit(channel, None),
                SET_BANMASK => {
                    self.remove_banmask(channel, argument)?;
                }
                SET_SPEAKER => {
                    self.remove_speaker(channel, argument)?;
                }
                SET_KEY => self.database.set_channel_key(channel, None),
                mode if VALID_MODES.contains(&mode) => {
                    if self.database.channel_has_mode(channel, mode as char) {
                        self.database.unset_channel_mode(channel, mode as char)
                    }
                }
                mode => self.send_response(&ErrorReply::UnknownMode472 { mode })?,
            }
        }

        Ok(())
    }

    fn add_channop(&mut self, channel: &str, operators: Option<&String>) -> io::Result<()> {
        let operators = match operators {
            Some(operators) => operators,
            None => {
                return self.send_response(&ErrorReply::NeedMoreParameters461 {
                    command: MODE_COMMAND.to_string(),
                })
            }
        };
        for (i, nickname) in operators.split(',').enumerate() {
            if i == 3 {
                break;
            }
            if let Err(error) = self.assert_can_modify_client_status_in_channel(channel, nickname) {
                self.send_response(&error)?;
                continue;
            }
            self.database.add_channop(channel, nickname);
        }
        Ok(())
    }

    fn remove_channop(&mut self, channel: &str, operators: Option<&String>) -> io::Result<()> {
        let operators = match operators {
            Some(operators) => operators,
            None => {
                return self.send_response(&ErrorReply::NeedMoreParameters461 {
                    command: MODE_COMMAND.to_string(),
                })
            }
        };
        for (i, nickname) in operators.split(',').enumerate() {
            if i == 3 {
                break;
            }
            if let Err(error) = self.assert_can_modify_client_status_in_channel(channel, nickname) {
                self.send_response(&error)?;
                continue;
            }
            if !self.database.is_channel_operator(channel, nickname) {
                continue;
            }
            self.database.remove_channop(channel, nickname);
        }
        Ok(())
    }

    fn set_limit(&mut self, channel: &str, limit: Option<&String>) -> io::Result<()> {
        let limit = match limit {
            Some(limit) => limit,
            None => {
                return self.send_response(&ErrorReply::NeedMoreParameters461 {
                    command: MODE_COMMAND.to_string(),
                })
            }
        };

        if let Ok(limit) = limit.parse::<usize>() {
            self.database.set_channel_limit(channel, Some(limit));
        }
        Ok(())
    }

    fn set_banmask(&mut self, channel: &str, banmasks: Option<&String>) -> io::Result<()> {
        if banmasks.is_none() {
            return self.send_ban_reply(channel);
        }
        let masks = banmasks.unwrap().split(',');
        for (i, banmask) in masks.enumerate() {
            if i == 3 {
                break;
            }
            self.database.set_channel_banmask(channel, banmask)
        }
        Ok(())
    }

    fn send_ban_reply(&mut self, channel: &str) -> io::Result<()> {
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

    fn remove_banmask(&mut self, channel: &str, banmasks: Option<&String>) -> io::Result<()> {
        let banmasks = match banmasks {
            Some(banmasks) => banmasks,
            None => {
                return self.send_response(&ErrorReply::NeedMoreParameters461 {
                    command: MODE_COMMAND.to_string(),
                })
            }
        };
        for banmask in banmasks.split(',') {
            self.database.unset_channel_banmask(channel, banmask)
        }
        Ok(())
    }

    fn add_speaker(&mut self, channel: &str, speakers: Option<&String>) -> io::Result<()> {
        let speakers = match speakers {
            Some(speakers) => speakers,
            None => {
                return self.send_response(&ErrorReply::NeedMoreParameters461 {
                    command: MODE_COMMAND.to_string(),
                })
            }
        };

        for nickname in speakers.split(',') {
            if let Err(error) = self.assert_can_modify_client_status_in_channel(channel, nickname) {
                self.send_response(&error)?;
                continue;
            }
            self.database.add_speaker(channel, nickname);
        }
        Ok(())
    }

    fn remove_speaker(&mut self, channel: &str, speakers: Option<&String>) -> io::Result<()> {
        let speakers = match speakers {
            Some(speakers) => speakers,
            None => {
                return self.send_response(&ErrorReply::NeedMoreParameters461 {
                    command: MODE_COMMAND.to_string(),
                });
            }
        };
        for nickname in speakers.split(',') {
            if let Err(error) = self.assert_can_modify_client_status_in_channel(channel, nickname) {
                self.send_response(&error)?;
                continue;
            }
            if !self.database.is_channel_speaker(channel, nickname) {
                continue;
            }
            self.database.remove_speaker(channel, nickname);
        }
        Ok(())
    }

    fn set_key(&mut self, channel: &str, key: Option<&String>) -> io::Result<()> {
        let key = match key {
            Some(key) => key,
            None => {
                return self.send_response(&ErrorReply::NeedMoreParameters461 {
                    command: MODE_COMMAND.to_string(),
                });
            }
        };

        if let Err(error) = self.assert_can_set_key(channel) {
            return self.send_response(&error);
        }
        self.database
            .set_channel_key(channel, Some(key.to_string()));

        Ok(())
    }
}

fn get_keys_split(keys: Option<&String>) -> Vec<String> {
    match keys {
        Some(keys) => keys.split(',').map(|s| s.to_string()).collect(),
        None => vec![],
    }
}

fn collect_parameters(parameters: &str) -> Vec<String> {
    parameters
        .split(',')
        .map(|string| string.to_string())
        .collect()
}

fn parse_modes(modes: Vec<char>) -> (Vec<char>, Vec<char>) {
    let mut add_modes: Vec<char> = vec![];
    let mut remove_modes: Vec<char> = vec![];
    let mut add: bool = false;
    for char in modes {
        match char {
            ADD_MODE => {
                add = true;
            }
            REMOVE_MODE => {
                add = false;
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
