use std::io;

use crate::server::client_trait::ClientTrait;
use crate::server::database::{Channel, Client};

use super::{
    utils::{client_matches_mask, client_matches_nickmask},
    ClientInfo, Database,
};

impl<T: ClientTrait> Database<T> {
    pub fn add_client(&mut self, client: Client<T>) {
        let clientinfo = client.get_info();

        println!("Client registered: {clientinfo:?}",);

        self.clients.insert(clientinfo.nickname, client);
    }

    pub fn set_server_operator(&mut self, nickname: &str) {
        println!("Setting {} as operator", nickname);

        if let Some(client) = self.clients.get_mut(nickname) {
            client.set_server_operator();
        }
    }

    pub fn is_server_operator(&mut self, nickname: &str) -> bool {
        if let Some(client) = self.clients.get_mut(&nickname.to_string()) {
            return client.is_server_operator();
        }

        false
    }

    pub fn disconnect_client(&mut self, nickname: &str) {
        println!("Disconnecting {} ", nickname);

        if let Some(client) = self.clients.get_mut(nickname) {
            client.disconnect();
        }
    }

    pub fn _is_online(&self, nickname: &str) -> bool {
        if let Some(client) = self.clients.get(nickname) {
            return client._is_online();
        }

        false
    }

    pub fn get_stream(&self, nickname: &str) -> io::Result<T> {
        if let Some(client) = self.clients.get(nickname) {
            return client.get_stream();
        }

        Err(io::Error::new(
            io::ErrorKind::Other,
            "Could not find client",
        ))
    }

    pub fn add_client_to_channel(&mut self, nickname: &str, channel_name: &str) {
        println!("Adding {} to channel {}", nickname, channel_name);

        let channel: Option<&mut Channel> = self.channels.get_mut(&channel_name.to_string());
        match channel {
            Some(channel) => channel.add_client(nickname.to_string()),
            None => {
                let new_channel = Channel::new(channel_name.to_string(), nickname.to_string());
                self.channels.insert(channel_name.to_string(), new_channel);
            }
        }
    }

    pub fn remove_client_from_channel(&mut self, nickname: &str, channel_name: &str) {
        println!("Removing {} from channel {}", nickname, channel_name);

        if let Some(channel) = self.channels.get_mut(&channel_name.to_string()) {
            channel.remove_client(nickname.to_string());
            if channel.get_clients().is_empty() {
                self.channels.remove(channel_name);
            }
        }
    }

    pub fn contains_client(&self, nickname: &str) -> bool {
        self.clients.contains_key(nickname)
    }

    pub fn contains_channel(&self, channel: &str) -> bool {
        self.channels.contains_key(channel)
    }

    pub fn is_client_in_channel(&self, nickname: &str, channel: &str) -> bool {
        self.get_clients_for_channel(channel)
            .contains(&nickname.to_string())
    }

    pub fn get_clients_for_channel(&self, channel: &str) -> Vec<String> {
        let channel_info = self.channels.get(channel);

        match channel_info {
            Some(channel_info) => channel_info.get_clients(),
            None => vec![],
        }
    }

    pub fn get_all_clients(&self) -> Vec<ClientInfo> {
        self.clients
            .values()
            .map(|client| client.get_info())
            .collect()
    }

    pub fn get_clients_for_mask(&self, mask: &str) -> Vec<ClientInfo> {
        self.filtered_clients(mask, client_matches_mask)
    }

    pub fn get_clients_for_nickmask(&self, mask: &str) -> Vec<ClientInfo> {
        self.filtered_clients(mask, client_matches_nickmask)
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

    pub fn filtered_clients(
        &self,
        mask: &str,
        filter: fn(&ClientInfo, &str) -> bool,
    ) -> Vec<ClientInfo> {
        self.clients
            .values()
            .map(|client| client.get_info())
            .filter(|client| filter(client, mask))
            .collect()
    }
}
