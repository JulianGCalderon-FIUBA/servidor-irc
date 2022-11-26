use std::sync::mpsc::Sender;

use crate::server::{connection::Connection, database::Database};

impl<C: Connection> Database<C> {
    pub fn handle_is_server_operator(&mut self, nickname: String, respond_to: Sender<bool>) {
        let is_server_operator = self.is_server_operator(&nickname);
        respond_to.send(is_server_operator).unwrap();
    }

    pub fn handle_contains_client(&self, nickname: String, respond_to: Sender<bool>) {
        let contains_client = self.contains_client(&nickname);
        respond_to.send(contains_client).unwrap();
    }

    pub fn handle_contains_channel(&self, channel: String, respond_to: Sender<bool>) {
        let contains_channel = self.contains_channel(&channel);
        respond_to.send(contains_channel).unwrap();
    }

    pub fn handle_is_client_in_channel(
        &self,
        nickname: String,
        channel: String,
        respond_to: Sender<bool>,
    ) {
        let is_client_in_channel = self.is_client_in_channel(&nickname, &channel);
        respond_to.send(is_client_in_channel).unwrap();
    }

    pub fn handle_are_credentials_valid(
        &self,
        username: String,
        password: String,
        respond_to: Sender<bool>,
    ) {
        let are_credentials_valid = self.are_credentials_valid(&username, &password);
        respond_to.send(are_credentials_valid).unwrap();
    }

    pub fn handle_is_channel_speaker(
        &self,
        channel: String,
        nickname: String,
        respond_to: Sender<bool>,
    ) {
        let is_speaker = self.is_channel_speaker(&channel, &nickname);
        respond_to.send(is_speaker).unwrap();
    }

    pub fn handle_is_channel_operator(
        &self,
        channel: String,
        nickname: String,
        respond_to: Sender<bool>,
    ) {
        let is_channel_operator = self.is_channel_operator(&channel, &nickname);
        respond_to.send(is_channel_operator).unwrap();
    }

    pub fn handle_clients_matches_banmask(
        &mut self,
        nickname: String,
        banmask: String,
        respond_to: Sender<bool>,
    ) {
        let matches_banmask = self.client_matches_banmask(&nickname, &banmask);
        respond_to.send(matches_banmask).unwrap();
    }

    pub fn handle_is_local_client(&self, nickname: String, respond_to: Sender<bool>) {
        let is_local = self.is_local_client(&nickname);
        respond_to.send(is_local).unwrap();
    }

    pub fn handle_contains_server(&self, servername: String, respond_to: Sender<bool>) {
        let contains_server = self.contains_server(&servername);
        respond_to.send(contains_server).unwrap();
    }
}

impl<C: Connection> Database<C> {
    fn are_credentials_valid(&self, username: &str, password: &str) -> bool {
        if let Some(real_password) = self.credentials.get(username) {
            return password == real_password;
        }

        false
    }

    fn is_server_operator(&mut self, nickname: &str) -> bool {
        if let Some(client) = self.get_client_info(nickname) {
            return client.operator;
        }

        false
    }

    fn contains_client(&self, nickname: &str) -> bool {
        if self.local_clients.contains_key(nickname) {
            return true;
        }
        if self.external_clients.contains_key(nickname) {
            return true;
        }

        false
    }

    fn is_local_client(&self, nickname: &str) -> bool {
        self.local_clients.contains_key(nickname)
    }

    fn contains_channel(&self, channel: &str) -> bool {
        self.channels.contains_key(channel)
    }

    fn is_client_in_channel(&self, nickname: &str, channel: &str) -> bool {
        if let Some(channel) = self.channels.get(channel) {
            return channel.is_member(nickname);
        }

        false
    }

    fn is_channel_operator(&self, channel: &str, nickname: &str) -> bool {
        if let Some(channel) = self.channels.get(channel) {
            return channel.is_operator(nickname);
        }
        false
    }

    fn client_matches_banmask(&mut self, nickname: &str, banmask: &str) -> bool {
        if let Some(client) = self.get_client_info(nickname) {
            return client.matches_banmask(banmask);
        }

        false
    }

    fn is_channel_speaker(&self, channel: &str, nickname: &str) -> bool {
        if let Some(channel) = self.channels.get(channel) {
            return channel.is_speaker(nickname);
        }
        false
    }

    fn contains_server(&self, servername: &str) -> bool {
        self.immediate_servers.contains_key(servername)
            || self.distant_servers.contains_key(servername)
    }
}
