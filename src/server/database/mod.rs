mod connection_info;
use std::sync::RwLock;

pub use connection_info::ConnectionInfo;
pub use connection_info::RegistrationState;
pub struct Database {
    pub _clients: RwLock<Vec<ConnectionInfo>>,
}

impl Database {
    pub fn new() -> Self {
        Self {
            _clients: RwLock::new(vec![]),
        }
    }
}

impl Default for Database {
    fn default() -> Self {
        Self::new()
    }
}
