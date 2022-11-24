mod database_handle;
mod database_message;
mod handlers;

#[cfg(test)]
mod tests;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::mpsc::{self, Receiver};
use std::thread::{self, JoinHandle};

use crate::server::data_structures::*;

use database_message::DatabaseMessage::*;

pub use database_handle::DatabaseHandle;
use database_message::DatabaseMessage;

use super::connection::Connection;
/// Represents a Database that implements ClientTrait.
pub struct Database<C: Connection> {
    receiver: Receiver<DatabaseMessage<C>>,
    clients: HashMap<String, Rc<RefCell<Client<C>>>>,
    channels: HashMap<String, Channel<C>>,
    servers: HashMap<String, ExternalServer<C>>,
    credentials: HashMap<String, String>,
    servername: String,
    serverinfo: String,
}

impl<C: Connection> Database<C> {
    /// Returns new [`DatabaseHandle`] and starts listening for requests.
    pub fn start(servername: String, serverinfo: String) -> (DatabaseHandle<C>, JoinHandle<()>) {
        let (sender, receiver) = mpsc::channel();

        let join_handle =
            thread::spawn(|| Database::<C>::new(receiver, servername, serverinfo).run());
        let database_handle = DatabaseHandle::new(sender);

        (database_handle, join_handle)
    }

    fn new(receiver: Receiver<DatabaseMessage<C>>, servername: String, serverinfo: String) -> Self {
        let mut database = Self {
            receiver,
            clients: HashMap::new(),
            channels: HashMap::new(),
            servers: HashMap::new(),
            credentials: HashMap::new(),
            servername,
            serverinfo,
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
            AddClient { client } => self.handle_add_client(client),
            GetStream {
                nickname,
                respond_to: response,
            } => self.handle_get_stream_request(&nickname, response),
            DisconnectClient { nickname } => self.disconnect_client(&nickname),
            SetServerOperator { nickname } => self.handle_set_server_operator(&nickname),
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
            AreCredentialsValid {
                username,
                password,
                respond_to,
            } => self.handle_are_credentials_valid(&username, &password, respond_to),
            SetAwayMessage { message, nickname } => {
                self.handle_set_away_message(&message, &nickname)
            }
            GetAwayMessage {
                nickname,
                respond_to,
            } => self.handle_get_away_message(&nickname, respond_to),
            SetChannelTopic { channel, topic } => self.set_channel_topic(&channel, &topic),
            GetChannelTopic {
                channel,
                respond_to,
            } => self.handle_get_channel_topic(&channel, respond_to),
            SetChannelKey { channel, key } => self.handle_set_channel_key(channel, key),
            GetChannelKey {
                channel,
                respond_to,
            } => self.handle_get_channel_key(channel, respond_to),
            SetChannelMode { channel, flag } => self.handle_set_mode(channel, flag),
            UnsetChannelMode { channel, flag } => self.handle_unset_mode(channel, flag),
            ChannelHasMode {
                channel,
                respond_to,
                flag,
            } => self.handle_channel_has_mode(channel, flag, respond_to),
            SetLimit { channel, limit } => self.handle_set_channel_limit(channel, limit),
            GetLimit {
                channel,
                respond_to,
            } => self.handle_get_channel_limit(channel, respond_to),
            AddChanop { channel, nickname } => self.handle_add_channop(channel, nickname),
            RemoveChanop { channel, nickname } => self.handle_remove_channop(channel, nickname),
            AddSpeaker { channel, nickname } => self.handle_add_speaker(channel, nickname),
            RemoveSpeaker { channel, nickname } => self.handle_remove_speaker(channel, nickname),
            IsChannelSpeaker {
                channel,
                nickname,
                respond_to,
            } => self.handle_is_channel_speaker(channel, nickname, respond_to),
            AddChannelBanMask { channel, mask } => self.handle_add_channel_banmask(channel, mask),
            GetChannelBanMask {
                channel,
                respond_to,
            } => self.handle_get_channel_banmask(channel, respond_to),
            RemoveChannelBanMask { channel, mask } => {
                self.handle_remove_channel_banmask(channel, mask)
            }
            // DatabaseMessage::GetAllChannelModes {
            //     channel,
            //     respond_to,
            // } => self.handle_get_all_channel_modes(channel, respond_to),
            IsChannelOperator {
                channel,
                nickname,
                respond_to,
            } => self.handle_is_channel_operator(&channel, &nickname, respond_to),
            ClientMatchesBanmask {
                nickname,
                mask,
                respond_to,
            } => self.handle_clients_matches_banmask(&nickname, &mask, respond_to),
            ContainsServer {
                servername,
                respond_to,
            } => self.handle_contains_server(&servername, respond_to),
            AddServer { server } => self.handle_add_server(server),
            AddExternalClient { server, client } => {
                self.handle_add_external_client(&server, client)
            }
            GetServerName { respond_to } => self.handle_get_servername(respond_to),
            GetServerInfo { respond_to } => self.handle_get_serverinfo(respond_to),
            GetChannelConfig {
                channel,
                respond_to,
            } => self.handle_get_channel_config(channel, respond_to),
            GetServerStream { server, respond_to } => {
                self.handle_get_server_stream(server, respond_to)
            }
            GetAllServers { respond_to } => self.handle_get_all_servers(respond_to),
            GetLocalClientsForChannel {
                channel,
                respond_to,
            } => self.handle_get_local_clients_for_channel(channel, respond_to),
        }
    }
}
