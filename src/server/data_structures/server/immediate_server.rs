use crate::server::connection::Connection;

use super::ServerInfo;
/// Represents a server connected to local server.
pub struct ImmediateServer<C: Connection> {
    stream: C,
    info: ServerInfo,
}

impl<C: Connection> ImmediateServer<C> {
    pub fn new(stream: C, servername: String, serverinfo: String, hopcount: usize) -> Self {
        Self {
            stream,
            info: ServerInfo::new(servername, serverinfo, hopcount),
        }
    }

    pub fn get_stream(&self) -> Result<C, std::io::Error> {
        self.stream.try_clone()
    }

    pub fn info(&self) -> ServerInfo {
        self.info.clone()
    }
}
