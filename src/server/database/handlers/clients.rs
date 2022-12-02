use std::sync::mpsc::Sender;

use crate::{
    macros::ok_or_return,
    server::{
        connection::Connection,
        consts::modes::UserFlag,
        data_structures::{ClientInfo, ExternalClient, LocalClient},
        database::{database_error::DatabaseError, Database},
    },
};

use crate::macros::{debug_print, some_or_return};

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

    pub fn handle_get_immediate_server(
        &self,
        client: String,
        respond_to: Sender<Result<String, DatabaseError>>,
    ) {
        let server = self.get_immediate_server(client);
        respond_to
            .send(server)
            .expect("Handler receiver should not be dropped");
    }

    /// Returns response to GetAllClients request.
    pub fn handle_get_all_clients(&self, respond_to: Sender<Vec<ClientInfo>>) {
        let clients = self.get_all_clients();
        respond_to
            .send(clients)
            .expect("Handler receiver should not be dropped");
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
        respond_to: Sender<Result<Option<String>, DatabaseError>>,
    ) {
        let message = self.get_away_message(nickname);
        respond_to
            .send(message)
            .expect("Handler receiver should not be dropped");
    }

    /// Returns response to GetChannelsForClient request.
    pub fn handle_get_channels_for_client(
        &self,
        nickname: String,
        respond_to: Sender<Result<Vec<String>, DatabaseError>>,
    ) {
        let channels = self.get_channels_for_client(nickname);
        respond_to
            .send(channels)
            .expect("Handler receiver should not be dropped");
    }

    pub fn handle_get_local_stream_request(
        &self,
        nickname: String,
        respond_to: Sender<Result<C, DatabaseError>>,
    ) {
        let stream = self.get_local_stream(nickname);
        respond_to
            .send(stream)
            .expect("Handler receiver should not be dropped");
    }
    pub fn handle_disconnect_client(&mut self, nickname: String) {
        self.disconnect_client(nickname);
    }

    pub fn handle_get_client_info(
        &mut self,
        client: String,
        respond_to: Sender<Result<ClientInfo, DatabaseError>>,
    ) {
        let client_info = self.get_client_info(&client);
        respond_to
            .send(client_info.cloned())
            .expect("Handler receiver should not be dropped");
    }

    pub fn handle_set_user_flag(&mut self, user: String, flag: UserFlag) {
        self.set_user_flag(user, flag);
    }

    pub fn handle_unset_user_flag(&mut self, user: String, flag: UserFlag) {
        self.unset_user_flag(user, flag);
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
        let client = ok_or_return!(self.get_client_info(&nickname));
        debug_print!("Setting {nickname}'s away message to {message:?}");

        client.away = message
    }

    fn add_external_client(&mut self, client: ExternalClient) {
        debug_print!("Adding external client {:?}", client.info);

        let nickname = client.info.nickname();
        self.external_clients.insert(nickname, client);
    }

    fn set_server_operator(&mut self, nickname: String) {
        let info = ok_or_return!(self.get_client_info(&nickname));
        debug_print!("Setting {} as server operator", nickname);

        info.add_flag(UserFlag::Operator)
    }
    fn add_local_client(&mut self, client: LocalClient<C>) {
        debug_print!("Adding local client {:?}", client.info);

        let nickname = client.info.nickname();
        self.local_clients.insert(nickname, client);
    }

    fn update_nickname(&mut self, old_nickname: String, new_nickname: String) {
        let client_info = ok_or_return!(self.get_client_info(&old_nickname));
        debug_print!("Updating nickname from {old_nickname} to {new_nickname}");

        client_info.update_nickname(new_nickname.to_string());

        self.update_nickname_keys(&old_nickname, &new_nickname);
        self.update_nickname_in_channels(old_nickname, new_nickname);
    }

    fn get_channels_for_client(&self, nickname: String) -> Result<Vec<String>, DatabaseError> {
        if !self.contains_client(nickname.clone()) {
            return Err(DatabaseError::NoSuchClient);
        }

        let mut channels = vec![];

        for (channel_name, channel) in self.channels.iter() {
            let clients = channel.get_clients();
            if clients.contains(&nickname.to_string()) {
                channels.push(channel_name.clone());
            }
        }

        Ok(channels)
    }

    fn get_away_message(&mut self, nickname: String) -> Result<Option<String>, DatabaseError> {
        let info = ok_or_return!(
            self.get_client_info(&nickname),
            Err(DatabaseError::NoSuchClient)
        );

        Ok(info.away.clone())
    }

    fn get_local_stream(&self, nickname: String) -> Result<C, DatabaseError> {
        let client = some_or_return!(
            self.local_clients.get(&nickname),
            Err(DatabaseError::NoSuchClient)
        );

        match &client.stream {
            Some(stream) => stream
                .try_clone()
                .map_err(|_| DatabaseError::CannotCloneStream),
            None => Err(DatabaseError::ClientIsOffline),
        }
    }

    fn get_immediate_server(&self, nickname: String) -> Result<String, DatabaseError> {
        let client = some_or_return!(
            self.external_clients.get(&nickname),
            Err(DatabaseError::NoSuchClient)
        );

        Ok(client.immediate.clone())
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

    fn set_user_flag(&mut self, user: String, flag: UserFlag) {
        let info = ok_or_return!(self.get_client_info(&user));
        info.add_flag(flag);
    }

    fn unset_user_flag(&mut self, user: String, flag: UserFlag) {
        let info = ok_or_return!(self.get_client_info(&user));
        info.remove_flag(flag);
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
