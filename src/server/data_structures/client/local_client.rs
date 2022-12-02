use crate::server::connection::Connection;

use super::ClientInfo;

/// Represents a Client that is connected to the Server.
pub struct LocalClient<C: Connection> {
    stream: Option<C>,
    _password: Option<String>,
    info: ClientInfo,
}

impl<C: Connection> LocalClient<C> {
    pub fn new(stream: C, _password: Option<String>, info: ClientInfo) -> Self {
        Self {
            stream: Some(stream),
            info,
            _password,
        }
    }

    pub fn get_info(&self) -> ClientInfo {
        self.info.clone()
    }

    pub fn disconnect(&mut self) {
        self.stream = None;
    }

    pub fn stream(&self) -> Option<&C> {
        self.stream.as_ref()
    }

    pub fn nickname(&self) -> String {
        self.info.nickname()
    }

    pub fn info_mut(&mut self) -> &mut ClientInfo {
        &mut self.info
    }
}
