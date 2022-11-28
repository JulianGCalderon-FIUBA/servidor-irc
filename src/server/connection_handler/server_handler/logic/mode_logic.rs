use crate::server::connection_handler::mode_requests::{ChannelModeRequest, UserModeRequest};

use crate::server::consts::modes::{ChannelFlag, UserFlag};
use crate::server::{connection::Connection, connection_handler::ServerHandler};
impl<C: Connection> ServerHandler<C> {
    pub(super) fn handle_channel_mode_request(
        &mut self,
        channel: &str,
        request: ChannelModeRequest,
    ) {
        match request {
            ChannelModeRequest::AddBanmask(banmask) => self.add_banmask_request(channel, banmask),
            ChannelModeRequest::AddOperator(operator) => {
                self.add_operator_request(channel, operator)
            }
            ChannelModeRequest::AddSpeaker(speaker) => self.add_speaker_request(channel, speaker),
            ChannelModeRequest::RemoveBanmask(banmask) => {
                self.remove_banmask_request(channel, banmask)
            }
            ChannelModeRequest::RemoveOperator(operator) => {
                self.remove_operator_request(channel, operator)
            }
            ChannelModeRequest::RemoveSpeaker(speaker) => {
                self.remove_speaker_request(channel, speaker)
            }
            ChannelModeRequest::SetFlag(flag) => self.set_channel_flag_request(channel, flag),
            ChannelModeRequest::UnsetFlag(flag) => self.unset_channel_flag_request(channel, flag),
            ChannelModeRequest::SetKey(key) => self.set_key_request(channel, key),
            ChannelModeRequest::SetLimit(limit) => self.set_limit_request(channel, limit),
            ChannelModeRequest::UnsetLimit() => self.unset_limit_request(channel),
            ChannelModeRequest::UnsetKey() => self.unset_key_request(channel),
            _ => (),
        }
    }

    fn add_banmask_request(&self, channel: &str, banmask: String) {
        self.database.add_channel_banmask(channel, &banmask);
    }

    fn set_limit_request(&self, channel: &str, limit: usize) {
        self.database.set_channel_limit(channel, Some(limit))
    }

    fn set_channel_flag_request(&self, channel: &str, flag: ChannelFlag) {
        self.database.set_channel_mode(channel, flag)
    }

    fn unset_channel_flag_request(&self, channel: &str, flag: ChannelFlag) {
        self.database.unset_channel_mode(channel, flag)
    }

    fn remove_banmask_request(&self, channel: &str, banmask: String) {
        self.database.remove_channel_banmask(channel, &banmask);
    }

    fn add_speaker_request(&self, channel: &str, speaker: String) {
        self.database.add_speaker(channel, &speaker);
    }

    fn set_key_request(&self, channel: &str, key: String) {
        self.database.set_channel_key(channel, Some(key));
    }

    fn add_operator_request(&self, channel: &str, operator: String) {
        self.database.add_channop(channel, &operator);
    }

    fn unset_limit_request(&self, channel: &str) {
        self.database.set_channel_limit(channel, None)
    }

    fn remove_speaker_request(&self, channel: &str, speaker: String) {
        self.database.remove_speaker(channel, &speaker);
    }

    fn unset_key_request(&self, channel: &str) {
        self.database.set_channel_key(channel, None)
    }

    fn remove_operator_request(&self, channel: &str, operator: String) {
        self.database.remove_channop(channel, &operator)
    }
}

impl<C: Connection> ServerHandler<C> {
    pub(super) fn handle_user_mode_request(&mut self, user: &str, request: UserModeRequest) {
        match request {
            UserModeRequest::SetFlag(flag) => self.set_user_flag_request(user, flag),
            UserModeRequest::UnsetFlag(flag) => self.unset_user_flag_request(user, flag),
            _ => (),
        }
    }

    fn set_user_flag_request(&self, user: &str, flag: UserFlag) {
        self.database.set_user_mode(user, flag);
    }

    fn unset_user_flag_request(&self, user: &str, flag: UserFlag) {
        self.database.unset_user_mode(user, flag);
    }
}
