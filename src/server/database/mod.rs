mod channel;
mod client;
mod database_handle;
mod database_message;
mod handlers;

#[cfg(test)]
mod tests;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::mpsc::{self, Receiver};
use std::thread;

pub use channel::Channel;
pub use client::{Client, ClientBuilder, ClientInfo};
pub use database_handle::DatabaseHandle;

use database_message::DatabaseMessage::{
    AddClient, AddClientToChannel, ContainsChannel, ContainsClient, DisconnectClient,
    GetAllChannels, GetAllClients, GetChannelsForClient, GetClientsForMask, GetClientsForNickMask,
    GetClientsFromChannel, GetStream, IsClientInChannel, IsServerOperator, RemoveClientFromChannel,
    SetServerOperator, UpdateNickname,
};

use self::database_message::DatabaseMessage;

use super::client_trait::ClientTrait;
/// Represents a Database that implements ClientTrait.
pub struct Database<T: ClientTrait> {
    receiver: Receiver<DatabaseMessage<T>>,
    clients: HashMap<String, Rc<RefCell<Client<T>>>>,
    channels: HashMap<String, Channel<T>>,
}

impl<T: ClientTrait> Database<T> {
    /// Returns new [`DatabaseHandle`] and starts listening for requests.
    pub fn start() -> DatabaseHandle<T> {
        let (sender, receiver) = mpsc::channel();

        thread::spawn(|| {
            let database = Self {
                receiver,
                clients: HashMap::new(),
                channels: HashMap::new(),
            };

            database.run()
        });

        DatabaseHandle::new(sender)
    }

    fn run(mut self) {
        while let Ok(request) = self.receiver.recv() {
            self.handle_message(request);
        }
    }

    fn handle_message(&mut self, request: DatabaseMessage<T>) {
        match request {
            AddClient { client } => self.add_client(client),
            GetStream {
                nickname,
                respond_to: response,
            } => self.handle_get_stream_request(&nickname, response),
            DisconnectClient { nickname } => self.disconnect_client(&nickname),
            SetServerOperator { nickname } => self.set_server_operator(&nickname),
            IsServerOperator {
                nickname,
                respond_to: response,
            } => self.handle_is_server_operator(&nickname, response),
            //IsOnline { nickname, response } => self.is_online_request(&nickname, response),
            ContainsClient {
                nickname,
                respond_to: response,
            } => self.handle_contains_client_request(&nickname, response),
            ContainsChannel {
                channel,
                respond_to: response,
            } => self.handle_contains_channel(&channel, response),
            AddClientToChannel { nickname, channel } => {
                self.add_client_to_channel(&nickname, &channel)
            }
            RemoveClientFromChannel { nickname, channel } => {
                self.remove_client_from_channel(&nickname, &channel)
            }
            IsClientInChannel {
                nickname,
                channel,
                respond_to: response,
            } => self.handle_is_client_in_channel(&nickname, &channel, response),
            GetChannelsForClient {
                nickname,
                respond_to: response,
            } => self.handle_get_channels_for_client(&nickname, response),
            GetClientsFromChannel {
                channel,
                respond_to: response,
            } => self.handle_get_clients_for_channel(&channel, response),
            GetAllClients {
                respond_to: response,
            } => self.handle_get_all_clients(response),
            GetAllChannels {
                respond_to: response,
            } => self.handle_get_all_channels(response),
            GetClientsForMask {
                mask,
                respond_to: response,
            } => self.handle_get_clients_for_mask(&mask, response),
            GetClientsForNickMask {
                nickmask,
                respond_to: response,
            } => self.handle_get_clients_for_nickmask(&nickmask, response),
            UpdateNickname {
                old_nickname,
                new_nickname,
            } => self.handle_update_nickname(&old_nickname, &new_nickname),
        }
    }
}
