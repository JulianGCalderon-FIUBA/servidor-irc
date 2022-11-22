use std::sync::mpsc::Sender;
use std::{io, rc::Rc};

/// This module contains the functions with the logic regarding the database's functionalities.
mod logic;

use crate::server::connection::Connection;
use crate::server::data_structures::*;

use super::Database;

impl<C: Connection> Database<C> {
    /// Returns response to IsServerOperator request.
    pub fn handle_is_server_operator(&mut self, nickname: &str, sender: Sender<bool>) {
        let is_server_operator = self.is_server_operator(nickname);
        sender.send(is_server_operator).unwrap();
    }

    /// Returns response to GetStream request.
    pub fn handle_get_stream_request(&self, nickname: &str, sender: Sender<Option<io::Result<C>>>) {
        let stream = self.get_stream(nickname);
        sender.send(stream).unwrap();
    }

    /// Returns response to ContainsClient request.
    pub fn handle_contains_client_request(&self, nickname: &str, sender: Sender<bool>) {
        let contains_client = self.contains_client(nickname);
        sender.send(contains_client).unwrap();
    }

    /// Returns response to ContainsChannel request.
    pub fn handle_contains_channel(&self, channel: &str, sender: Sender<bool>) {
        let contains_channel = self.contains_channel(channel);
        sender.send(contains_channel).unwrap();
    }

    /// Returns response to IsClientInChannel request.
    pub fn handle_is_client_in_channel(&self, nickname: &str, channel: &str, sender: Sender<bool>) {
        let is_client_in_channel = self.is_client_in_channel(nickname, channel);
        sender.send(is_client_in_channel).unwrap();
    }

    /// Returns response to GetClientsForChannel request.
    pub fn handle_get_clients_for_channel(&self, channel: &str, sender: Sender<Vec<String>>) {
        let clients = self.get_clients_for_channel(channel);
        sender.send(clients).unwrap();
    }

    /// Returns response to GetAllClients request.
    pub fn handle_get_all_clients(&self, sender: Sender<Vec<ClientInfo>>) {
        let clients = self.get_all_clients();
        sender.send(clients).unwrap();
    }

    /// Returns response to GetClientsForMask request.
    pub fn handle_get_clients_for_mask(&self, mask: &str, sender: Sender<Vec<ClientInfo>>) {
        let clients = self.get_clients_for_mask(mask);
        sender.send(clients).unwrap();
    }

    /// Returns response to GetClientsForNickMask request.
    pub fn handle_get_clients_for_nickmask(&self, mask: &str, sender: Sender<Vec<ClientInfo>>) {
        let clients = self.get_clients_for_nickmask(mask);
        sender.send(clients).unwrap();
    }

    /// Returns response to GetAllChannels request.
    pub fn handle_get_all_channels(&self, sender: Sender<Vec<String>>) {
        let channels = self.get_channels();
        sender.send(channels).unwrap();
    }

    /// Returns response to GetChannelsForClient request.
    pub fn handle_get_channels_for_client(&self, nickname: &str, sender: Sender<Vec<String>>) {
        let channels = self.get_channels_for_client(nickname);
        sender.send(channels).unwrap();
    }

    /// Returns response to UpdateNickname request.
    pub fn handle_update_nickname(&mut self, old_nickname: &str, new_nickname: &str) {
        if let Some(client) = self.clients.get_mut(old_nickname) {
            let client = Rc::get_mut(client).unwrap();
            client
                .borrow_mut()
                .update_nickname(new_nickname.to_string());

            let client = self.clients.remove(old_nickname).unwrap();
            self.clients.insert(new_nickname.to_string(), client);
        }
    }

    pub fn handle_are_credentials_valid(
        &self,
        username: &str,
        password: &str,
        sender: Sender<bool>,
    ) {
        let are_credentials_valid = self.are_credentials_valid(username, password);
        sender.send(are_credentials_valid).unwrap();
    }

    pub fn handle_set_away_message(&self, message: &Option<String>, nickname: &str) {
        if let Some(client) = self.clients.get(nickname) {
            client.borrow_mut().set_away_message(message.to_owned());
        }
    }

    pub fn handle_get_away_message(&self, nickname: &str, sender: Sender<Option<String>>) {
        let message = self.get_away_message(nickname);
        sender.send(message).unwrap();
    }
    pub fn handle_get_channel_topic(&self, channel: &str, sender: Sender<Option<String>>) {
        let topic = self.get_channel_topic(channel);
        sender.send(topic).unwrap();
    }

    pub fn handle_set_channel_key(&mut self, channel: String, key: Option<String>) {
        if let Some(channel) = self.channels.get_mut(&channel) {
            channel.set_key(key);
        }
    }

    pub fn handle_get_channel_key(&self, channel: String, sender: Sender<Option<String>>) {
        let key = self.get_channel_key(channel);
        sender.send(key).unwrap();
    }

    pub fn handle_set_mode(&mut self, channel: String, mode: char) {
        if let Some(channel) = self.channels.get_mut(&channel) {
            channel.set_mode(mode);
        }
    }

    pub fn handle_unset_mode(&mut self, channel: String, mode: char) {
        if let Some(channel) = self.channels.get_mut(&channel) {
            channel.unset_mode(mode);
        }
    }

    pub fn handle_channel_has_mode(&self, channel: String, mode: char, sender: Sender<bool>) {
        let has_mode = self.channel_has_mode(channel, mode);
        sender.send(has_mode).unwrap();
    }

    pub fn handle_set_channel_limit(&mut self, channel: String, limit: Option<usize>) {
        if let Some(channel) = self.channels.get_mut(&channel) {
            channel.set_limit(limit)
        }
    }

    pub fn handle_get_channel_limit(&self, channel: String, sender: Sender<Option<usize>>) {
        let limit = self.get_channel_limit(channel);
        sender.send(limit).unwrap();
    }

    pub fn handle_add_channop(&mut self, channel: String, nickname: String) {
        if let Some(channel) = self.channels.get_mut(&channel) {
            channel.add_operator(nickname);
        }
    }

    pub fn handle_remove_channop(&mut self, channel: String, nickname: String) {
        if let Some(channel) = self.channels.get_mut(&channel) {
            channel.remove_operator(nickname);
        }
    }

    pub fn handle_add_speaker(&mut self, channel: String, nickname: String) {
        if let Some(channel) = self.channels.get_mut(&channel) {
            channel.add_speaker(nickname);
        }
    }

    pub fn handle_remove_speaker(&mut self, channel: String, nickname: String) {
        if let Some(channel) = self.channels.get_mut(&channel) {
            channel.remove_speaker(nickname);
        }
    }

    pub fn handle_is_channel_speaker(
        &self,
        channel: String,
        nickname: String,
        sender: Sender<bool>,
    ) {
        let is_speaker = self.is_channel_speaker(channel, nickname);
        sender.send(is_speaker).unwrap();
    }

    pub fn handle_add_channel_banmask(&mut self, channel: String, mask: String) {
        if let Some(channel) = self.channels.get_mut(&channel) {
            channel.add_banmask(mask);
        }
    }

    pub fn handle_get_channel_banmask(&self, channel: String, sender: Sender<Vec<String>>) {
        let banmask = self.get_channel_banmask(channel);
        sender.send(banmask).unwrap();
    }

    pub fn handle_remove_channel_banmask(&mut self, channel: String, mask: String) {
        if let Some(channel) = self.channels.get_mut(&channel) {
            channel.remove_banmask(mask);
        }
    }

    // pub fn handle_get_all_channel_modes(&self, channel: String, sender: Sender<Vec<char>>) {
    //     let response = self.get_all_channel_modes(channel);
    //     sender.send(response).unwrap();
    // }
    pub fn handle_is_channel_operator(&self, channel: &str, nickname: &str, sender: Sender<bool>) {
        let is_channel_operator = self.is_channel_operator(channel, nickname);
        sender.send(is_channel_operator).unwrap();
    }

    pub fn handle_clients_matches_banmask(
        &self,
        nickname: &str,
        banmask: &str,
        sender: Sender<bool>,
    ) {
        let matches_banmask = self.client_matches_banmask(nickname, banmask);
        sender.send(matches_banmask).unwrap();
    }

    pub fn handle_add_server(&mut self, server: ExternalServer<C>) {
        let servername = server.servername();
        println!("Adding server named {servername}");

        self.servers.insert(servername, server);
    }

    pub fn handle_contains_server(&self, servername: &str, sender: Sender<bool>) {
        let contains_server = self.contains_server(servername);
        sender.send(contains_server).unwrap();
    }

    pub fn handle_add_external_client(&mut self, servername: &str, client: ExternalClient) {
        if let Some(server) = self.servers.get_mut(servername) {
            println!(
                "Adding external client {} to server {servername}",
                client.nickname(),
            );
            server.add_client(client);
        }
    }

    pub fn handle_get_servername(&self, sender: Sender<String>) {
        sender.send(self.servername.clone()).unwrap();
    }

    pub fn handle_get_serverinfo(&self, sender: Sender<String>) {
        sender.send(self.serverinfo.clone()).unwrap();
    }
}
