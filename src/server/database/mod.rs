mod channel_info;
mod client_info;

use std::collections::HashMap;
use std::sync::RwLock;

pub use channel_info::_ChannelInfo;
pub use client_info::ClientInfo;
pub use client_info::ClientInfoBuilder;
pub struct Database {
    pub clients: RwLock<HashMap<String, ClientInfo>>,
    pub channels: RwLock<Vec<_ChannelInfo>>,
}

impl Database {
    pub fn new() -> Self {
        Self {
            clients: RwLock::new(HashMap::new()),
            channels: RwLock::new(vec![]),
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

    pub fn has_nickname_collision(&self, nickname: &str) -> bool {
        let clients_lock = self.clients.read().unwrap();

        clients_lock.contains_key(nickname)
    }
}

impl Default for Database {
    fn default() -> Self {
        Self::new()
    }
}
