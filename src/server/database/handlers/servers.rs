use std::sync::mpsc::Sender;

use crate::server::{
    connection::Connection,
    data_structures::{ImmediateServer, ServerInfo},
    database::{database_error::DatabaseError, Database},
};

use crate::macros::{debug_print, some_or_return};

impl<C: Connection> Database<C> {
    pub fn handle_add_immediate_server(&mut self, server: ImmediateServer<C>) {
        self.add_immediate_server(server);
    }

    pub fn handle_add_distant_server(&mut self, server: ServerInfo) {
        self.add_distant_server(server);
    }

    pub fn handle_get_servername(&self, respond_to: Sender<String>) {
        let servername = self.info.servername.clone();
        respond_to
            .send(servername)
            .expect("Handler receiver should not be dropped");
    }

    pub fn handle_get_own_server_info(&self, respond_to: Sender<String>) {
        let serverinfo = self.info.serverinfo.clone();
        respond_to
            .send(serverinfo)
            .expect("Handler receiver should not be dropped");
    }

    pub fn handle_get_server_stream(
        &self,
        server: String,
        respond_to: Sender<Result<C, DatabaseError>>,
    ) {
        let stream = self.get_server_stream(&server);
        respond_to
            .send(stream)
            .expect("Handler receiver should not be dropped");
    }

    pub fn handle_get_all_servers(&self, respond_to: Sender<Vec<String>>) {
        let servers = self.get_all_servers();
        respond_to
            .send(servers)
            .expect("Handler receiver should not be dropped");
    }

    pub fn handle_remove_server(&mut self, servername: String) {
        self.remove_server(servername);
    }

    pub fn handle_get_server_info(
        &self,
        server: String,
        respond_to: Sender<Result<ServerInfo, DatabaseError>>,
    ) {
        let server_info = self.get_server_info(server);
        respond_to
            .send(server_info)
            .expect("Handler receiver should not be dropped");
    }
}

impl<C: Connection> Database<C> {
    fn remove_server(&mut self, servername: String) {
        let removed = self.immediate_servers.remove(&servername);
        if removed.is_none() {
            self.distant_servers.remove(&servername);
        }
    }

    fn get_server_stream(&self, server: &str) -> Result<C, DatabaseError> {
        let server = some_or_return!(
            self.immediate_servers.get(server),
            Err(DatabaseError::NoSuchServer)
        );

        server
            .get_stream()
            .map_err(|_| DatabaseError::CannotCloneStream)
    }

    fn get_all_servers(&self) -> Vec<String> {
        self.immediate_servers
            .keys()
            .map(|key| key.to_string())
            .collect()
    }

    fn add_immediate_server(&mut self, server: ImmediateServer<C>) {
        let servername = server.info().servername;
        debug_print!("Adding immediate server {servername}");
        self.immediate_servers.insert(servername, server);
    }
    fn add_distant_server(&mut self, server: ServerInfo) {
        let servername = server.servername.clone();
        debug_print!("Adding distant server {servername}");
        self.distant_servers.insert(servername, server);
    }

    fn get_server_info(&self, server: String) -> Result<ServerInfo, DatabaseError> {
        if let Some(server) = self.immediate_servers.get(&server) {
            return Ok(server.info());
        }

        let server = some_or_return!(
            self.distant_servers.get(&server),
            Err(DatabaseError::NoSuchServer)
        );

        Ok(server.clone())
    }
}
