use crate::server::connection::Connection;

use super::ServerInfo;

pub struct ImmediateServer<C: Connection> {
    pub stream: C,
    pub info: ServerInfo,
}

impl<C: Connection> ImmediateServer<C> {
    pub fn new(stream: C, servername: &str, serverinfo: &str, hopcount: usize) -> Self {
        Self {
            stream,
            info: ServerInfo::new(servername, serverinfo, hopcount),
        }
    }

    pub(crate) fn get_stream(&self) -> Result<C, std::io::Error> {
        self.stream.try_clone()
    }
}
