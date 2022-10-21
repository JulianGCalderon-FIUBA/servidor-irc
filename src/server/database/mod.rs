mod channel_info;
mod client_info;

use std::collections::HashMap;
use std::net::TcpStream;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::RwLock;

pub use channel_info::ChannelInfo;
pub use client_info::ClientInfo;
pub use client_info::ClientInfoBuilder;
pub struct Database {
    pub clients: RwLock<HashMap<String, ClientInfo>>,
    pub channels: RwLock<HashMap<String, ChannelInfo>>,
}

impl Database {
    pub fn new() -> Self {
        Self {
            clients: RwLock::new(HashMap::new()),
            channels: RwLock::new(HashMap::new()),
        }
    }

    pub fn add_client(&self, client: ClientInfo) {
        let mut clients_lock = self.clients.write().unwrap();

        println!(
            "Client registered: \npassword: {:?}\nnickname: {}\nrealname: {}",
            client.password, client.nickname, client.realname
        );

        clients_lock.insert(client.nickname.clone(), client);
    }

    pub fn _disconnect_client(&self, _nickname: &str) {
        todo!()
    }

    pub fn _set_server_operator(&self, _nickname: &str) {
        todo!()
    }

    pub fn add_client_to_channel(&self, nickname: &str, channel: &str) {
        let mut channels_lock = self.channels.write().unwrap();
        if let Some(channel) = channels_lock.get_mut(&channel.to_string()) {
            channel.add_client(nickname.to_string());
        }
    }

    pub fn remove_client_of_channel(&self, nickname: &str, channel: &str) {
        let mut channels_lock = self.channels.write().unwrap();
        if let Some(channel) = channels_lock.get_mut(&channel.to_string()) {
            channel.remove_client(nickname.to_string());
        }
    }

    pub fn contains_client(&self, nickname: &str) -> bool {
        let clients_lock = self.clients.read().unwrap();

        clients_lock.contains_key(nickname)
    }

    pub fn _get_stream(&self, _nickname: &str) -> Arc<Mutex<TcpStream>> {
        todo!()
    }

    pub fn _get_clients(&self, _channel: &str) -> Vec<String> {
        todo!()
    }

    pub fn _get_channels(&self) -> Vec<String> {
        todo!()
    }

    pub fn get_channels_for_client(&self, nickname: &str) -> Vec<String> {
        let clients_lock = self.clients.read().unwrap();

        let client = clients_lock.get(nickname).unwrap();
        client.channels.clone()
    }
}

impl Default for Database {
    fn default() -> Self {
        Self::new()
    }
}
