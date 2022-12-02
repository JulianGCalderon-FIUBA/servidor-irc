use std::sync::mpsc::Sender;

use crate::macros::own;
use crate::server::consts::modes::UserFlag;
use crate::server::data_structures::*;
use crate::server::{connection::Connection, consts::modes::ChannelFlag};

use super::{database_error::DatabaseError, database_message::DatabaseMessage};

/// A DatabaseHandle handles and makes request to the main Database. Works as an intermediary so external structures cannot acces the Database directly.
pub struct DatabaseHandle<C: Connection> {
    sender: Sender<DatabaseMessage<C>>,
}

impl<C: Connection> DatabaseHandle<C> {
    pub fn new(sender: Sender<DatabaseMessage<C>>) -> Self {
        Self { sender }
    }
    pub fn add_channel_banmask(&self, channel: &str, mask: &str) {
        let channel = channel.to_string();
        let mask = mask.to_string();
        let request = DatabaseMessage::AddChannelBanmask { channel, mask };
        self.sender
            .send(request)
            .expect("Database receiver should not be dropped");
    }
    pub fn add_channel_operator(&self, channel: &str, nickname: &str) {
        let channel = channel.to_string();
        let nickname = nickname.to_string();
        let request = DatabaseMessage::AddChannelOperator { channel, nickname };
        self.sender
            .send(request)
            .expect("Database receiver should not be dropped");
    }
    pub fn add_channel_speaker(&self, channel: &str, nickname: &str) {
        let channel = channel.to_string();
        let nickname = nickname.to_string();
        let request = DatabaseMessage::AddChannelSpeaker { channel, nickname };
        self.sender
            .send(request)
            .expect("Database receiver should not be dropped");
    }
    pub fn add_client_to_channel(&self, channel: &str, nickname: &str) {
        let channel = channel.to_string();
        let nickname = nickname.to_string();
        let request = DatabaseMessage::AddClientToChannel { channel, nickname };
        self.sender
            .send(request)
            .expect("Database receiver should not be dropped");
    }
    pub fn add_distant_server(&self, server: ServerInfo) {
        let request = DatabaseMessage::AddDistantServer { server };
        self.sender
            .send(request)
            .expect("Database receiver should not be dropped");
    }
    pub fn add_external_client(&self, client: ExternalClient) {
        let request = DatabaseMessage::AddExternalClient { client };
        self.sender
            .send(request)
            .expect("Database receiver should not be dropped");
    }
    pub fn add_immediate_server(&self, server: ImmediateServer<C>) {
        let request = DatabaseMessage::AddImmediateServer { server };
        self.sender
            .send(request)
            .expect("Database receiver should not be dropped");
    }
    pub fn add_local_client(&self, client: LocalClient<C>) {
        let request = DatabaseMessage::AddLocalClient { client };
        self.sender
            .send(request)
            .expect("Database receiver should not be dropped");
    }
    pub fn are_credentials_valid(&self, username: &str, password: &str) -> bool {
        let (respond_to, receive_from) = std::sync::mpsc::channel();
        let username = username.to_string();
        let password = password.to_string();
        let request = DatabaseMessage::AreCredentialsValid {
            username,
            password,
            respond_to,
        };
        self.sender
            .send(request)
            .expect("Database receiver should not be dropped");
        receive_from
            .recv()
            .expect("Handler receiver should not be dropped")
    }
    pub fn channel_has_flag(&self, channel: &str, flag: ChannelFlag) -> bool {
        let (respond_to, receive_from) = std::sync::mpsc::channel();
        let channel = channel.to_string();
        let request = DatabaseMessage::ChannelHasFlag {
            channel,
            flag,
            respond_to,
        };
        self.sender
            .send(request)
            .expect("Database receiver should not be dropped");
        receive_from
            .recv()
            .expect("Handler receiver should not be dropped")
    }
    pub fn contains_channel(&self, channel: &str) -> bool {
        let (respond_to, receive_from) = std::sync::mpsc::channel();
        let channel = channel.to_string();
        let request = DatabaseMessage::ContainsChannel {
            channel,
            respond_to,
        };
        self.sender
            .send(request)
            .expect("Database receiver should not be dropped");
        receive_from
            .recv()
            .expect("Handler receiver should not be dropped")
    }
    pub fn contains_client(&self, nickname: &str) -> bool {
        let (respond_to, receive_from) = std::sync::mpsc::channel();
        let nickname = nickname.to_string();
        let request = DatabaseMessage::ContainsClient {
            nickname,
            respond_to,
        };
        self.sender
            .send(request)
            .expect("Database receiver should not be dropped");
        receive_from
            .recv()
            .expect("Handler receiver should not be dropped")
    }
    pub fn contains_server(&self, servername: &str) -> bool {
        let (respond_to, receive_from) = std::sync::mpsc::channel();
        let servername = servername.to_string();
        let request = DatabaseMessage::ContainsServer {
            servername,
            respond_to,
        };
        self.sender
            .send(request)
            .expect("Database receiver should not be dropped");
        receive_from
            .recv()
            .expect("Handler receiver should not be dropped")
    }
    pub fn disconnect_client(&self, nickname: &str) {
        let nickname = nickname.to_string();
        let request = DatabaseMessage::DisconnectClient { nickname };
        self.sender
            .send(request)
            .expect("Database receiver should not be dropped");
    }
    pub fn get_all_channels(&self) -> Vec<String> {
        let (respond_to, receive_from) = std::sync::mpsc::channel();
        let request = DatabaseMessage::GetAllChannels { respond_to };
        self.sender
            .send(request)
            .expect("Database receiver should not be dropped");
        receive_from
            .recv()
            .expect("Handler receiver should not be dropped")
    }
    pub fn get_all_clients(&self) -> Vec<ClientInfo> {
        let (respond_to, receive_from) = std::sync::mpsc::channel();
        let request = DatabaseMessage::GetAllClients { respond_to };
        self.sender
            .send(request)
            .expect("Database receiver should not be dropped");
        receive_from
            .recv()
            .expect("Handler receiver should not be dropped")
    }
    pub fn get_all_servers(&self) -> Vec<String> {
        let (respond_to, receive_from) = std::sync::mpsc::channel();
        let request = DatabaseMessage::GetAllServers { respond_to };
        self.sender
            .send(request)
            .expect("Database receiver should not be dropped");
        receive_from
            .recv()
            .expect("Handler receiver should not be dropped")
    }
    pub fn get_away_message(&self, nickname: &str) -> Result<Option<String>, DatabaseError> {
        let (respond_to, receive_from) = std::sync::mpsc::channel();
        let nickname = nickname.to_string();
        let request = DatabaseMessage::GetAwayMessage {
            nickname,
            respond_to,
        };
        self.sender
            .send(request)
            .expect("Database receiver should not be dropped");
        receive_from
            .recv()
            .expect("Handler receiver should not be dropped")
    }
    pub fn get_channel_banmask(&self, channel: &str) -> Result<Vec<String>, DatabaseError> {
        let (respond_to, receive_from) = std::sync::mpsc::channel();
        let channel = channel.to_string();
        let request = DatabaseMessage::GetChannelBanmask {
            channel,
            respond_to,
        };
        self.sender
            .send(request)
            .expect("Database receiver should not be dropped");
        receive_from
            .recv()
            .expect("Handler receiver should not be dropped")
    }
    pub fn get_channel_clients(&self, channel: &str) -> Result<Vec<String>, DatabaseError> {
        let (respond_to, receive_from) = std::sync::mpsc::channel();
        let channel = channel.to_string();
        let request = DatabaseMessage::GetChannelClients {
            channel,
            respond_to,
        };
        self.sender
            .send(request)
            .expect("Database receiver should not be dropped");
        receive_from
            .recv()
            .expect("Handler receiver should not be dropped")
    }
    pub fn get_channel_config(&self, channel: &str) -> Result<ChannelConfiguration, DatabaseError> {
        let (respond_to, receive_from) = std::sync::mpsc::channel();
        let channel = channel.to_string();
        let request = DatabaseMessage::GetChannelConfig {
            channel,
            respond_to,
        };
        self.sender
            .send(request)
            .expect("Database receiver should not be dropped");
        receive_from
            .recv()
            .expect("Handler receiver should not be dropped")
    }
    pub fn get_channel_key(&self, channel: &str) -> Result<Option<String>, DatabaseError> {
        let (respond_to, receive_from) = std::sync::mpsc::channel();
        let channel = channel.to_string();
        let request = DatabaseMessage::GetChannelKey {
            channel,
            respond_to,
        };
        self.sender
            .send(request)
            .expect("Database receiver should not be dropped");
        receive_from
            .recv()
            .expect("Handler receiver should not be dropped")
    }
    pub fn get_channel_limit(&self, channel: &str) -> Result<Option<usize>, DatabaseError> {
        let (respond_to, receive_from) = std::sync::mpsc::channel();
        let channel = channel.to_string();
        let request = DatabaseMessage::GetChannelLimit {
            channel,
            respond_to,
        };
        self.sender
            .send(request)
            .expect("Database receiver should not be dropped");
        receive_from
            .recv()
            .expect("Handler receiver should not be dropped")
    }
    pub fn get_channel_topic(&self, channel: &str) -> Result<Option<String>, DatabaseError> {
        let (respond_to, receive_from) = std::sync::mpsc::channel();
        let channel = channel.to_string();
        let request = DatabaseMessage::GetChannelTopic {
            channel,
            respond_to,
        };
        self.sender
            .send(request)
            .expect("Database receiver should not be dropped");
        receive_from
            .recv()
            .expect("Handler receiver should not be dropped")
    }
    pub fn get_channels_for_client(&self, nickname: &str) -> Result<Vec<String>, DatabaseError> {
        let (respond_to, receive_from) = std::sync::mpsc::channel();
        let nickname = nickname.to_string();
        let request = DatabaseMessage::GetChannelsForClient {
            nickname,
            respond_to,
        };
        self.sender
            .send(request)
            .expect("Database receiver should not be dropped");
        receive_from
            .recv()
            .expect("Handler receiver should not be dropped")
    }
    pub fn get_client_info(&self, client: &str) -> Result<ClientInfo, DatabaseError> {
        let (respond_to, receive_from) = std::sync::mpsc::channel();
        let client = client.to_string();
        let request = DatabaseMessage::GetClientInfo { client, respond_to };
        self.sender
            .send(request)
            .expect("Database receiver should not be dropped");
        receive_from
            .recv()
            .expect("Handler receiver should not be dropped")
    }
    pub fn get_immediate_server(&self, client: &str) -> Result<String, DatabaseError> {
        let (respond_to, receive_from) = std::sync::mpsc::channel();
        let client = client.to_string();
        let request = DatabaseMessage::GetImmediateServer { client, respond_to };
        self.sender
            .send(request)
            .expect("Database receiver should not be dropped");
        receive_from
            .recv()
            .expect("Handler receiver should not be dropped")
    }
    pub fn get_local_stream(&self, nickname: &str) -> Result<C, DatabaseError> {
        let (respond_to, receive_from) = std::sync::mpsc::channel();
        let nickname = nickname.to_string();
        let request = DatabaseMessage::GetLocalStream {
            nickname,
            respond_to,
        };
        self.sender
            .send(request)
            .expect("Database receiver should not be dropped");
        receive_from
            .recv()
            .expect("Handler receiver should not be dropped")
    }
    pub fn get_server_info(&self) -> String {
        let (respond_to, receive_from) = std::sync::mpsc::channel();
        let request = DatabaseMessage::GetServerInfo { respond_to };
        self.sender
            .send(request)
            .expect("Database receiver should not be dropped");
        receive_from
            .recv()
            .expect("Handler receiver should not be dropped")
    }
    pub fn get_server_name(&self) -> String {
        let (respond_to, receive_from) = std::sync::mpsc::channel();
        let request = DatabaseMessage::GetServerName { respond_to };
        self.sender
            .send(request)
            .expect("Database receiver should not be dropped");
        receive_from
            .recv()
            .expect("Handler receiver should not be dropped")
    }
    pub fn get_server_stream(&self, server: &str) -> Result<C, DatabaseError> {
        let (respond_to, receive_from) = std::sync::mpsc::channel();
        let server = server.to_string();
        let request = DatabaseMessage::GetServerStream { server, respond_to };
        self.sender
            .send(request)
            .expect("Database receiver should not be dropped");
        receive_from
            .recv()
            .expect("Handler receiver should not be dropped")
    }
    pub fn is_channel_operator(&self, channel: &str, nickname: &str) -> bool {
        let (respond_to, receive_from) = std::sync::mpsc::channel();
        let channel = channel.to_string();
        let nickname = nickname.to_string();
        let request = DatabaseMessage::IsChannelOperator {
            channel,
            nickname,
            respond_to,
        };
        self.sender
            .send(request)
            .expect("Database receiver should not be dropped");
        receive_from
            .recv()
            .expect("Handler receiver should not be dropped")
    }
    pub fn is_channel_speaker(&self, channel: &str, nickname: &str) -> bool {
        let (respond_to, receive_from) = std::sync::mpsc::channel();
        let channel = channel.to_string();
        let nickname = nickname.to_string();
        let request = DatabaseMessage::IsChannelSpeaker {
            channel,
            nickname,
            respond_to,
        };
        self.sender
            .send(request)
            .expect("Database receiver should not be dropped");
        receive_from
            .recv()
            .expect("Handler receiver should not be dropped")
    }
    pub fn is_client_in_channel(&self, channel: &str, nickname: &str) -> bool {
        let (respond_to, receive_from) = std::sync::mpsc::channel();
        let channel = channel.to_string();
        let nickname = nickname.to_string();
        let request = DatabaseMessage::IsClientInChannel {
            channel,
            nickname,
            respond_to,
        };
        self.sender
            .send(request)
            .expect("Database receiver should not be dropped");
        receive_from
            .recv()
            .expect("Handler receiver should not be dropped")
    }
    pub fn is_immediate_server(&self, server: &str) -> bool {
        let (respond_to, receive_from) = std::sync::mpsc::channel();
        let server = server.to_string();
        let request = DatabaseMessage::IsImmediateServer { server, respond_to };
        self.sender
            .send(request)
            .expect("Database receiver should not be dropped");
        receive_from
            .recv()
            .expect("Handler receiver should not be dropped")
    }
    pub fn is_local_client(&self, nickname: &str) -> bool {
        let (respond_to, receive_from) = std::sync::mpsc::channel();
        let nickname = nickname.to_string();
        let request = DatabaseMessage::IsLocalClient {
            nickname,
            respond_to,
        };
        self.sender
            .send(request)
            .expect("Database receiver should not be dropped");
        receive_from
            .recv()
            .expect("Handler receiver should not be dropped")
    }
    pub fn is_server_operator(&self, nickname: &str) -> bool {
        let (respond_to, receive_from) = std::sync::mpsc::channel();
        let nickname = nickname.to_string();
        let request = DatabaseMessage::IsServerOperator {
            nickname,
            respond_to,
        };
        self.sender
            .send(request)
            .expect("Database receiver should not be dropped");
        receive_from
            .recv()
            .expect("Handler receiver should not be dropped")
    }
    pub fn remove_channel_banmask(&self, channel: &str, mask: &str) {
        let channel = channel.to_string();
        let mask = mask.to_string();
        let request = DatabaseMessage::RemoveChannelBanmask { channel, mask };
        self.sender
            .send(request)
            .expect("Database receiver should not be dropped");
    }
    pub fn remove_channel_operator(&self, channel: &str, nickname: &str) {
        let channel = channel.to_string();
        let nickname = nickname.to_string();
        let request = DatabaseMessage::RemoveChannelOperator { channel, nickname };
        self.sender
            .send(request)
            .expect("Database receiver should not be dropped");
    }
    pub fn remove_channel_speaker(&self, channel: &str, nickname: &str) {
        let channel = channel.to_string();
        let nickname = nickname.to_string();
        let request = DatabaseMessage::RemoveChannelSpeaker { channel, nickname };
        self.sender
            .send(request)
            .expect("Database receiver should not be dropped");
    }
    pub fn remove_client_from_channel(&self, channel: &str, nickname: &str) {
        let channel = channel.to_string();
        let nickname = nickname.to_string();
        let request = DatabaseMessage::RemoveClientFromChannel { channel, nickname };
        self.sender
            .send(request)
            .expect("Database receiver should not be dropped");
    }
    pub fn remove_server(&self, servername: &str) {
        let servername = servername.to_string();
        let request = DatabaseMessage::RemoveServer { servername };
        self.sender
            .send(request)
            .expect("Database receiver should not be dropped");
    }
    pub fn set_away_message(&self, nickname: &str, message: Option<String>) {
        let nickname = nickname.to_string();
        let request = DatabaseMessage::SetAwayMessage { nickname, message };
        self.sender
            .send(request)
            .expect("Database receiver should not be dropped");
    }
    pub fn set_channel_flag(&self, channel: &str, flag: ChannelFlag) {
        let channel = channel.to_string();
        let request = DatabaseMessage::SetChannelFlag { channel, flag };
        self.sender
            .send(request)
            .expect("Database receiver should not be dropped");
    }
    pub fn set_channel_key(&self, channel: &str, key: Option<String>) {
        let channel = channel.to_string();
        let request = DatabaseMessage::SetChannelKey { channel, key };
        self.sender
            .send(request)
            .expect("Database receiver should not be dropped");
    }
    pub fn set_channel_limit(&self, channel: &str, limit: Option<usize>) {
        let channel = channel.to_string();
        let request = DatabaseMessage::SetChannelLimit { channel, limit };
        self.sender
            .send(request)
            .expect("Database receiver should not be dropped");
    }
    pub fn set_channel_topic(&self, channel: &str, topic: &str) {
        let channel = channel.to_string();
        let topic = topic.to_string();
        let request = DatabaseMessage::SetChannelTopic { channel, topic };
        self.sender
            .send(request)
            .expect("Database receiver should not be dropped");
    }
    pub fn set_server_operator(&self, nickname: &str) {
        let nickname = nickname.to_string();
        let request = DatabaseMessage::SetServerOperator { nickname };
        self.sender
            .send(request)
            .expect("Database receiver should not be dropped");
    }
    pub fn set_user_flag(&self, user: &str, flag: UserFlag) {
        let user = user.to_string();
        let request = DatabaseMessage::SetUserFlag { user, flag };
        self.sender
            .send(request)
            .expect("Database receiver should not be dropped");
    }
    pub fn unset_channel_flag(&self, channel: &str, flag: ChannelFlag) {
        let channel = channel.to_string();
        let request = DatabaseMessage::UnsetChannelFlag { channel, flag };
        self.sender
            .send(request)
            .expect("Database receiver should not be dropped");
    }
    pub fn unset_user_flag(&self, user: &str, flag: UserFlag) {
        let user = user.to_string();
        let request = DatabaseMessage::UnsetUserFlag { user, flag };
        self.sender
            .send(request)
            .expect("Database receiver should not be dropped");
    }
    pub fn update_nickname(&self, old_nickname: &str, new_nickname: &str) {
        let old_nickname = old_nickname.to_string();
        let new_nickname = new_nickname.to_string();
        let request = DatabaseMessage::UpdateNickname {
            old_nickname,
            new_nickname,
        };
        self.sender
            .send(request)
            .expect("Database receiver should not be dropped");
    }
    pub fn add_channel_invite(&self, channel: &str, client: &str) {
        own!(channel, client);
        let request = DatabaseMessage::AddChannelInvite { channel, client };
        self.sender
            .send(request)
            .expect("Database receiver should not be dropped");
    }
    pub fn channel_has_invite(&self, channel: &str, client: &str) -> bool {
        own!(channel, client);
        let (respond_to, receive_from) = std::sync::mpsc::channel();
        let request = DatabaseMessage::ChannelHasClientInvite {
            channel,
            client,
            respond_to,
        };
        self.sender
            .send(request)
            .expect("Database receiver should not be dropped");
        receive_from
            .recv()
            .expect("Handler receiver should not be dropped")
    }
}
impl<C: Connection> Clone for DatabaseHandle<C> {
    fn clone(&self) -> Self {
        Self {
            sender: self.sender.clone(),
        }
    }
}
