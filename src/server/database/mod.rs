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

    pub fn contains_client(&self, nickname: &str) -> bool {
        let clients_lock = self.clients.read().unwrap();

        clients_lock.contains_key(nickname)
    }

    pub fn get_stream(&self, nickname: &str) -> Arc<Mutex<TcpStream>> {
        todo!()
    }

    pub fn get_clients(&self, channel: &str) -> Vec<String> {
        todo!()
    }

    pub fn get_channels(&self) -> Vec<String> {
        todo!()
    }

    pub fn set_server_operator(&mut self, nickname: &str) {
        todo!()
    }

    pub fn add_client_to_channel(&mut self, nickname: &str, channel: &str) {
        todo!()
    }
    pub fn remove_client_to_channel(&mut self, nickname: &str, channel: &str) {
        todo!()
    }

    pub fn disconnect_client(&mut self, nickname: &str) {
        todo!()
    }
}

impl Default for Database {
    fn default() -> Self {
        Self::new()
    }
}
