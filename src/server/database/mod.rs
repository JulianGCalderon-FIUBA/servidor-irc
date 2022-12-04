/// This module contains different errors that can happen when ... a Database.
mod database_error;
/// This module contains different handles a Database has for each request.
/// External parts communicate with the Database through the handles.
mod database_handle;
/// This module contains messages for each request a Database can handle.
mod database_message;
/// This module contains specific functions to handle each request.
mod handlers;

/// Unit tests for the Database's basic functionalities.
#[cfg(test)]
mod tests;

use std::collections::HashMap;
use std::sync::mpsc::{self, Receiver};
use std::thread::{self, JoinHandle};

use crate::server::data_structures::*;

use database_message::DatabaseMessage::*;

pub use database_handle::DatabaseHandle;
use database_message::DatabaseMessage;

use super::connection::Connection;
/// Represents a Database that stores all information a server should have.
pub struct Database<C: Connection> {
    receiver: Receiver<DatabaseMessage<C>>,
    info: ServerInfo,
    credentials: HashMap<String, String>,

    local_clients: HashMap<String, LocalClient<C>>,
    external_clients: HashMap<String, ExternalClient>,
    channels: HashMap<String, Channel>,

    immediate_servers: HashMap<String, ImmediateServer<C>>,
    distant_servers: HashMap<String, ServerInfo>,
}

impl<C: Connection> Database<C> {
    /// Creates [`Database`] for a specific server and stores receiver end from which to listen for requests.
    fn new(receiver: Receiver<DatabaseMessage<C>>, servername: String, serverinfo: String) -> Self {
        let mut database = Self {
            receiver,
            info: ServerInfo::new(servername, serverinfo, 0),
            credentials: Default::default(),
            local_clients: Default::default(),
            external_clients: Default::default(),
            channels: Default::default(),
            immediate_servers: Default::default(),
            distant_servers: Default::default(),
        };

        database
            .credentials
            .insert("admin".to_string(), "admin".to_string());

        database
    }

    /// Returns new [`DatabaseHandle`] and spawns thread that is listening for requests.
    pub fn start(servername: String, serverinfo: String) -> (DatabaseHandle<C>, JoinHandle<()>) {
        let (sender, receiver) = mpsc::channel();

        let join_handle =
            thread::spawn(|| Database::<C>::new(receiver, servername, serverinfo).run());
        let database_handle = DatabaseHandle::new(sender);

        (database_handle, join_handle)
    }

    fn run(mut self) {
        while let Ok(request) = self.receiver.recv() {
            self.handle_message(request);
        }
    }

    /// Calls specific handle for received request.
    fn handle_message(&mut self, request: DatabaseMessage<C>) {
        match request {
            DisconnectClient { nickname } => self.handle_disconnect_client(nickname),
            SetServerOperator { nickname } => self.handle_set_server_operator(nickname),
            IsServerOperator {
                nickname,
                respond_to: response,
            } => self.handle_is_server_operator(nickname, response),
            ContainsClient {
                nickname,
                respond_to: response,
            } => self.handle_contains_client(nickname, response),
            ContainsChannel {
                channel,
                respond_to: response,
            } => self.handle_contains_channel(channel, response),
            AddClientToChannel { nickname, channel } => {
                self.handle_add_client_to_channel(nickname, channel)
            }
            RemoveClientFromChannel { nickname, channel } => {
                self.handle_remove_client_from_channel(nickname, channel)
            }
            IsClientInChannel {
                nickname,
                channel,
                respond_to,
            } => self.handle_is_client_in_channel(nickname, channel, respond_to),
            GetChannelsForClient {
                nickname,
                respond_to,
            } => self.handle_get_channels_for_client(nickname, respond_to),
            GetChannelClients {
                channel,
                respond_to,
            } => self.handle_get_channel_clients(channel, respond_to),
            GetAllClients { respond_to } => self.handle_get_all_clients(respond_to),
            GetAllChannels { respond_to } => self.handle_get_all_channels(respond_to),
            UpdateNickname {
                old_nickname,
                new_nickname,
            } => self.handle_update_nickname(old_nickname, new_nickname),
            AreCredentialsValid {
                username,
                password,
                respond_to,
            } => self.handle_are_credentials_valid(username, password, respond_to),
            SetAwayMessage { message, nickname } => self.handle_set_away_message(message, nickname),
            GetAwayMessage {
                nickname,
                respond_to,
            } => self.handle_get_away_message(nickname, respond_to),
            SetChannelTopic { channel, topic } => self.handle_set_channel_topic(channel, topic),
            GetChannelTopic {
                channel,
                respond_to,
            } => self.handle_get_channel_topic(channel, respond_to),
            SetChannelKey { channel, key } => self.handle_set_channel_key(channel, key),
            GetChannelKey {
                channel,
                respond_to,
            } => self.handle_get_channel_key(channel, respond_to),
            SetChannelFlag { channel, flag } => self.handle_set_channel_flag(channel, flag),
            UnsetChannelFlag { channel, flag } => self.handle_unset_channel_flag(channel, flag),
            ChannelHasFlag {
                channel,
                respond_to,
                flag,
            } => self.handle_channel_has_flag(channel, flag, respond_to),
            SetChannelLimit { channel, limit } => self.handle_set_channel_limit(channel, limit),
            GetChannelLimit {
                channel,
                respond_to,
            } => self.handle_get_channel_limit(channel, respond_to),
            AddChannelOperator { channel, nickname } => self.handle_add_channop(channel, nickname),
            RemoveChannelOperator { channel, nickname } => {
                self.handle_remove_channop(channel, nickname)
            }
            AddChannelSpeaker { channel, nickname } => {
                self.handle_add_channel_speaker(channel, nickname)
            }
            RemoveChannelSpeaker { channel, nickname } => {
                self.handle_remove_channel_speaker(channel, nickname)
            }
            IsChannelSpeaker {
                channel,
                nickname,
                respond_to,
            } => self.handle_is_channel_speaker(channel, nickname, respond_to),
            AddChannelBanmask { channel, mask } => self.handle_add_channel_banmask(channel, mask),
            GetChannelBanmask {
                channel,
                respond_to,
            } => self.handle_get_channel_banmask(channel, respond_to),
            RemoveChannelBanmask { channel, mask } => {
                self.handle_remove_channel_banmask(channel, mask)
            }
            IsChannelOperator {
                channel,
                nickname,
                respond_to,
            } => self.handle_is_channel_operator(channel, nickname, respond_to),
            ContainsServer {
                servername,
                respond_to,
            } => self.handle_contains_server(servername, respond_to),
            AddExternalClient { client } => self.handle_add_external_client(client),
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
            AddDistantServer { server } => self.handle_add_distant_server(server),
            AddImmediateServer { server } => self.handle_add_immediate_server(server),
            AddLocalClient { client } => self.handle_add_local_client(client),
            GetLocalStream {
                nickname,
                respond_to,
            } => self.handle_get_local_stream_request(nickname, respond_to),
            IsLocalClient {
                nickname,
                respond_to,
            } => self.handle_is_local_client(nickname, respond_to),
            GetImmediateServer { client, respond_to } => {
                self.handle_get_immediate_server(client, respond_to)
            }
            GetClientInfo { client, respond_to } => self.handle_get_client_info(client, respond_to),
            IsImmediateServer { server, respond_to } => {
                self.handle_is_immediate_server(server, respond_to)
            }
            RemoveServer { servername } => self.handle_remove_server(servername),
            SetUserFlag { user, flag } => self.handle_set_user_flag(user, flag),
            UnsetUserFlag { user, flag } => self.handle_unset_user_flag(user, flag),
            AddChannelInvite { channel, client } => {
                self.handle_add_channel_invitation(channel, client)
            }
            ChannelHasClientInvite {
                channel,
                client,
                respond_to,
            } => self.handle_channel_has_client_invite(channel, client, respond_to),
        }
    }
}
