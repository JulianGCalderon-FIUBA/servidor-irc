use std::{io, sync::mpsc::Sender};

use crate::server::{
    connection::Connection,
    data_structures::{ClientInfo, ExternalClient, LocalClient},
    database::Database,
    debug_print,
};

impl<C: Connection> Database<C> {
    pub fn handle_add_local_client(&mut self, client: LocalClient<C>) {
        debug_print!("Adding local client {:?}", client.info);

        let nickname = client.info.nickname();
        self.local_clients.insert(nickname, client);
    }

    pub fn handle_add_external_client(&mut self, client: ExternalClient) {
        debug_print!("Adding external client {:?}", client.info);

        let nickname = client.info.nickname();
        self.external_clients.insert(nickname, client);
    }

    /// Sets client as server operator.
    pub fn handle_set_server_operator(&mut self, nickname: String) {
        if let Some(info) = self.get_client_info(&nickname) {
            debug_print!("Setting {} as server operator", nickname);

            info.operator = true;
        }
    }

    pub fn handle_get_immediate_server(&self, client: String, respond_to: Sender<Option<String>>) {
        let server = self.get_immediate_server(&client);
        respond_to.send(server).unwrap();
    }

    /// Returns response to GetAllClients request.
    pub fn handle_get_all_clients(&self, respond_to: Sender<Vec<ClientInfo>>) {
        let clients = self.get_all_clients();
        respond_to.send(clients).unwrap();
    }

    /// Returns response to GetClientsForMask request.
    pub fn handle_get_clients_for_mask(&self, mask: String, respond_to: Sender<Vec<ClientInfo>>) {
        let clients = self.get_clients_for_mask(&mask);
        respond_to.send(clients).unwrap();
    }

    /// Returns response to GetClientsForNickMask request.
    pub fn handle_get_clients_for_nickmask(
        &self,
        mask: String,
        respond_to: Sender<Vec<ClientInfo>>,
    ) {
        let clients = self.get_clients_for_nickmask(&mask);
        respond_to.send(clients).unwrap();
    }

    /// Returns response to UpdateNickname request.
    pub fn handle_update_nickname(&mut self, old_nickname: String, new_nickname: String) {
        if let Some(client) = self.get_client_info(&old_nickname) {
            debug_print!("Updating nickname from {old_nickname} to {new_nickname}");

            client.update_nickname(new_nickname.to_string());

            if let Some(client) = self.local_clients.remove(&old_nickname) {
                self.local_clients.insert(new_nickname.to_string(), client);
            }
            if let Some(client) = self.external_clients.remove(&old_nickname) {
                self.external_clients
                    .insert(new_nickname.to_string(), client);
            }
            for channel in self.channels.values_mut() {
                channel.update_nickname(&old_nickname, &new_nickname);
            }
        }
    }

    pub fn handle_set_away_message(&mut self, message: Option<String>, nickname: String) {
        if let Some(client) = self.get_client_info(&nickname) {
            debug_print!("Setting {nickname}'s away message to {message:?}");

            client.away = message
        }
    }

    pub fn handle_get_away_message(
        &mut self,
        nickname: String,
        respond_to: Sender<Option<String>>,
    ) {
        let message = self.get_away_message(&nickname);
        respond_to.send(message).unwrap();
    }

    /// Returns response to GetChannelsForClient request.
    pub fn handle_get_channels_for_client(
        &self,
        nickname: String,
        respond_to: Sender<Vec<String>>,
    ) {
        let channels = self.get_channels_for_client(&nickname);
        respond_to.send(channels).unwrap();
    }
}

impl<C: Connection> Database<C> {
    fn filtered_clients(
        &self,
        mask: &str,
        filter: fn(&ClientInfo, &str) -> bool,
    ) -> Vec<ClientInfo> {
        let clients = self.get_all_clients();

        clients
            .into_iter()
            .filter(|client| filter(client, mask))
            .collect()
    }

    pub fn get_away_message(&mut self, nickname: &str) -> Option<String> {
        if let Some(info) = self.get_client_info(nickname) {
            return info.away.clone();
        }

        None
    }

    pub fn get_local_stream(&self, nickname: &str) -> Option<io::Result<C>> {
        if let Some(client) = self.local_clients.get(nickname) {
            return client.get_stream();
        }

        None
    }

    pub fn get_immediate_server(&self, nickname: &str) -> Option<String> {
        if let Some(client) = self.external_clients.get(nickname) {
            return Some(client.immediate.clone());
        }

        None
    }

    /// Returns array with ClientInfo for connected clients.
    pub fn get_all_clients(&self) -> Vec<ClientInfo> {
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

    /// Returns array with ClientInfo for channels that match mask.
    pub fn get_clients_for_mask(&self, mask: &str) -> Vec<ClientInfo> {
        self.filtered_clients(mask, ClientInfo::matches_mask)
    }

    /// Returns array with ClientInfo for channels that match nick mask.
    pub fn get_clients_for_nickmask(&self, mask: &str) -> Vec<ClientInfo> {
        self.filtered_clients(mask, ClientInfo::matches_nickmask)
    }

    pub fn disconnect_client(&mut self, nickname: String) {
        if let Some(client) = self.local_clients.get_mut(&nickname) {
            client.disconnect();
        }
        if let Some(client) = self.external_clients.get_mut(&nickname) {
            client.disconnect();
        }
    }

    pub fn get_client_info(&mut self, nickname: &str) -> Option<&mut ClientInfo> {
        if let Some(client) = self.local_clients.get_mut(nickname) {
            return Some(&mut client.info);
        }
        if let Some(client) = self.external_clients.get_mut(nickname) {
            return Some(&mut client.info);
        }
        None
    }

   
}
