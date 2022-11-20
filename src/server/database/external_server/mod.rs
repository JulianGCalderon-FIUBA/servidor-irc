mod external_client;
use external_client::ExternalClient;

use std::collections::HashMap;

use crate::server::connection::Connection;

/// Represents a Client that is connected to the Server.
pub struct ExternalServer<C: Connection> {
    _stream: C,
    servername: String,
    _serverinfo: String,
    _hopcount: usize,
    _clients: HashMap<String, ExternalClient>,
}

impl<C: Connection> ExternalServer<C> {
    pub fn new(_stream: C, servername: String, _serverinfo: String, _hopcount: usize) -> Self {
        Self {
            _stream,
            servername,
            _serverinfo,
            _hopcount,
            _clients: HashMap::new(),
        }
    }

    pub fn servername(&self) -> String {
        self.servername.clone()
    }
}
