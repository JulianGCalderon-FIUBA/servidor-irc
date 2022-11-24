use std::io;

use crate::server::connection::Connection;
use crate::server::consts::modes::ChannelFlag;
use crate::server::data_structures::{ChannelConfig, Client, ClientInfo};
use crate::server::database::Database;

impl<C: Connection> Database<C> {
    /// Adds client to Database.

    /// Verifies if operator credentials are valid.
    pub fn are_credentials_valid(&self, username: &str, password: &str) -> bool {
        if let Some(real_password) = self.credentials.get(username) {
            return password == real_password;
        }

        false
    }

    /// Returns if client is server operator.
    pub fn is_server_operator(&mut self, nickname: &str) -> bool {
        if let Some(client) = self.clients.get(nickname) {
            return client.borrow_mut().is_server_operator();
        }

        false
    }

    /// Returns the client's stream or error if client is disconnected.
    pub fn get_stream(&self, nickname: &str) -> Option<io::Result<C>> {
        if let Some(client) = self.clients.get(nickname) {
            return client.borrow().get_stream();
        }

        for server in self.servers.values() {
            if server.contains_client(nickname) {
                return server.get_stream();
            }
        }

        None
    }

    /// Returns if Database contains client.
    pub fn contains_client(&self, nickname: &str) -> bool {
        if self.clients.contains_key(nickname) {
            return true;
        }

        self.servers
            .values()
            .any(|server| server.contains_client(nickname))
    }

    /// Returns if Database contains channel.
    pub fn contains_channel(&self, channel: &str) -> bool {
        self.channels.contains_key(channel)
    }

    /// Returns if client is in channel.
    pub fn is_client_in_channel(&self, nickname: &str, channel: &str) -> bool {
        if let Some(channel) = self.channels.get(channel) {
            return channel.contains_client(nickname);
        }

        false
    }

    /// Returns array of clients for channel.
    pub fn get_clients_for_channel(&self, channel: &str) -> Vec<String> {
        let channel_info = self.channels.get(channel);

        match channel_info {
            Some(channel_info) => channel_info.get_clients(),
            None => vec![],
        }
    }

    /// Returns array with ClientInfo for connected clients.
    pub fn get_all_clients(&self) -> Vec<ClientInfo> {
        let mut local_clients: Vec<ClientInfo> = self
            .clients
            .values()
            .map(|client| client.borrow().get_info())
            .collect();

        for server in self.servers.values() {
            local_clients.append(&mut server.get_all_clients());
        }

        local_clients
    }

    /// Returns array with ClientInfo for channels that match mask.
    pub fn get_clients_for_mask(&self, mask: &str) -> Vec<ClientInfo> {
        self.filtered_clients(mask, Client::matches_mask)
    }

    /// Returns array with ClientInfo for channels that match nick mask.
    pub fn get_clients_for_nickmask(&self, mask: &str) -> Vec<ClientInfo> {
        self.filtered_clients(mask, Client::matches_nickmask)
    }

    /// Returns array of channels in Database.
    pub fn get_channels(&self) -> Vec<String> {
        self.channels.keys().cloned().collect()
    }

    /// Returns array of channels the client is connected to.
    pub fn get_channels_for_client(&self, nickname: &str) -> Vec<String> {
        let mut channels = vec![];

        for (channel_name, channel) in self.channels.iter() {
            let clients = channel.get_clients();
            if clients.contains(&nickname.to_string()) {
                channels.push(channel_name.clone());
            }
        }
        channels
    }

    pub fn get_away_message(&self, nickname: &str) -> Option<String> {
        if let Some(client) = self.clients.get(nickname) {
            return client.borrow().away_message();
        }

        None
    }

    fn filtered_clients(
        &self,
        mask: &str,
        filter: fn(&Client<C>, &str) -> bool,
    ) -> Vec<ClientInfo> {
        self.clients
            .values()
            .filter(|client| filter(&client.borrow(), mask))
            .map(|client| client.borrow().get_info())
            .collect()
    }

    pub fn get_channel_topic(&self, channel: &str) -> Option<String> {
        if let Some(channel) = self.channels.get(channel) {
            return channel.get_topic();
        }
        None
    }

    pub fn get_channel_key(&self, channel: String) -> Option<String> {
        if let Some(channel) = self.channels.get(&channel) {
            return channel.get_key();
        }
        None
    }

    pub fn channel_has_mode(&self, channel: String, mode: ChannelFlag) -> bool {
        if let Some(channel) = self.channels.get(&channel) {
            return channel.has_mode(mode);
        }
        false
    }

    pub fn get_channel_limit(&self, channel: String) -> Option<usize> {
        if let Some(channel) = self.channels.get(&channel) {
            return channel.get_limit();
        }
        None
    }

    pub fn is_channel_speaker(&self, channel: String, nickname: String) -> bool {
        if let Some(channel) = self.channels.get(&channel) {
            return channel.is_speaker(nickname);
        }
        false
    }

    pub fn get_channel_banmask(&self, channel: String) -> Vec<String> {
        if let Some(channel) = self.channels.get(&channel) {
            return channel.get_banmasks();
        }
        vec![]
    }

    pub fn is_channel_operator(&self, channel: &str, nickname: &str) -> bool {
        if let Some(channel) = self.channels.get(channel) {
            return channel.is_operator(nickname);
        }
        false
    }

    pub fn client_matches_banmask(&self, nickname: &str, banmask: &str) -> bool {
        if let Some(client) = self.clients.get(nickname) {
            return client.borrow().matches_banmask(banmask);
        }

        false
    }

    pub fn contains_server(&self, servername: &str) -> bool {
        self.servers.contains_key(servername)
    }

    pub fn get_channel_config(&self, channel: &str) -> Option<ChannelConfig> {
        if let Some(channel) = self.channels.get(channel) {
            return channel.get_config();
        }
        None
    }

    pub fn get_server_stream(&self, server: &str) -> Option<Result<C, std::io::Error>> {
        if let Some(server) = self.servers.get(server) {
            return server.get_stream();
        }

        None
    }

    pub fn get_all_servers(&self) -> Vec<String> {
        todo!()
    }

    pub fn get_local_clients_for_channel(&self, channel: &str) -> Vec<String> {
        let channel_info = self.channels.get(channel);

        match channel_info {
            Some(channel_info) => channel_info.get_local_clients(),
            None => vec![],
        }
    }
}
