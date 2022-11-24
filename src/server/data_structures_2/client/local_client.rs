use crate::server::connection::Connection;

use super::ClientInfo;

/// Represents a Client that is connected to the Server.
pub struct LocalClient<C: Connection> {
    pub stream: Option<C>,
    pub password: Option<String>,
    pub info: ClientInfo,
}

impl<C: Connection> LocalClient<C> {
    pub fn new(stream: C, password: &Option<String>, info: ClientInfo) -> Self {
        Self {
            stream: Some(stream),
            password: password.clone(),
            info,
        }
    }

    pub(crate) fn get_stream(&self) -> Option<Result<C, std::io::Error>> {
        Some(self.stream.as_ref()?.try_clone())
    }

    pub fn get_info(&self) -> ClientInfo {
        self.info.clone()
    }
}
