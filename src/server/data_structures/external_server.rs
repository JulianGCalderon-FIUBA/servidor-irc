use std::{collections::HashMap, io};

use crate::server::connection::Connection;

use super::*;

pub struct ExternalServer<C: Connection> {
    stream: Option<C>,
    servername: String,
    _serverinfo: String,
    _hopcount: usize,
    clients: HashMap<String, ExternalClient>,
}

impl<C: Connection> ExternalServer<C> {
    pub fn new(stream: C, servername: String, _serverinfo: String, _hopcount: usize) -> Self {
        Self {
            stream: Some(stream),
            servername,
            _serverinfo,
            _hopcount,
            clients: HashMap::new(),
        }
    }

    pub fn servername(&self) -> String {
        self.servername.clone()
    }

    pub fn get_all_clients(&self) -> Vec<ClientInfo> {
        self.clients
            .values()
            .map(|client| client.get_info())
            .collect()
    }

    pub fn contains_client(&self, nickname: &str) -> bool {
        self.clients.contains_key(nickname)
    }

    pub fn get_stream(&self) -> Option<io::Result<C>> {
        Some(self.stream.as_ref()?.try_clone())
    }

    pub fn add_client(&mut self, client: ExternalClient) {
        let nickname = client.nickname();
        self.clients.insert(nickname, client);
    }
}
