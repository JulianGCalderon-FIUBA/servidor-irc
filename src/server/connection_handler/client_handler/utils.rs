use crate::macros::ok_or_return;
use crate::server::connection::Connection;
use crate::server::connection_handler::ConnectionHandlerUtils;
use crate::server::consts::modes::{ChannelFlag, OPERATOR_SYMBOL, SPEAKER_SYMBOL};
use crate::server::data_structures::ClientInfo;

use super::ClientHandler;

impl<C: Connection> ConnectionHandlerUtils<C> for ClientHandler<C> {}

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

    pub fn get_client_role_in_channel(&self, channel: &str, nickname: &str) -> Option<char> {
        if self.database.is_channel_operator(channel, nickname) {
            return Some(OPERATOR_SYMBOL);
        }
        if self
            .database
            .channel_has_flag(channel, ChannelFlag::Moderated)
            && self.database.is_channel_speaker(channel, nickname)
        {
            return Some(SPEAKER_SYMBOL);
        }
        None
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

    pub(super) fn clients_in_no_channel(&self) -> Vec<ClientInfo> {
        let mut all_clients = self.database.get_all_clients();

        all_clients.retain(|client| {
            let channels = ok_or_return!(
                self.database.get_channels_for_client(&client.nickname()),
                false
            );

            if channels.is_empty() {
                return true;
            }

            !channels
                .iter()
                .any(|channel| self.is_visible_channel(channel))
        });

        all_clients
    }
}

pub fn collect_list(parameters: Option<&String>) -> Vec<String> {
    match parameters {
        Some(parameters) => parameters.split(',').map(|s| s.to_string()).collect(),
        None => vec![],
    }
}
