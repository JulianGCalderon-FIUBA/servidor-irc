use std::cell::RefCell;
use std::io;
use std::rc::Rc;

use crate::server::client_trait::Connection;
use crate::server::database::{Channel, Client};

use super::{
    utils::{client_matches_mask, client_matches_nickmask},
    ClientInfo, Database,
};

impl<C: Connection> Database<C> {
    /// Adds client to Database.
    pub fn add_client(&mut self, client: Client<C>) {
        let clientinfo = client.get_info();

        println!("Client registered: {clientinfo:?}",);

        let client = Rc::new(RefCell::new(client));

        self.clients.insert(clientinfo.nickname, client);
    }

    /// Verifies if operator credentials are valid.
    pub fn are_credentials_valid(&self, username: &str, password: &str) -> bool {
        if let Some(real_password) = self.credentials.get(username) {
            return password == real_password;
        }

        false
    }

    /// Sets client as server operator.
    pub fn set_server_operator(&mut self, nickname: &str) {
        println!("Setting {} as operator", nickname);

        if let Some(client) = self.clients.get_mut(nickname) {
            client.borrow_mut().set_server_operator();
        }
    }

    /// Returns if client is server operator.
    pub fn is_server_operator(&mut self, nickname: &str) -> bool {
        if let Some(client) = self.clients.get(nickname) {
            return client.borrow_mut().is_server_operator();
        }

        false
    }

    /// Disconnects client from server, removing it from Database.
    pub fn disconnect_client(&mut self, nickname: &str) {
        println!("Disconnecting {} ", nickname);

        self.clients.remove(nickname);

        let channels = self.get_channels_for_client(nickname);

        for channel in channels {
            self.remove_client_from_channel(nickname, &channel)
        }
    }

    /// Returns the client's stream or error if client is disconnected.
    pub fn get_stream(&self, nickname: &str) -> io::Result<C> {
        if let Some(client) = self.clients.get(nickname) {
            return client.borrow().get_stream();
        }

        Err(io::Error::new(
            io::ErrorKind::Other,
            "Could not find client",
        ))
    }

    /// Adds client to channel.
    pub fn add_client_to_channel(&mut self, nickname: &str, channel_name: &str) {
        println!("Adding {} to channel {}", nickname, channel_name);

        let channel: Option<&mut Channel<C>> = self.channels.get_mut(&channel_name.to_string());
        if let Some(client) = self.clients.get(nickname) {
            let client_rc = client.clone();

            match channel {
                Some(channel) => channel.add_client(client_rc),
                None => {
                    let new_channel = Channel::new(channel_name.to_string(), client_rc);
                    self.channels.insert(channel_name.to_string(), new_channel);
                }
            }
        }
    }

    /// Removes client from channel.
    pub fn remove_client_from_channel(&mut self, nickname: &str, channel_name: &str) {
        println!("Removing {} from channel {}", nickname, channel_name);

        if let Some(channel) = self.channels.get_mut(&channel_name.to_string()) {
            channel.remove_client(nickname);
            if channel.get_clients().is_empty() {
                self.channels.remove(channel_name);
            }
        }
    }

    /// Returns if Database contains client.
    pub fn contains_client(&self, nickname: &str) -> bool {
        self.clients.get(nickname).is_some()
    }

    /// Returns if Database contains channel.
    pub fn contains_channel(&self, channel: &str) -> bool {
        self.channels.contains_key(channel)
    }

    /// Returns if client is in channel.
    pub fn is_client_in_channel(&self, nickname: &str, channel: &str) -> bool {
        if let Some(channel) = self.channels.get(channel) {
            return channel.contains_client(nickname);
        }

        false
    }

    /// Returns array of clients for channel.
    pub fn get_clients_for_channel(&self, channel: &str) -> Vec<String> {
        let channel_info = self.channels.get(channel);

        match channel_info {
            Some(channel_info) => channel_info.get_clients(),
            None => vec![],
        }
    }

    /// Returns array with ClientInfo for connected clients.
    pub fn get_all_clients(&self) -> Vec<ClientInfo> {
        self.clients
            .values()
            .map(|client| client.borrow().get_info())
            .collect()
    }

    /// Returns array with ClientInfo for channels that match mask.
    pub fn get_clients_for_mask(&self, mask: &str) -> Vec<ClientInfo> {
        self.filtered_clients(mask, client_matches_mask)
    }

    /// Returns array with ClientInfo for channels that match nick mask.
    pub fn get_clients_for_nickmask(&self, mask: &str) -> Vec<ClientInfo> {
        self.filtered_clients(mask, client_matches_nickmask)
    }

    /// Returns array of channels in Database.
    pub fn get_channels(&self) -> Vec<String> {
        self.channels.keys().cloned().collect()
    }

    /// Returns array of channels the client is connected to.
    pub fn get_channels_for_client(&self, nickname: &str) -> Vec<String> {
        let mut channels = vec![];

        for (channel_name, channel) in self.channels.iter() {
            let clients = channel.get_clients();
            if clients.contains(&nickname.to_string()) {
                channels.push(channel_name.clone());
            }
        }
        channels
    }

    fn filtered_clients(
        &self,
        mask: &str,
        filter: fn(&ClientInfo, &str) -> bool,
    ) -> Vec<ClientInfo> {
        self.clients
            .values()
            .map(|client| client.borrow().get_info())
            .filter(|client| filter(client, mask))
            .collect()
    }

    pub fn set_channel_topic(&mut self, channel: &str, topic: &str) {
        if let Some(channel) = self.channels.get_mut(channel) {
            channel.set_topic(topic);
        }
    }

    pub fn get_channel_topic(&self, channel: &str) -> Option<String> {
        if let Some(channel) = self.channels.get(channel) {
            return channel.get_topic();
        }
        None
    }
}
