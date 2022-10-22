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

    pub fn add_client_to_channel(&self, nickname: &str, channel_name: &str) {
        let mut channels_lock = self.channels.write().unwrap();
        let channel: Option<&mut ChannelInfo> = channels_lock.get_mut(&channel_name.to_string());
        match channel {
            Some(channel) => channel.add_client(nickname.to_string()),
            None => {
                let new_channel = ChannelInfo::new(channel_name.to_string(), nickname.to_string());
                channels_lock.insert(channel_name.to_string(), new_channel);
            }
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
        let channels_lock = self.clients.read().unwrap();
        let mut channels = vec![];

        for (channel_name, _) in channels_lock.iter() {
            let clients = self._get_clients(channel_name);
            if clients.contains(&nickname.to_string()) {
                channels.push(channel_name.clone());
            }
        }
        channels
    }
}

impl Default for Database {
    fn default() -> Self {
        Self::new()
    }
}
