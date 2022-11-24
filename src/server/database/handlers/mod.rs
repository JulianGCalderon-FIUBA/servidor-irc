use std::cell::RefCell;
use std::sync::mpsc::Sender;
use std::{io, rc::Rc};

/// This module contains the functions with the logic regarding the database's functionalities.
mod logic;

use crate::server::connection::Connection;
<<<<<<< HEAD
use crate::server::{data_structures::*, debug_print};
=======
use crate::server::consts::modes::ChannelFlag;
use crate::server::data_structures::*;
>>>>>>> feature/server_handler

use super::Database;

impl<C: Connection> Database<C> {
    /// Returns response to IsServerOperator request.
    pub fn handle_is_server_operator(&mut self, nickname: &str, respond_to: Sender<bool>) {
        let is_server_operator = self.is_server_operator(nickname);
        respond_to.send(is_server_operator).unwrap();
    }

    pub fn handle_add_client(&mut self, client: Client<C>) {
        debug_print!("Adding client {:?}", client.get_info());

        let nickname = client.nickname();
        let client = Rc::new(RefCell::new(client));
        self.clients.insert(nickname, client);
    }

    /// Sets client as server operator.
    pub fn handle_set_server_operator(&mut self, nickname: &str) {
        if let Some(client) = self.clients.get_mut(nickname) {
            debug_print!("Setting {} as server operator", nickname);

            client.borrow_mut().set_server_operator();
        }
    }

    /// Adds client to channel.
    pub fn add_client_to_channel(&mut self, nickname: &str, channel_name: &str) {
        let channel: Option<&mut Channel<C>> = self.channels.get_mut(&channel_name.to_string());
        if let Some(client) = self.clients.get(nickname) {
            debug_print!("Adding {} to channel {}", nickname, channel_name);

            let client_rc = client.clone();

            match channel {
                Some(channel) => channel.add_client(client_rc),
                None => {
                    let new_channel = Channel::new(channel_name.to_string(), client_rc);
                    self.channels.insert(channel_name.to_string(), new_channel);
                }
            }
        }
    }

    /// Removes client from channel.
    pub fn remove_client_from_channel(&mut self, nickname: &str, channel_name: &str) {
        if let Some(channel) = self.channels.get_mut(&channel_name.to_string()) {
            debug_print!("Removing {} from channel {}", nickname, channel_name);

            channel.remove_client(nickname);
            if channel.get_clients().is_empty() {
                self.channels.remove(channel_name);
            }
        }
    }

    /// Disconnects client from server, removing it from Database.
    pub fn disconnect_client(&mut self, nickname: &str) {
        if let Some(client) = self.clients.get_mut(nickname) {
            debug_print!("Disconnecting client {} ", nickname);

            client.borrow_mut().disconnect();
        }
    }

    pub fn set_channel_topic(&mut self, channel_name: &str, topic: &str) {
        if let Some(channel) = self.channels.get_mut(channel_name) {
            debug_print!("Setting {channel_name}'s topic to {topic}");

            channel.set_topic(topic);
        }
    }

    /// Returns response to GetStream request.
    pub fn handle_get_stream_request(
        &self,
        nickname: &str,
        respond_to: Sender<Option<io::Result<C>>>,
    ) {
        let stream = self.get_stream(nickname);
        respond_to.send(stream).unwrap();
    }

    /// Returns response to ContainsClient request.
    pub fn handle_contains_client_request(&self, nickname: &str, respond_to: Sender<bool>) {
        let contains_client = self.contains_client(nickname);
        respond_to.send(contains_client).unwrap();
    }

    /// Returns response to ContainsChannel request.
    pub fn handle_contains_channel(&self, channel: &str, respond_to: Sender<bool>) {
        let contains_channel = self.contains_channel(channel);
        respond_to.send(contains_channel).unwrap();
    }

    /// Returns response to IsClientInChannel request.
    pub fn handle_is_client_in_channel(
        &self,
        nickname: &str,
        channel: &str,
        respond_to: Sender<bool>,
    ) {
        let is_client_in_channel = self.is_client_in_channel(nickname, channel);
        respond_to.send(is_client_in_channel).unwrap();
    }

    /// Returns response to GetClientsForChannel request.
    pub fn handle_get_clients_for_channel(&self, channel: &str, respond_to: Sender<Vec<String>>) {
        let clients = self.get_clients_for_channel(channel);
        respond_to.send(clients).unwrap();
    }

    /// Returns response to GetAllClients request.
    pub fn handle_get_all_clients(&self, respond_to: Sender<Vec<ClientInfo>>) {
        let clients = self.get_all_clients();
        respond_to.send(clients).unwrap();
    }

    /// Returns response to GetClientsForMask request.
    pub fn handle_get_clients_for_mask(&self, mask: &str, respond_to: Sender<Vec<ClientInfo>>) {
        let clients = self.get_clients_for_mask(mask);
        respond_to.send(clients).unwrap();
    }

    /// Returns response to GetClientsForNickMask request.
    pub fn handle_get_clients_for_nickmask(&self, mask: &str, respond_to: Sender<Vec<ClientInfo>>) {
        let clients = self.get_clients_for_nickmask(mask);
        respond_to.send(clients).unwrap();
    }

    /// Returns response to GetAllChannels request.
    pub fn handle_get_all_channels(&self, respond_to: Sender<Vec<String>>) {
        let channels = self.get_channels();
        respond_to.send(channels).unwrap();
    }

    /// Returns response to GetChannelsForClient request.
    pub fn handle_get_channels_for_client(&self, nickname: &str, respond_to: Sender<Vec<String>>) {
        let channels = self.get_channels_for_client(nickname);
        respond_to.send(channels).unwrap();
    }

    /// Returns response to UpdateNickname request.
    pub fn handle_update_nickname(&mut self, old_nickname: &str, new_nickname: &str) {
        if let Some(client) = self.clients.get_mut(old_nickname) {
            debug_print!("Updating nickname from {old_nickname} to {new_nickname}");

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
        respond_to: Sender<bool>,
    ) {
        let are_credentials_valid = self.are_credentials_valid(username, password);
        respond_to.send(are_credentials_valid).unwrap();
    }

    pub fn handle_set_away_message(&self, message: &Option<String>, nickname: &str) {
        if let Some(client) = self.clients.get(nickname) {
            debug_print!("Setting {nickname}'s away message to {message:?}");

            client.borrow_mut().set_away_message(message.to_owned());
        }
    }

    pub fn handle_get_away_message(&self, nickname: &str, respond_to: Sender<Option<String>>) {
        let message = self.get_away_message(nickname);
        respond_to.send(message).unwrap();
    }
    pub fn handle_get_channel_topic(&self, channel: &str, respond_to: Sender<Option<String>>) {
        let topic = self.get_channel_topic(channel);
        respond_to.send(topic).unwrap();
    }

    pub fn handle_set_channel_key(&mut self, channel_name: String, key: Option<String>) {
        if let Some(channel) = self.channels.get_mut(&channel_name) {
            debug_print!("Setting {channel_name}'s key to {key:?}");

            channel.set_key(key);
        }
    }

    pub fn handle_get_channel_key(&self, channel: String, respond_to: Sender<Option<String>>) {
        let key = self.get_channel_key(channel);
        respond_to.send(key).unwrap();
    }

<<<<<<< HEAD
    pub fn handle_set_mode(&mut self, channel_name: String, mode: char) {
        if let Some(channel) = self.channels.get_mut(&channel_name) {
            debug_print!("Setting {channel_name}'s mode {mode:?}");

            channel.set_mode(mode);
        }
    }

    pub fn handle_unset_mode(&mut self, channel_name: String, mode: char) {
        if let Some(channel) = self.channels.get_mut(&channel_name) {
            debug_print!("Unsetting {channel_name}'s mode {mode:?}");

            channel.unset_mode(mode);
=======
    pub fn handle_set_mode(&mut self, channel: String, flag: ChannelFlag) {
        if let Some(channel) = self.channels.get_mut(&channel) {
            channel.set_mode(flag);
        }
    }

    pub fn handle_unset_mode(&mut self, channel: String, flag: ChannelFlag) {
        if let Some(channel) = self.channels.get_mut(&channel) {
            channel.unset_mode(flag);
>>>>>>> feature/server_handler
        }
    }

    pub fn handle_channel_has_mode(
        &self,
        channel: String,
        flag: ChannelFlag,
        respond_to: Sender<bool>,
    ) {
        let has_mode = self.channel_has_mode(channel, flag);
        respond_to.send(has_mode).unwrap();
    }

    pub fn handle_set_channel_limit(&mut self, channel_name: String, limit: Option<usize>) {
        if let Some(channel) = self.channels.get_mut(&channel_name) {
            debug_print!("Setting {channel_name}'s limit to {limit:?}");

            channel.set_limit(limit)
        }
    }

    pub fn handle_get_channel_limit(&self, channel: String, respond_to: Sender<Option<usize>>) {
        let limit = self.get_channel_limit(channel);
        respond_to.send(limit).unwrap();
    }

    pub fn handle_add_channop(&mut self, channel_name: String, nickname: String) {
        if let Some(channel) = self.channels.get_mut(&channel_name) {
            debug_print!("Setting {nickname} as operator of {channel_name}");

            channel.add_operator(nickname);
        }
    }

    pub fn handle_remove_channop(&mut self, channel_name: String, nickname: String) {
        if let Some(channel) = self.channels.get_mut(&channel_name) {
            debug_print!("Setting {nickname} as operator of {channel_name}");

            channel.remove_operator(nickname);
        }
    }

    pub fn handle_add_speaker(&mut self, channel_name: String, nickname: String) {
        if let Some(channel) = self.channels.get_mut(&channel_name) {
            debug_print!("Setting {nickname} as speaker of {channel_name}");

            channel.add_speaker(nickname);
        }
    }

    pub fn handle_remove_speaker(&mut self, channel_name: String, nickname: String) {
        if let Some(channel) = self.channels.get_mut(&channel_name) {
            debug_print!("Unsetting {nickname} as speaker of {channel_name}");

            channel.remove_speaker(nickname);
        }
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

    pub fn handle_add_channel_banmask(&mut self, channel_name: String, mask: String) {
        if let Some(channel) = self.channels.get_mut(&channel_name) {
            debug_print!("Adding banmask {mask} to {channel_name}");

            channel.add_banmask(mask);
        }
    }

    pub fn handle_get_channel_banmask(&self, channel: String, respond_to: Sender<Vec<String>>) {
        let banmask = self.get_channel_banmask(channel);
        respond_to.send(banmask).unwrap();
    }

    pub fn handle_remove_channel_banmask(&mut self, channel_name: String, mask: String) {
        if let Some(channel) = self.channels.get_mut(&channel_name) {
            debug_print!("Removing banmask {mask} from {channel_name}");

            channel.remove_banmask(mask);
        }
    }

    pub fn handle_is_channel_operator(
        &self,
        channel: &str,
        nickname: &str,
        respond_to: Sender<bool>,
    ) {
        let is_channel_operator = self.is_channel_operator(channel, nickname);
        respond_to.send(is_channel_operator).unwrap();
    }

    pub fn handle_clients_matches_banmask(
        &self,
        nickname: &str,
        banmask: &str,
        respond_to: Sender<bool>,
    ) {
        let matches_banmask = self.client_matches_banmask(nickname, banmask);
        respond_to.send(matches_banmask).unwrap();
    }

    pub fn handle_add_server(&mut self, server: ExternalServer<C>) {
        let servername = server.servername();
        debug_print!("Adding external server {servername}");

        self.servers.insert(servername, server);
    }

    pub fn handle_contains_server(&self, servername: &str, respond_to: Sender<bool>) {
        let contains_server = self.contains_server(servername);
        respond_to.send(contains_server).unwrap();
    }

    pub fn handle_add_external_client(&mut self, servername: &str, client: ExternalClient) {
        if let Some(server) = self.servers.get_mut(servername) {
            debug_print!(
                "Adding external client {} to server {servername}",
                client.nickname()
            );
            server.add_client(client);
        }
    }

    pub fn handle_get_servername(&self, respond_to: Sender<String>) {
        respond_to.send(self.servername.clone()).unwrap();
    }

    pub fn handle_get_serverinfo(&self, respond_to: Sender<String>) {
        respond_to.send(self.serverinfo.clone()).unwrap();
    }

    pub fn handle_get_channel_config(
        &self,
        channel: String,
        respond_to: Sender<Option<ChannelConfig>>,
    ) {
        let channel_config = self.get_channel_config(&channel);
        respond_to.send(channel_config).unwrap();
    }
}
