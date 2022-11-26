use std::{io, sync::mpsc::Sender};

use crate::server::{
    connection::Connection,
    data_structures::{ImmediateServer, ServerInfo},
    database::Database,
    debug_print,
};

impl<C: Connection> Database<C> {
    pub fn handle_get_local_stream_request(
        &self,
        nickname: String,
        respond_to: Sender<Option<io::Result<C>>>,
    ) {
        let stream = self.get_local_stream(&nickname);
        respond_to.send(stream).unwrap();
    }

    pub fn handle_add_immediate_server(&mut self, server: ImmediateServer<C>) {
        let servername = server.info.servername.clone();
        debug_print!("Adding immediate server {servername}");

        self.immediate_servers.insert(servername, server);
    }

    pub fn handle_add_distant_server(&mut self, server: ServerInfo) {
        let servername = server.servername.clone();
        debug_print!("Adding distant server {servername}");

        self.distant_servers.insert(servername, server);
    }

    pub fn handle_get_servername(&self, respond_to: Sender<String>) {
        respond_to.send(self.info.servername.clone()).unwrap();
    }

    pub fn handle_get_serverinfo(&self, respond_to: Sender<String>) {
        respond_to.send(self.info.serverinfo.clone()).unwrap();
    }

    pub fn handle_get_server_stream(
        &self,
        server: String,
        respond_to: Sender<Option<io::Result<C>>>,
    ) {
        let stream = self.get_server_stream(&server);
        respond_to.send(stream).unwrap();
    }

    pub fn handle_get_all_servers(&self, respond_to: Sender<Vec<String>>) {
        let stream = self.get_all_servers();
        respond_to.send(stream).unwrap();
    }
}

impl<C: Connection> Database<C> {
    pub fn get_server_stream(&self, server: &str) -> Option<Result<C, std::io::Error>> {
        if let Some(server) = self.immediate_servers.get(server) {
            return Some(server.get_stream());
        }

        None
    }

    pub fn get_all_servers(&self) -> Vec<String> {
        self.immediate_servers
            .keys()
            .map(|key| key.to_string())
            .collect()
    }
}
