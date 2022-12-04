use crate::macros::ok_or_return;
use crate::server::consts::channel::{DISTRIBUTED_CHANNEL, LOCAL_CHANNEL};
use crate::server::consts::channel_flag::ChannelFlag;
use crate::server::{connection::Connection, connection_handler::client_handler::ClientHandler};

use crate::server::data_structures::*;

impl<C: Connection> ClientHandler<C> {
    pub fn can_name_channel(&mut self, channel: &str) -> bool {
        let exists_channel = self.database.contains_channel(channel);

        exists_channel && self.is_visible_channel(channel)
    }

    pub fn is_visible_channel(&self, channel: &str) -> bool {
        let private = self
            .database
            .channel_has_flag(channel, ChannelFlag::Private);

        let secret = self.database.channel_has_flag(channel, ChannelFlag::Secret);

        if !(private || secret) {
            return true;
        }

        self.is_in_channel(channel)
    }

    pub fn can_list_channel(&self, channel: &str) -> bool {
        if self.database.channel_has_flag(channel, ChannelFlag::Secret)
            && !self.is_in_channel(channel)
        {
            return false;
        }

        self.database.contains_channel(channel)
    }

    pub fn shares_channel_with(&self, client_info: &ClientInfo) -> bool {
        let client_channels = ok_or_return!(
            self.database
                .get_channels_for_client(&client_info.nickname()),
            false
        );
        let own_channels =
            ok_or_return!(self.database.get_channels_for_client(&self.nickname), false);

        !client_channels
            .iter()
            .any(|channel| own_channels.contains(channel))
    }

    pub fn is_in_channel(&self, channel: &str) -> bool {
        self.database.is_client_in_channel(channel, &self.nickname)
    }

    pub fn client_matches_banmask(&self, nickname: &str, mask: &str) -> bool {
        let client = ok_or_return!(self.database.get_client_info(nickname), false);

        client.matches_banmask(mask)
    }

    pub fn is_channel(&self, target: &str) -> bool {
        target.starts_with([DISTRIBUTED_CHANNEL, LOCAL_CHANNEL])
    }
}

pub fn is_distributed_channel(channel: &str) -> bool {
    channel.starts_with(DISTRIBUTED_CHANNEL)
}
