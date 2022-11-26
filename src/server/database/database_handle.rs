use std::sync::mpsc::{self, Sender};

use crate::server::data_structures::*;
use crate::server::{connection::Connection, consts::modes::ChannelFlag};

use super::{database_error::DatabaseError, database_message::DatabaseMessage};

/// A DatabaseHandle handles and makes request to the main Database. Works as an intermediary so external structures cannot acces the Database directly.
pub struct DatabaseHandle<C: Connection> {
    sender: Sender<DatabaseMessage<C>>,
}

impl<C: Connection> DatabaseHandle<C> {
    pub fn add_channop(&self, channel: &str, nickname: &str) {
        let request = DatabaseMessage::AddChanop {
            channel: channel.to_string(),
            nickname: nickname.to_string(),
        };
        self.sender.send(request).unwrap();
    }

    /// Sends AddClient request.
    pub fn add_local_client(&self, client: LocalClient<C>) {
        let request = DatabaseMessage::AddLocalClient { client };
        self.sender.send(request).unwrap();
    }

    /// Sends AddClient request.
    pub fn add_external_client(&self, client: ExternalClient) {
        let request = DatabaseMessage::AddExternalClient { client };
        self.sender.send(request).unwrap();
    }

    /// Sends AddClientToChannel request.
    pub fn add_client_to_channel(&self, nickname: &str, channel_name: &str) {
        let request = DatabaseMessage::AddClientToChannel {
            nickname: nickname.to_string(),
            channel: channel_name.to_string(),
        };
        self.sender.send(request).unwrap();
    }

    pub fn add_immediate_server(&self, server: ImmediateServer<C>) {
        let request = DatabaseMessage::AddImmediateServer { server };
        self.sender.send(request).unwrap();
    }

    pub fn add_distant_server(&self, server: ServerInfo) {
        let request = DatabaseMessage::AddDistantServer { server };
        self.sender.send(request).unwrap();
    }

    pub fn add_speaker(&self, channel: &str, nickname: &str) {
        let request = DatabaseMessage::AddSpeaker {
            channel: channel.to_string(),
            nickname: nickname.to_string(),
        };
        self.sender.send(request).unwrap();
    }

    /// Sends AreCredentialsValid request and returns answer.
    pub fn are_credentials_valid(&self, username: &str, password: &str) -> bool {
        let (sender, receiver) = mpsc::channel();
        let request = DatabaseMessage::AreCredentialsValid {
            username: username.to_string(),
            password: password.to_string(),
            respond_to: sender,
        };
        self.sender.send(request).unwrap();
        receiver.recv().unwrap()
    }

    pub fn channel_has_mode(&self, channel: &str, flag: &ChannelFlag) -> bool {
        let (sender, receiver) = mpsc::channel();
        let request = DatabaseMessage::ChannelHasMode {
            channel: channel.to_string(),
            flag: flag.clone(),
            respond_to: sender,
        };
        self.sender.send(request).unwrap();
        receiver.recv().unwrap()
    }

    /// Sends ContainsChannel request and returns answer.
    pub fn contains_channel(&self, channel: &str) -> bool {
        let (sender, receiver) = mpsc::channel();
        let request = DatabaseMessage::ContainsChannel {
            channel: channel.to_string(),
            respond_to: sender,
        };
        self.sender.send(request).unwrap();
        receiver.recv().unwrap()
    }

    /// Sends ContainsClient request and returns answer.
    pub fn contains_client(&self, nickname: &str) -> bool {
        let (sender, receiver) = mpsc::channel();
        let request = DatabaseMessage::ContainsClient {
            nickname: nickname.to_string(),
            respond_to: sender,
        };
        self.sender.send(request).unwrap();
        receiver.recv().unwrap()
    }

    pub fn contains_server(&self, servername: &str) -> bool {
        let (sender, receiver) = mpsc::channel();
        let request = DatabaseMessage::ContainsServer {
            servername: servername.to_string(),
            respond_to: sender,
        };
        self.sender.send(request).unwrap();
        receiver.recv().unwrap()
    }

    /// Sends DisconnectClient request.
    pub fn disconnect_client(&self, nickname: &str) {
        let request = DatabaseMessage::DisconnectClient {
            nickname: nickname.to_string(),
        };
        self.sender.send(request).unwrap();
    }

    /// Sends GetAllChannels request and returns answer.
    pub fn get_all_channels(&self) -> Vec<String> {
        let (sender, receiver) = mpsc::channel();
        let request = DatabaseMessage::GetAllChannels { respond_to: sender };
        self.sender.send(request).unwrap();
        receiver.recv().unwrap()
    }

    /// Sends GetAllClients request and returns answer.
    pub fn get_all_clients(&self) -> Vec<ClientInfo> {
        let (sender, receiver) = mpsc::channel();
        let request = DatabaseMessage::GetAllClients { respond_to: sender };
        self.sender.send(request).unwrap();
        receiver.recv().unwrap()
    }

    pub fn get_away_message(&self, nickname: &str) -> Result<Option<String>, DatabaseError> {
        let (sender, receiver) = mpsc::channel();
        let request = DatabaseMessage::GetAwayMessage {
            nickname: nickname.to_string(),
            respond_to: sender,
        };
        self.sender.send(request).unwrap();
        receiver.recv().unwrap()
    }

    pub fn get_channel_banmask(&self, channel: &str) -> Result<Vec<String>, DatabaseError> {
        let (sender, receiver) = mpsc::channel();
        let request = DatabaseMessage::GetChannelBanMask {
            channel: channel.to_string(),
            respond_to: sender,
        };
        self.sender.send(request).unwrap();
        receiver.recv().unwrap()
    }

    pub fn get_channel_key(&self, channel: &str) -> Result<Option<String>, DatabaseError> {
        let (sender, receiver) = mpsc::channel();
        let request = DatabaseMessage::GetChannelKey {
            channel: channel.to_string(),
            respond_to: sender,
        };
        self.sender.send(request).unwrap();
        receiver.recv().unwrap()
    }

    pub fn get_channel_limit(&self, channel: &str) -> Result<Option<usize>, DatabaseError> {
        let (sender, receiver) = mpsc::channel();
        let request = DatabaseMessage::GetLimit {
            channel: channel.to_string(),
            respond_to: sender,
        };
        self.sender.send(request).unwrap();
        receiver.recv().unwrap()
    }

    /// Sends GetChannelsForClient request and returns answer.
    pub fn get_channels_for_client(&self, nickname: &str) -> Result<Vec<String>, DatabaseError> {
        let (sender, receiver) = mpsc::channel();
        let request = DatabaseMessage::GetChannelsForClient {
            nickname: nickname.to_string(),
            respond_to: sender,
        };
        self.sender.send(request).unwrap();
        receiver.recv().unwrap()
    }

    /// Sends GetClientsForChannel request and returns answer.
    pub fn get_channel_clients(&self, channel: &str) -> Result<Vec<String>, DatabaseError> {
        let (sender, receiver) = mpsc::channel();
        let request = DatabaseMessage::GetChannelClients {
            channel: channel.to_string(),
            respond_to: sender,
        };
        self.sender.send(request).unwrap();
        receiver.recv().unwrap()
    }

    /// Sends GetStream request and returns answer.
    pub fn get_local_stream(&self, nickname: &str) -> Result<C, DatabaseError> {
        let (sender, receiver) = mpsc::channel();
        let request = DatabaseMessage::GetLocalStream {
            nickname: nickname.to_string(),
            respond_to: sender,
        };
        self.sender.send(request).unwrap();
        receiver.recv().unwrap()
    }

    pub fn get_server_stream(&self, servername: &str) -> Result<C, DatabaseError> {
        let (sender, receiver) = mpsc::channel();
        let request = DatabaseMessage::GetServerStream {
            server: servername.to_string(),
            respond_to: sender,
        };
        self.sender.send(request).unwrap();
        receiver.recv().unwrap()
    }

    pub fn get_topic_for_channel(&self, channel: &str) -> Result<Option<String>, DatabaseError> {
        let (sender, receiver) = mpsc::channel();
        let request = DatabaseMessage::GetChannelTopic {
            channel: channel.to_string(),
            respond_to: sender,
        };
        self.sender.send(request).unwrap();
        receiver.recv().unwrap()
    }

    pub fn is_channel_operator(&self, channel: &str, nick: &str) -> bool {
        let (sender, receiver) = mpsc::channel();
        let request = DatabaseMessage::IsChannelOperator {
            channel: channel.to_string(),
            nickname: nick.to_string(),
            respond_to: sender,
        };
        self.sender.send(request).unwrap();
        receiver.recv().unwrap()
    }

    pub fn is_channel_speaker(&self, channel: &str, nickname: &str) -> bool {
        let (sender, receiver) = mpsc::channel();
        let request = DatabaseMessage::IsChannelSpeaker {
            channel: channel.to_string(),
            nickname: nickname.to_string(),
            respond_to: sender,
        };
        self.sender.send(request).unwrap();
        receiver.recv().unwrap()
    }

    /// Sends IsClientInChannel request and returns answer.
    pub fn is_client_in_channel(&self, nickname: &str, channel: &str) -> bool {
        let (sender, receiver) = mpsc::channel();
        let request = DatabaseMessage::IsClientInChannel {
            nickname: nickname.to_string(),
            channel: channel.to_string(),
            respond_to: sender,
        };
        self.sender.send(request).unwrap();
        receiver.recv().unwrap()
    }

    /// Sends IsServerOperator request and returns answer.
    pub fn is_server_operator(&self, nickname: &str) -> bool {
        let (sender, receiver) = mpsc::channel();
        let request = DatabaseMessage::IsServerOperator {
            nickname: nickname.to_string(),
            respond_to: sender,
        };
        self.sender.send(request).unwrap();
        receiver.recv().unwrap()
    }

    /// Creates new DatabaseHandle
    pub fn new(sender: Sender<DatabaseMessage<C>>) -> Self {
        Self { sender }
    }

    pub fn remove_channop(&self, channel: &str, nickname: &str) {
        let request = DatabaseMessage::RemoveChanop {
            channel: channel.to_string(),
            nickname: nickname.to_string(),
        };
        self.sender.send(request).unwrap();
    }

    /// Sends RemoveClientToChannel request.
    pub fn remove_client_from_channel(&self, nickname: &str, channel_name: &str) {
        let request = DatabaseMessage::RemoveClientFromChannel {
            nickname: nickname.to_string(),
            channel: channel_name.to_string(),
        };
        self.sender.send(request).unwrap();
    }

    pub fn remove_speaker(&self, channel: &str, nickname: &str) {
        let request = DatabaseMessage::RemoveSpeaker {
            channel: channel.to_string(),
            nickname: nickname.to_string(),
        };
        self.sender.send(request).unwrap();
    }

    pub fn set_away_message(&self, message: &Option<String>, nickname: &str) {
        let request = DatabaseMessage::SetAwayMessage {
            message: message.to_owned(),
            nickname: nickname.to_string(),
        };
        self.sender.send(request).unwrap();
    }

    pub fn add_channel_banmask(&self, channel: &str, banmask: &str) {
        let request = DatabaseMessage::AddChannelBanMask {
            channel: channel.to_string(),
            mask: banmask.to_string(),
        };
        self.sender.send(request).unwrap();
    }

    pub fn set_channel_key(&self, channel: &str, key: Option<String>) {
        let request = DatabaseMessage::SetChannelKey {
            channel: channel.to_string(),
            key,
        };
        self.sender.send(request).unwrap();
    }

    pub fn set_channel_limit(&self, channel: &str, limit: Option<usize>) {
        let request = DatabaseMessage::SetLimit {
            channel: channel.to_string(),
            limit,
        };
        self.sender.send(request).unwrap();
    }

    pub fn set_channel_mode(&self, channel: &str, flag: ChannelFlag) {
        let request = DatabaseMessage::SetChannelMode {
            channel: channel.to_string(),
            flag,
        };
        self.sender.send(request).unwrap();
    }

    pub fn set_channel_topic(&self, channel: &str, topic: &str) {
        let request = DatabaseMessage::SetChannelTopic {
            channel: channel.to_string(),
            topic: topic.to_string(),
        };
        self.sender.send(request).unwrap();
    }

    /// Sends SetServerOperator request.
    pub fn set_server_operator(&self, nickname: &str) {
        let request = DatabaseMessage::SetServerOperator {
            nickname: nickname.to_string(),
        };
        self.sender.send(request).unwrap();
    }

    pub fn remove_channel_banmask(&self, channel: &str, banmask: &str) {
        let request = DatabaseMessage::RemoveChannelBanMask {
            channel: channel.to_string(),
            mask: banmask.to_string(),
        };
        self.sender.send(request).unwrap();
    }

    pub fn unset_channel_mode(&self, channel: &str, flag: ChannelFlag) {
        let request = DatabaseMessage::UnsetChannelMode {
            channel: channel.to_string(),
            flag,
        };
        self.sender.send(request).unwrap();
    }

    /// Sends UpdateNickname request and returns answer.
    pub fn update_nickname(&self, old_nickname: &str, new_nickname: &str) {
        let request = DatabaseMessage::UpdateNickname {
            old_nickname: old_nickname.to_string(),
            new_nickname: new_nickname.to_string(),
        };
        self.sender.send(request).unwrap();
    }

    pub fn get_server_name(&self) -> String {
        let (sender, receiver) = mpsc::channel();
        let request = DatabaseMessage::GetServerName { respond_to: sender };
        self.sender.send(request).unwrap();
        receiver.recv().unwrap()
    }

    pub fn get_server_info(&self) -> String {
        let (sender, receiver) = mpsc::channel();
        let request = DatabaseMessage::GetServerInfo { respond_to: sender };
        self.sender.send(request).unwrap();
        receiver.recv().unwrap()
    }

    pub fn get_channel_config(&self, channel: &str) -> Result<ChannelConfiguration, DatabaseError> {
        let (sender, receiver) = mpsc::channel();
        let request = DatabaseMessage::GetChannelConfig {
            channel: channel.to_string(),
            respond_to: sender,
        };
        self.sender.send(request).unwrap();
        receiver.recv().unwrap()
    }

    pub fn get_all_servers(&self) -> Vec<String> {
        let (sender, receiver) = mpsc::channel();
        let request = DatabaseMessage::GetAllServers { respond_to: sender };
        self.sender.send(request).unwrap();
        receiver.recv().unwrap()
    }

    pub(crate) fn is_local_client(&self, client: &str) -> bool {
        let (sender, receiver) = mpsc::channel();
        let request = DatabaseMessage::IsLocalClient {
            nickname: client.to_string(),
            respond_to: sender,
        };
        self.sender.send(request).unwrap();
        receiver.recv().unwrap()
    }

    pub(crate) fn get_immediate_server(&self, client: &str) -> Result<String, DatabaseError> {
        let (sender, receiver) = mpsc::channel();
        let request = DatabaseMessage::GetImmediateServer {
            client: client.to_string(),
            respond_to: sender,
        };
        self.sender.send(request).unwrap();
        receiver.recv().unwrap()
    }

    pub(crate) fn get_client_info(&self, nickname: &str) -> Result<ClientInfo, DatabaseError> {
        let (sender, receiver) = mpsc::channel();
        let request = DatabaseMessage::GetClientInfo {
            client: nickname.to_string(),
            respond_to: sender,
        };
        self.sender.send(request).unwrap();
        receiver.recv().unwrap()
    }
}

impl<C: Connection> Clone for DatabaseHandle<C> {
    fn clone(&self) -> Self {
        Self {
            sender: self.sender.clone(),
        }
    }
}
