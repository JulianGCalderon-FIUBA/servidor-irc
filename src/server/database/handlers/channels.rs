use std::sync::mpsc::Sender;

use crate::server::database::database_error::DatabaseError;
use crate::server::{connection::Connection, data_structures::Channel, database::Database};

use crate::macros::{debug_print, some_or_return};

impl<C: Connection> Database<C> {
    pub fn handle_get_channel_clients(
        &self,
        channel: String,
        respond_to: Sender<Result<Vec<String>, DatabaseError>>,
    ) {
        let clients = self.get_channel_clients(channel);
        respond_to
            .send(clients)
            .expect("Handler receiver should not be dropped");
    }

    pub fn handle_get_all_channels(&self, respond_to: Sender<Vec<String>>) {
        let channels = self.get_channels();
        respond_to
            .send(channels)
            .expect("Handler receiver should not be dropped");
    }

    pub fn handle_add_client_to_channel(&mut self, nickname: String, channel: String) {
        self.add_client_to_channel(channel, nickname);
    }

    pub fn handle_remove_client_from_channel(&mut self, nickname: String, channel_name: String) {
        self.remove_client_from_channel(channel_name, nickname);
    }

    pub fn handle_add_channel_invitation(&mut self, channel: String, client: String) {
        self.add_channel_invitation(channel, client);
    }
}

impl<C: Connection> Database<C> {
    fn add_client_to_channel(&mut self, channel: String, nickname: String) {
        match self.channels.get_mut(&channel) {
            Some(channel) => {
                debug_print!("Adding {} to channel {}", nickname, channel.name());
                channel.add_member(nickname)
            }
            None => {
                debug_print!("Creating channel {} with client {}", &channel, &nickname);
                self.create_channel(channel, nickname);
            }
        }
    }

    fn remove_client_from_channel(&mut self, channel_name: String, nickname: String) {
        let channel = some_or_return!(self.channels.get_mut(&channel_name.to_string()));

        debug_print!("Removing {} from channel {}", nickname, channel_name);
        channel.remove_client(&nickname);
        if channel.get_clients().is_empty() {
            self.channels.remove(&channel_name);
        }
    }
    fn get_channel_clients(&self, channel: String) -> Result<Vec<String>, DatabaseError> {
        let channel = some_or_return!(
            self.channels.get(&channel),
            Err(DatabaseError::NoSuchChannel)
        );
        Ok(channel.get_clients())
    }

    fn get_channels(&self) -> Vec<String> {
        self.channels.keys().cloned().collect()
    }

    fn add_channel_invitation(&mut self, channel: String, client: String) {
        let channel = some_or_return!(self.channels.get_mut(&channel));
        channel.add_client_invite(client);
    }
}

impl<C: Connection> Database<C> {
    fn create_channel(&mut self, channel: String, nickname: String) {
        let channel = Channel::new(channel, nickname);
        let name = channel.name();
        self.channels.insert(name, channel);
    }
}
