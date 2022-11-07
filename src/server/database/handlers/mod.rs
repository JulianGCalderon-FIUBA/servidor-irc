use std::sync::mpsc::Sender;
use std::{io, rc::Rc};

/// This module contains the functions with the logic regarding the database's functionalities.
mod logic;
/// This module contains extra functions that are useful for the database to complete it's tasks.
mod utils;

use crate::server::client_trait::ClientTrait;

use super::{ClientInfo, Database};

impl<T: ClientTrait> Database<T> {
    /// Returns response to IsServerOperator request.
    pub fn handle_is_server_operator(&mut self, nickname: &str, sender: Sender<bool>) {
        let response = self.is_server_operator(nickname);
        sender.send(response).unwrap();
    }

    /// Returns response to GetStream request.
    pub fn handle_get_stream_request(&self, nickname: &str, sender: Sender<io::Result<T>>) {
        let response = self.get_stream(nickname);
        sender.send(response).unwrap();
    }

    /// Returns response to ContainsClient request.
    pub fn handle_contains_client_request(&self, nickname: &str, sender: Sender<bool>) {
        let response = self.contains_client(nickname);
        sender.send(response).unwrap();
    }

    /// Returns response to ContainsChannel request.
    pub fn handle_contains_channel(&self, channel: &str, sender: Sender<bool>) {
        let response = self.contains_channel(channel);
        sender.send(response).unwrap();
    }

    /// Returns response to IsClientInChannel request.
    pub fn handle_is_client_in_channel(&self, nickname: &str, channel: &str, sender: Sender<bool>) {
        let response = self.is_client_in_channel(nickname, channel);
        sender.send(response).unwrap();
    }

    /// Returns response to GetClientsForChannel request.
    pub fn handle_get_clients_for_channel(&self, channel: &str, sender: Sender<Vec<String>>) {
        let response = self.get_clients_for_channel(channel);
        sender.send(response).unwrap();
    }

    /// Returns response to GetAllClients request.
    pub fn handle_get_all_clients(&self, sender: Sender<Vec<ClientInfo>>) {
        sender.send(self.get_all_clients()).unwrap();
    }

    /// Returns response to GetClientsForMask request.
    pub fn handle_get_clients_for_mask(&self, mask: &str, sender: Sender<Vec<ClientInfo>>) {
        let response = self.get_clients_for_mask(mask);
        sender.send(response).unwrap();
    }

    /// Returns response to GetClientsForNickMask request.
    pub fn handle_get_clients_for_nickmask(&self, mask: &str, sender: Sender<Vec<ClientInfo>>) {
        let response = self.get_clients_for_nickmask(mask);
        sender.send(response).unwrap();
    }

    /// Returns response to GetAllChannels request.
    pub fn handle_get_all_channels(&self, sender: Sender<Vec<String>>) {
        let response = self.get_channels();
        sender.send(response).unwrap();
    }

    /// Returns response to GetChannelsForClient request.
    pub fn handle_get_channels_for_client(&self, nickname: &str, sender: Sender<Vec<String>>) {
        let response = self.get_channels_for_client(nickname);
        sender.send(response).unwrap();
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
        let response = self.are_credentials_valid(username, password);
        sender.send(response).unwrap();
    }
}
