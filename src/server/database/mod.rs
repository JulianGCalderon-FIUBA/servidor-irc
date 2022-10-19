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
        clients_lock.push(client)
    }
}

impl Default for Database {
    fn default() -> Self {
        Self::new()
    }
}
