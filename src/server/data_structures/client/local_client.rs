use crate::server::connection::Connection;

use super::ClientInfo;

/// Represents a Client that is connected to the Server.
pub struct LocalClient<C: Connection> {
    pub stream: Option<C>,
    pub password: Option<String>,
    pub info: ClientInfo,
}

impl<C: Connection> LocalClient<C> {
    pub fn get_info(&self) -> ClientInfo {
        self.info.clone()
    }

    pub fn disconnect(&mut self) {
        self.stream = None;
    }
}