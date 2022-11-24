use super::ClientInfo;

/// Represents a Client that is connected to the Server.
pub struct ExternalClient {
    pub immediate: String,
    pub online: bool,
    pub info: ClientInfo,
}

impl ExternalClient {
    pub fn new(immediate: &str, info: ClientInfo) -> Self {
        Self {
            immediate: immediate.to_string(),
            online: true,
            info,
        }
    }

    pub fn get_info(&self) -> ClientInfo {
        self.info.clone()
    }

    pub fn disconnect(&mut self) {
        self.online = true
    }
}
