mod channel;
mod client;
mod utils;

#[cfg(test)]
mod tests;

use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};

pub use channel::Channel;
pub use client::{Client, ClientBuilder, ClientInfo};

use super::client_trait::ClientTrait;
pub struct Database<T: ClientTrait> {
    clients: RwLock<HashMap<String, Client<T>>>,
    channels: RwLock<HashMap<String, Channel>>,
}

impl<T: ClientTrait> Database<T> {
    pub fn new() -> Self {
        Self {
            clients: RwLock::new(HashMap::new()),
            channels: RwLock::new(HashMap::new()),
        }
    }

    pub fn add_client(&self, client: Client<T>) {
        let mut clients_lock = self.clients.write().unwrap();

        println!(
            "Client registered: \npassword: {:?}\nnickname: {}\nrealname: {}",
            client.password(),
            client.nickname(),
            client.realname()
        );

        clients_lock.insert(client.nickname(), client);
    }

    pub fn set_server_operator(&self, nickname: &str) {
        println!("Setting {} as operator", nickname);

        let mut clients_lock = self.clients.write().unwrap();
        if let Some(client) = clients_lock.get_mut(&nickname.to_string()) {
            client.set_server_operator();
        }
    }

    pub fn _is_server_operator(&self, nickname: &str) -> bool {
        let mut clients_lock = self.clients.write().unwrap();
        if let Some(client) = clients_lock.get_mut(&nickname.to_string()) {
            return client.is_server_operator();
        }

        false
    }

    pub fn disconnect_client(&self, nickname: &str) {
        println!("Disconnecting {} ", nickname);

        if let Some(client) = self.clients.write().unwrap().get_mut(nickname) {
            client.disconnect();
        }
    }

    pub fn get_stream(&self, nickname: &str) -> Option<Arc<Mutex<T>>> {
        let clients_rlock = self.clients.read().unwrap();
        let client = clients_rlock.get(nickname)?;

        client.get_stream()
    }

    pub fn add_client_to_channel(&self, nickname: &str, channel_name: &str) {
        println!("Adding {} to channel {}", nickname, channel_name);

        let mut channels_lock = self.channels.write().unwrap();
        let channel: Option<&mut Channel> = channels_lock.get_mut(&channel_name.to_string());
        match channel {
            Some(channel) => channel.add_client(nickname.to_string()),
            None => {
                let new_channel = Channel::new(channel_name.to_string(), nickname.to_string());
                channels_lock.insert(channel_name.to_string(), new_channel);
            }
        }
    }

    pub fn remove_client_of_channel(&self, nickname: &str, channel_name: &str) {
        println!("Removing {} from channel {}", nickname, channel_name);

        let mut channels_lock = self.channels.write().unwrap();
        if let Some(channel) = channels_lock.get_mut(&channel_name.to_string()) {
            channel.remove_client(nickname.to_string());
            if channel.get_clients().is_empty() {
                channels_lock.remove(channel_name);
            }
        }
    }

    pub fn contains_client(&self, nickname: &str) -> bool {
        let clients_lock = self.clients.read().unwrap();

        clients_lock.contains_key(nickname)
    }

    pub fn contains_channel(&self, channel: &str) -> bool {
        let channels_lock = self.channels.read().unwrap();

        channels_lock.contains_key(channel)
    }

    pub fn is_client_in_channel(&self, nickname: &str, channel: &str) -> bool {
        self.get_clients(channel).contains(&nickname.to_string())
    }

    pub fn get_clients(&self, channel: &str) -> Vec<String> {
        let channels_lock = self.channels.read().unwrap();

        let channel_info = channels_lock.get(channel);

        match channel_info {
            Some(channel_info) => channel_info.get_clients(),
            None => vec![],
        }
    }

    pub fn get_channels(&self) -> Vec<String> {
        let channels_lock = self.channels.read().unwrap();

        channels_lock.keys().cloned().collect()
    }

    pub fn get_channels_for_client(&self, nickname: &str) -> Vec<String> {
        let channels_lock = self.channels.read().unwrap();
        let mut channels = vec![];

        for (channel_name, channel) in channels_lock.iter() {
            let clients = channel.get_clients();
            if clients.contains(&nickname.to_string()) {
                channels.push(channel_name.clone());
            }
        }
        channels
    }

    pub fn get_client_info(&self, nickname: &str) -> Option<ClientInfo> {
        let clients_lock = self.clients.read().unwrap();
        let client = clients_lock.get(nickname)?;

        Some(client.get_info())
    }
}

impl<T: ClientTrait> Default for Database<T> {
    fn default() -> Self {
        Self::new()
    }
}
