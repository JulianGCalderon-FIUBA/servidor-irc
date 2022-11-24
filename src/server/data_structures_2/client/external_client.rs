use super::ClientInfo;

/// Represents a Client that is connected to the Server.
pub struct ExternalClient {
    pub immediate: String,
    pub info: ClientInfo,
}

impl ExternalClient {
    pub fn new(immediate: &str, info: ClientInfo) -> Self {
        Self {
            immediate: immediate.to_string(),
            info,
        }
    }
}
