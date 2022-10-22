mod client_info;

use std::sync::RwLock;

pub use client_info::ClientInfo;
pub use client_info::ClientInfoBuilder;
pub struct Database {
    pub clients: RwLock<Vec<ClientInfo>>,
}

impl Database {
    pub fn new() -> Self {
        Self {
            clients: RwLock::new(vec![]),
        }
    }

    pub fn add_client(&self, client: ClientInfo) {
        let mut clients_lock = self.clients.write().unwrap();

        println!(
            "Client registered: \npassword: {:?}\nnickname: {}\nrealname: {}",
            client.password, client.nickname, client.realname
        );
        clients_lock.push(client)
    }

    pub fn has_nickname_collision(&self, nickname: &str) -> bool {
        let clients_lock = self.clients.read().unwrap();

        for client in clients_lock.iter() {
            if client.nickname == nickname {
                return true;
            }
        }
        false
    }
}

impl Default for Database {
    fn default() -> Self {
        Self::new()
    }
}
