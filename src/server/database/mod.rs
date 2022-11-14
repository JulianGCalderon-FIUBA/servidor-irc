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

use super::client_trait::Connection;
/// Represents a Database that implements ClientTrait.
pub struct Database<C: Connection> {
    receiver: Receiver<DatabaseMessage<C>>,
    clients: HashMap<String, Rc<RefCell<Client<C>>>>,
    channels: HashMap<String, Channel<C>>,
    credentials: HashMap<String, String>,
}

impl<C: Connection> Database<C> {
    /// Returns new [`DatabaseHandle`] and starts listening for requests.
    pub fn start() -> DatabaseHandle<C> {
        let (sender, receiver) = mpsc::channel();

        thread::spawn(|| Database::<C>::new(receiver).run());

        DatabaseHandle::new(sender)
    }

    fn new(receiver: Receiver<DatabaseMessage<C>>) -> Self {
        let mut database = Self {
            receiver,
            clients: HashMap::new(),
            channels: HashMap::new(),
            credentials: HashMap::new(),
        };

        database
            .credentials
            .insert("admin".to_string(), "admin".to_string());

        database
    }

    fn run(mut self) {
        while let Ok(request) = self.receiver.recv() {
            self.handle_message(request);
        }
    }

    fn handle_message(&mut self, request: DatabaseMessage<C>) {
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
                respond_to,
            } => self.handle_is_client_in_channel(&nickname, &channel, respond_to),
            GetChannelsForClient {
                nickname,
                respond_to,
            } => self.handle_get_channels_for_client(&nickname, respond_to),
            GetClientsFromChannel {
                channel,
                respond_to,
            } => self.handle_get_clients_for_channel(&channel, respond_to),
            GetAllClients { respond_to } => self.handle_get_all_clients(respond_to),
            GetAllChannels { respond_to } => self.handle_get_all_channels(respond_to),
            GetClientsForMask { mask, respond_to } => {
                self.handle_get_clients_for_mask(&mask, respond_to)
            }
            GetClientsForNickMask {
                nickmask,
                respond_to,
            } => self.handle_get_clients_for_nickmask(&nickmask, respond_to),
            UpdateNickname {
                old_nickname,
                new_nickname,
            } => self.handle_update_nickname(&old_nickname, &new_nickname),
            DatabaseMessage::AreCredentialsValid {
                username,
                password,
                respond_to,
            } => self.handle_are_credentials_valid(&username, &password, respond_to),
            DatabaseMessage::SetAwayMessage { message, nickname } => {
                self.handle_set_away_message(&message, &nickname)
            }
            DatabaseMessage::GetAwayMessage {
                nickname,
                respond_to,
            } => self.handle_get_away_message(&nickname, respond_to),
            DatabaseMessage::SetChannelTopic { channel, topic } => {
                self.set_channel_topic(&channel, &topic)
            }
            DatabaseMessage::GetChannelTopic {
                channel,
                respond_to,
            } => self.handle_get_channel_topic(&channel, respond_to),
            DatabaseMessage::SetChannelKey { channel, key } => {
                self.handle_set_channel_key(channel, key)
            }
            DatabaseMessage::GetChannelKey {
                channel,
                respond_to,
            } => self.handle_get_channel_key(channel, respond_to),
            DatabaseMessage::SetChannelMode { channel, mode } => {
                self.handle_set_mode(channel, mode)
            }
            DatabaseMessage::UnsetChannelMode { channel, mode } => {
                self.handle_unset_mode(channel, mode)
            }
            DatabaseMessage::ChannelHasMode {
                channel,
                mode,
                respond_to,
            } => self.handle_channel_has_mode(channel, mode, respond_to),
            DatabaseMessage::SetLimit { channel, limit } => {
                self.handle_set_channel_limit(channel, limit)
            }
            DatabaseMessage::GetLimit {
                channel,
                respond_to,
            } => self.handle_get_channel_limit(channel, respond_to),
            DatabaseMessage::AddChanop { channel, nickname } => {
                self.handle_add_channop(channel, nickname)
            }
            DatabaseMessage::RemoveChanop { channel, nickname } => {
                self.handle_remove_channop(channel, nickname)
            }
            DatabaseMessage::AddSpeaker { channel, nickname } => {
                self.handle_add_speaker(channel, nickname)
            }
            DatabaseMessage::RemoveSpeaker { channel, nickname } => {
                self.handle_remove_speaker(channel, nickname)
            }
            DatabaseMessage::IsChannelSpeaker {
                channel,
                nickname,
                respond_to,
            } => self.handle_is_channel_speaker(channel, nickname, respond_to),
            DatabaseMessage::SetChannelBanMask { channel, mask } => {
                self.handle_set_channel_banmask(channel, mask)
            }
            DatabaseMessage::GetChannelBanMask {
                channel,
                respond_to,
            } => self.handle_get_channel_banmask(channel, respond_to),
            DatabaseMessage::UnsetChannelBanMask { channel, mask } => {
                self.handle_unset_channel_banmask(channel, mask)
            }
            // DatabaseMessage::GetAllChannelModes {
            //     channel,
            //     respond_to,
            // } => self.handle_get_all_channel_modes(channel, respond_to),
            DatabaseMessage::IsChannelOperator {
                channel,
                nickname,
                respond_to,
            } => self.handle_is_channel_operator(&channel, &nickname, respond_to),
        }
    }
}
