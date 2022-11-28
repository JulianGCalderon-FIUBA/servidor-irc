use crate::macros::ok_or_return;
use crate::server::consts::channel::DISTRIBUTED_CHANNEL;
use crate::server::consts::modes::ChannelFlag;
use crate::server::{connection::Connection, connection_handler::client_handler::ClientHandler};

use crate::server::data_structures::*;

impl<C: Connection> ClientHandler<C> {
    pub fn can_name_channel(&mut self, channel: &str) -> bool {
        let exists_channel = self.database.contains_channel(channel);

        let is_priv_or_secret = self
            .database
            .channel_has_mode(channel, &ChannelFlag::Secret)
            || self
                .database
                .channel_has_mode(channel, &ChannelFlag::Private);

        let is_client_in_channel = self.is_in_channel(channel);

        exists_channel && (!is_priv_or_secret || is_client_in_channel)
    }

    pub fn can_list_channel(&self, channel: &str) -> bool {
        if self
            .database
            .channel_has_mode(channel, &ChannelFlag::Secret)
            && !self.is_in_channel(channel)
        {
            return false;
        }

        self.database.contains_channel(channel)
    }

    pub fn shares_channel_with(&self, client_info: &ClientInfo) -> bool {
        let client_channels = self
            .database
            .get_channels_for_client(&client_info.nickname())
            .unwrap();
        let own_channels = self
            .database
            .get_channels_for_client(&self.nickname)
            .unwrap();

        !client_channels
            .iter()
            .any(|channel| own_channels.contains(channel))
    }

    pub fn is_in_channel(&self, channel: &str) -> bool {
        self.database.is_client_in_channel(&self.nickname, channel)
    }

    pub fn client_matches_banmask(&self, nickname: &str, mask: &str) -> bool {
        let client = ok_or_return!(self.database.get_client_info(nickname), false);

        client.matches_banmask(mask)
    }
}

pub fn is_distributed_channel(channel: &str) -> bool {
    channel.starts_with(DISTRIBUTED_CHANNEL)
}