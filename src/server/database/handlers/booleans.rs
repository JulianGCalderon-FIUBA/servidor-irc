use std::sync::mpsc::Sender;

use crate::macros::{ok_or_return, some_or_return};
use crate::server::{connection::Connection, database::Database};

impl<C: Connection> Database<C> {
    pub fn handle_is_server_operator(&mut self, nickname: String, respond_to: Sender<bool>) {
        let is_server_operator = self.is_server_operator(nickname);
        respond_to.send(is_server_operator).unwrap();
    }

    pub fn handle_contains_client(&self, nickname: String, respond_to: Sender<bool>) {
        let contains_client = self.contains_client(nickname);
        respond_to.send(contains_client).unwrap();
    }

    pub fn handle_is_local_client(&self, nickname: String, respond_to: Sender<bool>) {
        let is_local = self.is_local_client(nickname);
        respond_to.send(is_local).unwrap();
    }

    pub fn handle_contains_channel(&self, channel: String, respond_to: Sender<bool>) {
        let contains_channel = self.contains_channel(channel);
        respond_to.send(contains_channel).unwrap();
    }

    pub fn handle_is_client_in_channel(
        &self,
        nickname: String,
        channel: String,
        respond_to: Sender<bool>,
    ) {
        let is_client_in_channel = self.is_client_in_channel(nickname, channel);
        respond_to.send(is_client_in_channel).unwrap();
    }
    pub fn handle_is_channel_speaker(
        &self,
        channel: String,
        nickname: String,
        respond_to: Sender<bool>,
    ) {
        let is_speaker = self.is_channel_speaker(channel, nickname);
        respond_to.send(is_speaker).unwrap();
    }
    pub fn handle_is_channel_operator(
        &self,
        channel: String,
        nickname: String,
        respond_to: Sender<bool>,
    ) {
        let is_channel_operator = self.is_channel_operator(channel, nickname);
        respond_to.send(is_channel_operator).unwrap();
    }

    pub fn handle_are_credentials_valid(
        &self,
        username: String,
        password: String,
        respond_to: Sender<bool>,
    ) {
        let are_credentials_valid = self.are_credentials_valid(username, password);
        respond_to.send(are_credentials_valid).unwrap();
    }

    pub fn handle_contains_server(&self, servername: String, respond_to: Sender<bool>) {
        let contains_server = self.contains_server(servername);
        respond_to.send(contains_server).unwrap();
    }

    pub fn handle_is_immediate_server(&self, server: String, respond_to: Sender<bool>) {
        let response = self.is_immediate_server(&server);
        respond_to.send(response).unwrap();
    }
}

impl<C: Connection> Database<C> {
    fn is_immediate_server(&self, server: &str) -> bool {
        self.immediate_servers.contains_key(server)
    }
    fn are_credentials_valid(&self, username: String, password: String) -> bool {
        let real_password = some_or_return!(self.credentials.get(&username), false);
        &password == real_password
    }

    fn is_server_operator(&mut self, nickname: String) -> bool {
        let client = ok_or_return!(self.get_client_info(&nickname), false);

        client.operator
    }

    pub fn contains_client(&self, nickname: String) -> bool {
        self.local_clients.contains_key(&nickname) || self.external_clients.contains_key(&nickname)
    }

    fn is_local_client(&self, nickname: String) -> bool {
        self.local_clients.contains_key(&nickname)
    }

    fn contains_channel(&self, channel: String) -> bool {
        self.channels.contains_key(&channel)
    }

    fn is_client_in_channel(&self, nickname: String, channel: String) -> bool {
        let channel = some_or_return!(self.channels.get(&channel), false);
        channel.is_member(&nickname)
    }

    fn is_channel_operator(&self, channel: String, nickname: String) -> bool {
        let channel = some_or_return!(self.channels.get(&channel), false);
        channel.is_operator(&nickname)
    }

    fn is_channel_speaker(&self, channel: String, nickname: String) -> bool {
        let channel = some_or_return!(self.channels.get(&channel), false);
        channel.is_speaker(&nickname)
    }

    fn contains_server(&self, servername: String) -> bool {
        self.immediate_servers.contains_key(&servername)
            || self.distant_servers.contains_key(&servername)
    }
}
