use crate::server::connection::Connection;
use crate::server::connection_handler::channel_const::*;
use crate::server::connection_handler::command_const::*;
use crate::server::connection_handler::connection_handler_trait::ConnectionHandlerAsserts;
use crate::server::connection_handler::mode_const::*;
use crate::server::connection_handler::responses::ErrorReply;

use super::ClientHandler;

impl<C: Connection> ConnectionHandlerAsserts<C> for ClientHandler<C> {
    fn assert_pass_command_is_valid(&self, _params: &[String]) -> Result<(), ErrorReply> {
        Err(ErrorReply::AlreadyRegistered462)
    }

    fn assert_nick_command_is_valid(&self, params: &[String]) -> Result<(), ErrorReply> {
        if params.is_empty() {
            return Err(ErrorReply::NoNicknameGiven431);
        }

        let nickname = &params[0];
        self.assert_nickname_not_in_use(nickname)
    }

    fn assert_user_command_is_valid(
        &self,
        _params: &[String],
        _trail: &Option<String>,
    ) -> Result<(), ErrorReply> {
        Err(ErrorReply::AlreadyRegistered462)
    }

    fn assert_oper_command_is_valid(&self, params: &[String]) -> Result<(), ErrorReply> {
        if params.len() != 2 {
            let command = OPER_COMMAND.to_string();
            return Err(ErrorReply::NeedMoreParameters461 { command });
        }

        let username = &params[0];
        let password = &params[1];
        self.assert_are_credentials_valid(username, password)
    }

    fn assert_privmsg_command_is_valid(
        &self,
        params: &[String],
        trail: &Option<String>,
    ) -> Result<(), ErrorReply> {
        if params.is_empty() {
            let command = PRIVMSG_COMMAND.to_string();
            return Err(ErrorReply::NoRecipient411 { command });
        }

        if trail.is_none() {
            return Err(ErrorReply::NoTextToSend412 {});
        }

        Ok(())
    }

    fn assert_notice_command_is_valid(
        &self,
        params: &[String],
        trail: &Option<String>,
    ) -> Result<(), ErrorReply> {
        if params.is_empty() {
            let command = NOTICE_COMMAND.to_string();
            return Err(ErrorReply::NoRecipient411 { command });
        }

        if trail.is_none() {
            return Err(ErrorReply::NoTextToSend412 {});
        }

        Ok(())
    }

    fn assert_join_command_is_valid(&self, params: &[String]) -> Result<(), ErrorReply> {
        if params.is_empty() {
            let command = JOIN_COMMAND.to_string();
            return Err(ErrorReply::NeedMoreParameters461 { command });
        }

        Ok(())
    }

    fn assert_part_command_is_valid(&self, params: &[String]) -> Result<(), ErrorReply> {
        if params.is_empty() {
            let command = PART_COMMAND.to_string();
            return Err(ErrorReply::NeedMoreParameters461 { command });
        }

        Ok(())
    }

    fn assert_invite_command_is_valid(&self, params: &[String]) -> Result<(), ErrorReply> {
        if params.len() != 2 {
            let command = INVITE_COMMAND.to_string();
            return Err(ErrorReply::NeedMoreParameters461 { command });
        }

        let invited_client = &params[0];
        let channel = &params[1];

        self.assert_exists_client(invited_client)?;

        if self.database.contains_channel(channel) {
            self.assert_is_in_channel(channel)?;
            self.assert_client_not_on_channel(invited_client, channel)?;
        }

        if self.database.channel_has_mode(channel, INVISIBLE) {
            self.assert_is_channel_operator(channel)?;
        }

        Ok(())
    }

    fn assert_names_command_is_valid(&self, _params: &[String]) -> Result<(), ErrorReply> {
        Ok(())
    }

    fn assert_list_command_is_valid(&self, _params: &[String]) -> Result<(), ErrorReply> {
        Ok(())
    }

    fn assert_who_command_is_valid(&self, _params: &[String]) -> Result<(), ErrorReply> {
        Ok(())
    }

    fn assert_whois_command_is_valid(&self, params: &[String]) -> Result<(), ErrorReply> {
        if params.is_empty() {
            return Err(ErrorReply::NoNicknameGiven431);
        }

        Ok(())
    }

    fn assert_away_command_is_valid(&self, _trail: &Option<String>) -> Result<(), ErrorReply> {
        Ok(())
    }

    fn assert_topic_command_is_valid(&self, params: &[String]) -> Result<(), ErrorReply> {
        if params.is_empty() {
            let command = TOPIC_COMMAND.to_string();
            return Err(ErrorReply::NeedMoreParameters461 { command });
        }

        let channel = &params[0];

        self.assert_is_in_channel(channel)?;

        if self.database.channel_has_mode(channel, TOPIC_SETTABLE) {
            self.assert_is_channel_operator(channel)?;
        }

        Ok(())
    }

    fn assert_kick_command_is_valid(
        &self,
        params: &[String],
        _trail: &Option<String>,
    ) -> Result<(), ErrorReply> {
        if params.len() < 2 {
            let command = KICK_COMMAND.to_string();
            return Err(ErrorReply::NeedMoreParameters461 { command });
        }

        Ok(())
    }

    fn assert_mode_command_is_valid(&self, params: &[String]) -> Result<(), ErrorReply> {
        if params.is_empty() {
            let command = MODE_COMMAND.to_string();
            return Err(ErrorReply::NeedMoreParameters461 { command });
        }

        let channel = &params[0];

        self.assert_exists_channel(channel)?;
        self.assert_is_in_channel(channel)?;

        if params.len() > 1 {
            self.assert_is_channel_operator(channel)?;
            self.assert_modes_starts_correctly(&params[1])?;
        }

        Ok(())
    }

    fn assert_quit_command_is_valid(&self, _trail: &Option<String>) -> Result<(), ErrorReply> {
        Ok(())
    }
}

impl<C: Connection> ClientHandler<C> {
    pub fn assert_is_channel_operator(&self, channel: &str) -> Result<(), ErrorReply> {
        let channel = channel.to_string();

        if !self.database.is_channel_operator(&channel, &self.nickname) {
            return Err(ErrorReply::ChanOPrivIsNeeded482 { channel });
        }

        Ok(())
    }

    pub fn assert_is_in_channel(&self, channel: &str) -> Result<(), ErrorReply> {
        let channel = channel.to_string();

        if !self.database.is_client_in_channel(&self.nickname, &channel) {
            return Err(ErrorReply::NotOnChannel442 { channel });
        }

        Ok(())
    }

    pub fn assert_client_not_on_channel(
        &self,
        client: &str,
        channel: &str,
    ) -> Result<(), ErrorReply> {
        if self.database.is_client_in_channel(client, channel) {
            return Err(ErrorReply::UserOnChannel443 {
                nickname: client.to_string(),
                channel: channel.to_string(),
            });
        }

        Ok(())
    }

    pub fn assert_exists_client(&self, client: &str) -> Result<(), ErrorReply> {
        let nickname = client.to_string();

        if !self.database.contains_client(&nickname) {
            return Err(ErrorReply::NoSuchNickname401 { nickname });
        }

        Ok(())
    }

    pub fn assert_exists_channel(&self, channel: &str) -> Result<(), ErrorReply> {
        let channel = channel.to_string();

        if !self.database.contains_channel(&channel) {
            return Err(ErrorReply::NoSuchChannel403 { channel });
        }

        Ok(())
    }

    pub fn assert_channel_name_is_valid(&self, channel: &str) -> Result<(), ErrorReply> {
        let channel = channel.to_string();

        if !(channel.as_bytes()[0] as char == LOCAL_CHANNEL
            || channel.as_bytes()[0] as char == DISTRIBUTED_CHANNEL)
            || channel.contains(INVALID_CHARACTER)
        {
            return Err(ErrorReply::NoSuchChannel403 { channel });
        }

        Ok(())
    }

    pub fn assert_nickname_not_in_use(&self, nickname: &str) -> Result<(), ErrorReply> {
        let nickname = nickname.to_string();

        if self.database.contains_client(&nickname) {
            return Err(ErrorReply::NicknameInUse433 { nickname });
        }

        Ok(())
    }

    pub fn assert_are_credentials_valid(
        &self,
        username: &str,
        password: &str,
    ) -> Result<(), ErrorReply> {
        if !self.database.are_credentials_valid(username, password) {
            return Err(ErrorReply::PasswordMismatch464);
        }

        Ok(())
    }

    pub fn assert_target_is_valid(&self, target: &str) -> Result<(), ErrorReply> {
        self.assert_target_exists(target)?;

        if self.database.contains_channel(target) {
            self.assert_can_send_to_channel(target)?;
        }

        Ok(())
    }

    pub fn assert_target_exists(&self, target: &str) -> Result<(), ErrorReply> {
        let is_client = self.database.contains_client(target);
        let is_channel = self.database.contains_channel(target);
        let nickname = target.to_string();

        if !(is_client || is_channel) {
            return Err(ErrorReply::NoSuchNickname401 { nickname });
        }

        Ok(())
    }

    pub fn assert_can_send_to_channel(&self, channel: &str) -> Result<(), ErrorReply> {
        let channel = channel.to_string();

        if self
            .database
            .channel_has_mode(&channel, NO_OUTSIDE_MESSAGES)
            && !self.database.is_client_in_channel(&self.nickname, &channel)
        {
            return Err(ErrorReply::CannotSendToChannel404 { channel });
        }

        if self.database.channel_has_mode(&channel, MODERATED)
            && !self.database.is_channel_speaker(&channel, &self.nickname)
        {
            return Err(ErrorReply::CannotSendToChannel404 { channel });
        }

        Ok(())
    }

    pub fn assert_can_join_channel(
        &self,
        channel: &str,
        key: &Option<String>,
    ) -> Result<(), ErrorReply> {
        let channels_for_client = self.database.get_channels_for_client(&self.nickname);
        if channels_for_client.len() == MAX_CHANNELS {
            let channel = channel.to_string();
            return Err(ErrorReply::TooManyChannels405 { channel });
        }

        self.assert_channel_name_is_valid(channel)?;
        self.assert_client_not_on_channel(&self.nickname, channel)?;

        self.assert_is_valid_key(channel, key)?;

        self.assert_channel_is_not_full(channel)?;

        self.assert_is_not_banned_from_channel(channel)
    }

    pub fn assert_can_kick_from_channel(&self, channel: &str) -> Result<(), ErrorReply> {
        self.assert_exists_channel(channel)?;

        self.assert_is_in_channel(channel)?;

        self.assert_is_channel_operator(channel)
    }

    pub fn assert_can_part_channel(&self, channel: &str) -> Result<(), ErrorReply> {
        self.assert_channel_name_is_valid(channel)?;
        self.assert_exists_channel(channel)?;

        self.assert_is_in_channel(channel)
    }

    pub fn assert_can_modify_client_status_in_channel(
        &self,
        channel: &str,
        client: &str,
    ) -> Result<(), ErrorReply> {
        self.assert_exists_client(client)?;

        self.assert_is_client_in_channel(channel, client)
    }

    pub fn assert_is_client_in_channel(
        &self,
        channel: &str,
        client: &str,
    ) -> Result<(), ErrorReply> {
        let channel = channel.to_string();

        if !self.database.is_client_in_channel(client, &channel) {
            return Err(ErrorReply::NotOnChannel442 { channel });
        }

        Ok(())
    }

    pub fn assert_can_set_key(&mut self, channel: &str) -> Result<(), ErrorReply> {
        if self.database.get_channel_key(channel).is_some() {
            return Err(ErrorReply::KeySet467 {
                channel: channel.to_string(),
            });
        }

        Ok(())
    }

    pub fn assert_modes_starts_correctly(&self, modes: &str) -> Result<(), ErrorReply> {
        if !modes.starts_with([ADD_MODE, REMOVE_MODE]) {
            return Err(ErrorReply::NoReply);
        }

        Ok(())
    }

    pub fn assert_is_valid_key(
        &self,
        channel: &str,
        key: &Option<String>,
    ) -> Result<(), ErrorReply> {
        let channel = channel.to_string();

        if self.database.get_channel_key(&channel) != *key {
            return Err(ErrorReply::BadChannelKey475 { channel });
        }

        Ok(())
    }

    pub fn assert_channel_is_not_full(&self, channel: &str) -> Result<(), ErrorReply> {
        let channel = channel.to_string();

        if let Some(limit) = self.database.get_channel_limit(&channel) {
            if self.database.get_clients_for_channel(&channel).len() >= limit {
                return Err(ErrorReply::ChannelIsFull471 { channel });
            }
        }
        Ok(())
    }

    pub fn assert_is_not_banned_from_channel(&self, channel: &str) -> Result<(), ErrorReply> {
        for mask in self.database.get_channel_banmask(channel) {
            if self.database.client_matches_banmask(&self.nickname, &mask) {
                let channel = channel.to_string();
                return Err(ErrorReply::BannedFromChannel474 { channel });
            }
        }

        Ok(())
    }
}
