use std::sync::mpsc::Sender;

use crate::server::consts::channel_flag::ChannelFlag;
use crate::server::consts::user_flag::UserFlag;
use crate::server::data_structures::*;

use crate::server::connection::Connection;

use super::database_error::DatabaseError;

/// Requests a Database can receive and must be able to answer.
pub enum DatabaseMessage<C: Connection> {
    AddChannelBanmask {
        channel: String,
        mask: String,
    },
    AddChannelInvite {
        channel: String,
        client: String,
    },
    AddChannelOperator {
        channel: String,
        nickname: String,
    },
    AddChannelSpeaker {
        channel: String,
        nickname: String,
    },
    AddClientToChannel {
        nickname: String,
        channel: String,
    },
    AddDistantServer {
        server: ServerInfo,
    },
    AddExternalClient {
        client: ExternalClient,
    },
    AddImmediateServer {
        server: ImmediateServer<C>,
    },
    AddLocalClient {
        client: LocalClient<C>,
    },
    AreCredentialsValid {
        username: String,
        password: String,
        respond_to: Sender<bool>,
    },
    ChannelHasFlag {
        channel: String,
        flag: ChannelFlag,
        respond_to: Sender<bool>,
    },
    ContainsChannel {
        channel: String,
        respond_to: Sender<bool>,
    },
    ContainsClient {
        nickname: String,
        respond_to: Sender<bool>,
    },
    ContainsServer {
        servername: String,
        respond_to: Sender<bool>,
    },
    DisconnectClient {
        nickname: String,
    },
    GetAllChannels {
        respond_to: Sender<Vec<String>>,
    },
    GetAllClients {
        respond_to: Sender<Vec<ClientInfo>>,
    },
    GetAllServers {
        respond_to: Sender<Vec<String>>,
    },
    GetAwayMessage {
        nickname: String,
        respond_to: Sender<Result<Option<String>, DatabaseError>>,
    },
    GetChannelBanmask {
        channel: String,
        respond_to: Sender<Result<Vec<String>, DatabaseError>>,
    },
    GetChannelClients {
        channel: String,
        respond_to: Sender<Result<Vec<String>, DatabaseError>>,
    },
    GetChannelConfig {
        channel: String,
        respond_to: Sender<Result<ChannelConfiguration, DatabaseError>>,
    },
    GetChannelKey {
        channel: String,
        respond_to: Sender<Result<Option<String>, DatabaseError>>,
    },
    GetChannelLimit {
        channel: String,
        respond_to: Sender<Result<Option<usize>, DatabaseError>>,
    },
    GetChannelTopic {
        channel: String,
        respond_to: Sender<Result<Option<String>, DatabaseError>>,
    },
    GetChannelsForClient {
        nickname: String,
        respond_to: Sender<Result<Vec<String>, DatabaseError>>,
    },
    GetClientInfo {
        client: String,
        respond_to: Sender<Result<ClientInfo, DatabaseError>>,
    },
    GetImmediateServer {
        client: String,
        respond_to: Sender<Result<String, DatabaseError>>,
    },
    GetLocalStream {
        nickname: String,
        respond_to: Sender<Result<C, DatabaseError>>,
    },
    GetServerInfo {
        respond_to: Sender<String>,
    },
    GetServerName {
        respond_to: Sender<String>,
    },
    GetServerStream {
        server: String,
        respond_to: Sender<Result<C, DatabaseError>>,
    },
    IsChannelOperator {
        channel: String,
        nickname: String,
        respond_to: Sender<bool>,
    },
    IsChannelSpeaker {
        channel: String,
        nickname: String,
        respond_to: Sender<bool>,
    },
    IsClientInChannel {
        nickname: String,
        channel: String,
        respond_to: Sender<bool>,
    },
    IsImmediateServer {
        server: String,
        respond_to: Sender<bool>,
    },
    IsLocalClient {
        nickname: String,
        respond_to: Sender<bool>,
    },
    IsServerOperator {
        nickname: String,
        respond_to: Sender<bool>,
    },
    RemoveChannelBanmask {
        channel: String,
        mask: String,
    },
    RemoveChannelOperator {
        channel: String,
        nickname: String,
    },
    RemoveChannelSpeaker {
        channel: String,
        nickname: String,
    },
    RemoveClientFromChannel {
        nickname: String,
        channel: String,
    },
    RemoveServer {
        servername: String,
    },
    SetAwayMessage {
        message: Option<String>,
        nickname: String,
    },
    SetChannelFlag {
        channel: String,
        flag: ChannelFlag,
    },
    SetChannelKey {
        channel: String,
        key: Option<String>,
    },
    SetChannelLimit {
        channel: String,
        limit: Option<usize>,
    },
    SetChannelTopic {
        channel: String,
        topic: String,
    },
    SetServerOperator {
        nickname: String,
    },
    SetUserFlag {
        user: String,
        flag: UserFlag,
    },
    UnsetChannelFlag {
        channel: String,
        flag: ChannelFlag,
    },
    UnsetUserFlag {
        user: String,
        flag: UserFlag,
    },
    UpdateNickname {
        old_nickname: String,
        new_nickname: String,
    },
    ChannelHasClientInvite {
        channel: String,
        client: String,
        respond_to: Sender<bool>,
    },
}

impl<C: Connection> DatabaseMessage<C> {}
