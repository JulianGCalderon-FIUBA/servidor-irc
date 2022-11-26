use std::io;
use std::sync::mpsc::Sender;

use crate::server::consts::modes::ChannelFlag;
use crate::server::data_structures::*;

use crate::server::connection::Connection;

/// Possible messages or requests a Database can receive.
pub enum DatabaseMessage<C: Connection> {
    AddChannelBanMask {
        channel: String,
        mask: String,
    },
    AddChanop {
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
    AddSpeaker {
        channel: String,
        nickname: String,
    },
    AreCredentialsValid {
        username: String,
        password: String,
        respond_to: Sender<bool>,
    },
    ChannelHasMode {
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
        respond_to: Sender<Option<String>>,
    },
    GetChannelBanMask {
        channel: String,
        respond_to: Sender<Vec<String>>,
    },
    GetChannelConfig {
        channel: String,
        respond_to: Sender<Option<ChannelConfiguration>>,
    },
    GetChannelKey {
        channel: String,
        respond_to: Sender<Option<String>>,
    },
    GetChannelTopic {
        channel: String,
        respond_to: Sender<Option<String>>,
    },
    GetChannelsForClient {
        nickname: String,
        respond_to: Sender<Vec<String>>,
    },
    GetChannelClients {
        channel: String,
        respond_to: Sender<Vec<String>>,
    },
    GetLimit {
        channel: String,
        respond_to: Sender<Option<usize>>,
    },

    GetLocalStream {
        nickname: String,
        respond_to: Sender<Option<io::Result<C>>>,
    },
    GetServerInfo {
        respond_to: Sender<String>,
    },
    GetServerName {
        respond_to: Sender<String>,
    },
    GetServerStream {
        server: String,
        respond_to: Sender<Option<io::Result<C>>>,
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
    IsServerOperator {
        nickname: String,
        respond_to: Sender<bool>,
    },
    RemoveChannelBanMask {
        channel: String,
        mask: String,
    },
    RemoveChanop {
        channel: String,
        nickname: String,
    },
    RemoveClientFromChannel {
        nickname: String,
        channel: String,
    },
    RemoveSpeaker {
        channel: String,
        nickname: String,
    },
    SetAwayMessage {
        message: Option<String>,
        nickname: String,
    },
    SetChannelKey {
        channel: String,
        key: Option<String>,
    },
    SetChannelMode {
        channel: String,
        flag: ChannelFlag,
    },
    SetChannelTopic {
        channel: String,
        topic: String,
    },
    SetLimit {
        channel: String,
        limit: Option<usize>,
    },
    SetServerOperator {
        nickname: String,
    },
    UnsetChannelMode {
        channel: String,
        flag: ChannelFlag,
    },
    UpdateNickname {
        old_nickname: String,
        new_nickname: String,
    },
    IsLocalClient {
        nickname: String,
        respond_to: Sender<bool>,
    },
    GetImmediateServer {
        client: String,
        respond_to: Sender<Option<String>>,
    },
    GetClientInfo {
        client: String,
        respond_to: Sender<Option<ClientInfo>>,
    },
}
