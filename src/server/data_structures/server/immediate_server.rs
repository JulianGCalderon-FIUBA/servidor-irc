use crate::server::connection::Connection;

use super::ServerInfo;

pub struct ImmediateServer<C: Connection> {
    pub stream: C,
    pub info: ServerInfo,
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
}