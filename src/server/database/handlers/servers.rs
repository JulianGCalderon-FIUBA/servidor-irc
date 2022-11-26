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
        respond_to.send(servername).unwrap();
    }

    pub fn handle_get_serverinfo(&self, respond_to: Sender<String>) {
        let serverinfo = self.info.serverinfo.clone();
        respond_to.send(serverinfo).unwrap();
    }

    pub fn handle_get_server_stream(
        &self,
        server: String,
        respond_to: Sender<Result<C, DatabaseError>>,
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
        let servername = server.info.servername.clone();
        debug_print!("Adding immediate server {servername}");
        self.immediate_servers.insert(servername, server);
    }
    fn add_distant_server(&mut self, server: ServerInfo) {
        let servername = server.servername.clone();
        debug_print!("Adding distant server {servername}");
        self.distant_servers.insert(servername, server);
    }
}
