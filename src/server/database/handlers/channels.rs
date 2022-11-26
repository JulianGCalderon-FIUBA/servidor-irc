use std::sync::mpsc::Sender;

use crate::server::{
    connection::Connection, data_structures::Channel, database::Database, debug_print,
};

impl<C: Connection> Database<C> {
    pub fn handle_get_clients_for_channel(&self, channel: String, respond_to: Sender<Vec<String>>) {
        let clients = self.get_clients_for_channel(&channel);
        respond_to.send(clients).unwrap();
    }

    pub fn handle_get_all_channels(&self, respond_to: Sender<Vec<String>>) {
        let channels = self.get_channels();
        respond_to.send(channels).unwrap();
    }

    pub fn handle_get_local_clients_for_channel(
        &self,
        channel: String,
        respond_to: Sender<Vec<String>>,
    ) {
        let stream = self.get_local_clients_for_channel(&channel);
        respond_to.send(stream).unwrap();
    }
}

impl<C: Connection> Database<C> {
    pub fn add_client_to_channel(&mut self, nickname: String, channel: String) {
        if let Some(channel) = self.channels.get_mut(&channel) {
            debug_print!("Adding {} to channel {}", nickname, channel.name);

            channel.add_member(nickname)
        } else {
            let channel = Channel::new(channel, nickname.clone());

            let name = channel.name.to_string();
            debug_print!("Creating channel {} with client {}", &name, nickname);

            self.channels.insert(name, channel);
        }
    }

    pub fn remove_client_from_channel(&mut self, nickname: String, channel_name: String) {
        if let Some(channel) = self.channels.get_mut(&channel_name.to_string()) {
            debug_print!("Removing {} from channel {}", nickname, channel_name);

            channel.remove_client(&nickname);
            if channel.get_clients().is_empty() {
                self.channels.remove(&channel_name);
            }
        }
    }

    pub fn get_clients_for_channel(&self, channel: &str) -> Vec<String> {
        let channel_info = self.channels.get(channel);

        match channel_info {
            Some(channel_info) => channel_info.get_clients(),
            None => vec![],
        }
    }

    pub fn get_channels(&self) -> Vec<String> {
        self.channels.keys().cloned().collect()
    }

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

    pub fn get_local_clients_for_channel(&self, channel: &str) -> Vec<String> {
        if let Some(channel) = self.channels.get(channel) {
            return channel
                .get_clients()
                .into_iter()
                .filter(|client| self.local_clients.contains_key(client))
                .collect();
        }

        vec![]
    }
}
