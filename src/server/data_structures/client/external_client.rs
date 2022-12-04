use super::ClientInfo;

/// Represents an external client.
/// Must contain information about through which server they can be contacted.
pub struct ExternalClient {
    immediate: String,
    online: bool,
    info: ClientInfo,
}

impl ExternalClient {
    pub fn new(immediate: String, info: ClientInfo) -> Self {
        Self {
            immediate,
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

    pub fn info_mut(&mut self) -> &mut ClientInfo {
        &mut self.info
    }

    pub fn nickname(&self) -> String {
        self.info.nickname()
    }

    pub fn immediate(&self) -> String {
        self.immediate.clone()
    }
}
