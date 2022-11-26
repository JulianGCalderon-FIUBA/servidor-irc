use std::{io, sync::mpsc::Sender};

use crate::server::{
    connection::Connection,
    data_structures::{ClientInfo, ExternalClient, LocalClient},
    database::Database,
};

use crate::macros::{debug_print, unwrap_or_return};

impl<C: Connection> Database<C> {
    pub fn handle_add_local_client(&mut self, client: LocalClient<C>) {
        self.add_local_client(client);
    }

    pub fn handle_add_external_client(&mut self, client: ExternalClient) {
        self.add_external_client(client);
    }

    /// Sets client as server operator.
    pub fn handle_set_server_operator(&mut self, nickname: String) {
        self.set_server_operator(nickname);
    }

    pub fn handle_get_immediate_server(&self, client: String, respond_to: Sender<Option<String>>) {
        let server = self.get_immediate_server(client);
        respond_to.send(server).unwrap();
    }

    /// Returns response to GetAllClients request.
    pub fn handle_get_all_clients(&self, respond_to: Sender<Vec<ClientInfo>>) {
        let clients = self.get_all_clients();
        respond_to.send(clients).unwrap();
    }

    /// Returns response to UpdateNickname request.
    pub fn handle_update_nickname(&mut self, old_nickname: String, new_nickname: String) {
        self.update_nickname(old_nickname, new_nickname);
    }

    pub fn handle_set_away_message(&mut self, message: Option<String>, nickname: String) {
        self.set_away_message(nickname, message);
    }

    pub fn handle_get_away_message(
        &mut self,
        nickname: String,
        respond_to: Sender<Option<String>>,
    ) {
        let message = self.get_away_message(nickname);
        respond_to.send(message).unwrap();
    }

    /// Returns response to GetChannelsForClient request.
    pub fn handle_get_channels_for_client(
        &self,
        nickname: String,
        respond_to: Sender<Vec<String>>,
    ) {
        let channels = self.get_channels_for_client(nickname);
        respond_to.send(channels).unwrap();
    }

    pub fn handle_get_local_stream_request(
        &self,
        nickname: String,
        respond_to: Sender<Option<io::Result<C>>>,
    ) {
        let stream = self.get_local_stream(nickname);
        respond_to.send(stream).unwrap();
    }
    pub fn handle_disconnect_client(&mut self, nickname: String) {
        self.disconnect_client(nickname);
    }

    pub fn handle_get_client_info(
        &mut self,
        client: String,
        respond_to: Sender<Option<ClientInfo>>,
    ) {
        let client_info = self.get_client_info(&client);
        respond_to.send(client_info.cloned()).unwrap();
    }
}

impl<C: Connection> Database<C> {
    fn disconnect_client(&mut self, nickname: String) {
        if let Some(client) = self.local_clients.get_mut(&nickname) {
            client.disconnect();
        }
        if let Some(client) = self.external_clients.get_mut(&nickname) {
            client.disconnect();
        }
    }
    fn set_away_message(&mut self, nickname: String, message: Option<String>) {
        let client = unwrap_or_return!(self.get_client_info(&nickname));
        debug_print!("Setting {nickname}'s away message to {message:?}");

        client.away = message
    }

    fn add_external_client(&mut self, client: ExternalClient) {
        debug_print!("Adding external client {:?}", client.info);

        let nickname = client.info.nickname();
        self.external_clients.insert(nickname, client);
    }

    fn set_server_operator(&mut self, nickname: String) {
        let info = unwrap_or_return!(self.get_client_info(&nickname));
        debug_print!("Setting {} as server operator", nickname);

        info.operator = true;
    }
    fn add_local_client(&mut self, client: LocalClient<C>) {
        debug_print!("Adding local client {:?}", client.info);

        let nickname = client.info.nickname();
        self.local_clients.insert(nickname, client);
    }

    fn update_nickname(&mut self, old_nickname: String, new_nickname: String) {
        let client_info = unwrap_or_return!(self.get_client_info(&old_nickname));
        debug_print!("Updating nickname from {old_nickname} to {new_nickname}");

        client_info.update_nickname(new_nickname.to_string());

        self.update_nickname_keys(&old_nickname, &new_nickname);
        self.update_nickname_in_channels(old_nickname, new_nickname);
    }

    fn get_channels_for_client(&self, nickname: String) -> Vec<String> {
        let mut channels = vec![];

        for (channel_name, channel) in self.channels.iter() {
            let clients = channel.get_clients();
            if clients.contains(&nickname.to_string()) {
                channels.push(channel_name.clone());
            }
        }
        channels
    }

    fn get_away_message(&mut self, nickname: String) -> Option<String> {
        let info = unwrap_or_return!(self.get_client_info(&nickname), None);

        info.away.clone()
    }

    fn get_local_stream(&self, nickname: String) -> Option<io::Result<C>> {
        let client = unwrap_or_return!(self.local_clients.get(&nickname), None);

        client.get_stream()
    }

    fn get_immediate_server(&self, nickname: String) -> Option<String> {
        let client = unwrap_or_return!(self.external_clients.get(&nickname), None);

        Some(client.immediate.clone())
    }

    /// Returns array with ClientInfo for connected clients.
    fn get_all_clients(&self) -> Vec<ClientInfo> {
        let mut clients: Vec<ClientInfo> = self
            .local_clients
            .values()
            .map(LocalClient::get_info)
            .collect();

        let mut external_clients: Vec<ClientInfo> = self
            .external_clients
            .values()
            .map(ExternalClient::get_info)
            .collect();

        clients.append(&mut external_clients);

        clients
    }
}

impl<C: Connection> Database<C> {
    fn update_nickname_keys(&mut self, old_nickname: &String, new_nickname: &String) {
        if let Some(client) = self.local_clients.remove(old_nickname) {
            self.local_clients.insert(new_nickname.to_string(), client);
        }
        if let Some(client) = self.external_clients.remove(old_nickname) {
            self.external_clients
                .insert(new_nickname.to_string(), client);
        }
    }

    fn update_nickname_in_channels(&mut self, old_nickname: String, new_nickname: String) {
        for channel in self.channels.values_mut() {
            channel.update_nickname(&old_nickname, &new_nickname);
        }
    }
}
