use crate::server::consts::modes::*;
use crate::server::{connection::Connection, connection_handler::client_handler::ClientHandler};

use crate::server::data_structures::*;

impl<C: Connection> ClientHandler<C> {
    pub(super) fn channels_to_list(&mut self, channels: Option<&String>) -> Vec<String> {
        if channels.is_none() {
            return self.database.get_all_channels();
        }

        collect_list(channels)
    }

    pub(super) fn clients_for_default_who(&self) -> Vec<ClientInfo> {
        self.database
            .get_all_clients()
            .into_iter()
            .filter(|client_info| self.shares_channel_with(client_info))
            .collect()
    }

    pub fn append_channel_role(&mut self, channels: &mut Vec<String>, nickname: &str) {
        for channel in channels {
            if self.database.is_channel_operator(channel, nickname) {
                channel.insert(0, OPERATOR_SYMBOL);
            } else if self
                .database
                .channel_has_mode(channel, &ChannelFlag::Moderated)
                && self.database.is_channel_speaker(channel, nickname)
            {
                channel.insert(0, SPEAKER_SYMBOL);
            }
        }
    }

    pub(super) fn get_clients_for_mask(&self, mask: &str) -> Vec<ClientInfo> {
        let clients = self.database.get_all_clients();

        clients
            .into_iter()
            .filter(|client| client.matches_mask(mask))
            .collect()
    }

    pub(super) fn get_clients_for_nickmask(&self, nickmask: &str) -> Vec<ClientInfo> {
        let clients = self.database.get_all_clients();

        clients
            .into_iter()
            .filter(|client| client.matches_nickmask(nickmask))
            .collect()
    }
}

pub fn collect_list(parameters: Option<&String>) -> Vec<String> {
    match parameters {
        Some(parameters) => parameters.split(',').map(|s| s.to_string()).collect(),
        None => vec![],
    }
}
