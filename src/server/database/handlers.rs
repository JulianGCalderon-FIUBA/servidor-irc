use std::{io, sync::mpsc::Sender};

use crate::server::client_trait::ClientTrait;

use super::{ClientInfo, Database};

impl<T: ClientTrait> Database<T> {
    pub fn handle_is_server_operator(&mut self, nickname: &str, sender: Sender<bool>) {
        let response = self.is_server_operator(nickname);
        sender.send(response).unwrap();
    }

    pub fn handle_get_stream_request(&self, nickname: &str, sender: Sender<io::Result<T>>) {
        let response = self.get_stream(nickname);
        sender.send(response).unwrap();
    }

    pub fn handle_contains_client_request(&self, nickname: &str, sender: Sender<bool>) {
        let response = self.contains_client(nickname);
        sender.send(response).unwrap();
    }

    pub fn handle_contains_channel(&self, channel: &str, sender: Sender<bool>) {
        let response = self.contains_channel(channel);
        sender.send(response).unwrap();
    }

    pub fn handle_is_client_in_channel(&self, nickname: &str, channel: &str, sender: Sender<bool>) {
        let response = self.is_client_in_channel(nickname, channel);
        sender.send(response).unwrap();
    }

    pub fn handle_get_clients_for_channel(&self, channel: &str, sender: Sender<Vec<String>>) {
        let response = self.get_clients_for_channel(channel);
        sender.send(response).unwrap();
    }

    pub fn handle_get_all_clients(&self, sender: Sender<Vec<ClientInfo>>) {
        sender.send(self.get_all_clients()).unwrap();
    }

    pub fn handle_get_clients_for_mask(&self, mask: &str, sender: Sender<Vec<ClientInfo>>) {
        let response = self.get_clients_for_mask(mask);
        sender.send(response).unwrap();
    }

    pub fn handle_get_clients_for_nickmask(&self, mask: &str, sender: Sender<Vec<ClientInfo>>) {
        let response = self.get_clients_for_nickmask(mask);
        sender.send(response).unwrap();
    }

    pub fn handle_get_channels(&self, sender: Sender<Vec<String>>) {
        let response = self.get_channels();
        sender.send(response).unwrap();
    }

    pub fn handle_get_channels_for_client(&self, nickname: &str, sender: Sender<Vec<String>>) {
        let response = self.get_channels_for_client(nickname);
        sender.send(response).unwrap();
    }
}
