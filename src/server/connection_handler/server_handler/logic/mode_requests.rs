use crate::server::consts::modes::{
    ChannelFlag, SET_BANMASK, SET_KEY, SET_OPERATOR, SET_SPEAKER, SET_USER_LIMIT,
};
use crate::{
    macros::{ok_or_return, some_or_return},
    server::{connection::Connection, connection_handler::ServerHandler},
};
impl<C: Connection> ServerHandler<C> {
    pub(super) fn handle_add_mode(&mut self, mode: char, channel: &str, argument: Option<String>) {
        match mode {
            SET_USER_LIMIT => self.set_limit(channel, argument),
            SET_BANMASK => {
                self.add_banmasks(channel, argument);
            }
            SET_SPEAKER => {
                self.add_speakers(channel, argument);
            }
            SET_KEY => {
                self.set_key(channel, argument);
            }
            SET_OPERATOR => self.add_channops(channel, argument),
            char => {
                let flag = ChannelFlag::from_char(char);
                self.database.set_channel_mode(channel, flag)
            }
        }
    }

    pub(super) fn handle_remove_mode(
        &mut self,
        mode: char,
        channel: &str,
        argument: Option<String>,
    ) {
        match mode {
            SET_USER_LIMIT => self.unset_limit(channel),
            SET_BANMASK => {
                self.remove_banmasks(channel, argument);
            }
            SET_SPEAKER => {
                self.remove_speakers(channel, argument);
            }
            SET_KEY => {
                self.unset_key(channel);
            }
            SET_OPERATOR => self.remove_channops(channel, argument),
            char => {
                let flag = ChannelFlag::from_char(char);
                self.database.unset_channel_mode(channel, flag)
            }
        }
    }

    pub(super) fn set_limit(&self, channel: &str, argument: Option<String>) {
        let limit = some_or_return!(argument);
        let limit = ok_or_return!(limit.parse::<usize>());
        self.database.set_channel_limit(channel, Some(limit))
    }

    pub(super) fn add_banmasks(&self, channel: &str, argument: Option<String>) {
        let banmask = some_or_return!(argument);
        self.database.add_channel_banmask(channel, &banmask);
    }

    pub(super) fn remove_banmasks(&self, channel: &str, argument: Option<String>) {
        let banmask = some_or_return!(argument);
        self.database.remove_channel_banmask(channel, &banmask);
    }

    pub(super) fn add_speakers(&self, channel: &str, argument: Option<String>) {
        let speaker = some_or_return!(argument);
        self.database.add_speaker(channel, &speaker);
    }

    pub(super) fn set_key(&self, channel: &str, argument: Option<String>) {
        let key = some_or_return!(argument);
        self.database.set_channel_key(channel, Some(key));
    }

    pub(super) fn add_channops(&self, channel: &str, argument: Option<String>) {
        let channop = some_or_return!(argument);
        self.database.add_channop(channel, &channop);
    }

    pub(super) fn unset_limit(&self, channel: &str) {
        self.database.set_channel_limit(channel, None)
    }

    pub(super) fn remove_speakers(&self, channel: &str, argument: Option<String>) {
        let speaker = some_or_return!(argument);
        self.database.remove_speaker(channel, &speaker);
    }

    pub(super) fn unset_key(&self, channel: &str) {
        self.database.set_channel_key(channel, None)
    }

    pub(super) fn remove_channops(&self, channel: &str, argument: Option<String>) {
        let channop = some_or_return!(argument);
        self.database.remove_channop(channel, &channop)
    }
}
