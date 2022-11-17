use crate::server::{
    connection::Connection,
    connection_handler::{
        client_handler::ClientHandler,
        consts::modes::{PRIVATE, SECRET},
    },
    database::ClientInfo,
};

impl<C: Connection> ClientHandler<C> {
    pub(super) fn can_name_channel(&mut self, channel: &str) -> bool {
        let exists_channel = self.database.contains_channel(channel);

        let is_priv_or_secret = self.database.channel_has_mode(channel, SECRET)
            || self.database.channel_has_mode(channel, PRIVATE);

        let is_client_in_channel = self.database.is_client_in_channel(&self.nickname, channel);

        exists_channel && (!is_priv_or_secret || is_client_in_channel)
    }

    pub(super) fn can_list_channel(&self, channel: &str) -> bool {
        if self.database.channel_has_mode(channel, SECRET)
            && !self.database.is_client_in_channel(&self.nickname, channel)
        {
            return false;
        }

        self.database.contains_channel(channel)
    }

    pub(super) fn shares_channel_with(&self, client_info: &ClientInfo) -> bool {
        let client_channels = self.database.get_channels_for_client(&client_info.nickname);
        let self_channels = self.database.get_channels_for_client(&self.nickname);

        !client_channels
            .iter()
            .any(|channel| self_channels.contains(channel))
    }
}
