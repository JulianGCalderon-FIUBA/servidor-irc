use std::io;
use std::sync::mpsc::Sender;

use crate::server::data_structures::*;

use crate::server::connection::Connection;

/// Possible messages or requests a Database can receive.
pub enum DatabaseMessage<C: Connection> {
    AddClient {
        client: Client<C>,
    },
    GetStream {
        nickname: String,
        respond_to: Sender<Option<io::Result<C>>>,
    },
    DisconnectClient {
        nickname: String,
    },
    SetServerOperator {
        nickname: String,
    },
    IsServerOperator {
        nickname: String,
        respond_to: Sender<bool>,
    },
    ContainsClient {
        nickname: String,
        respond_to: Sender<bool>,
    },
    ContainsChannel {
        channel: String,
        respond_to: Sender<bool>,
    },
    AddClientToChannel {
        nickname: String,
        channel: String,
    },
    RemoveClientFromChannel {
        nickname: String,
        channel: String,
    },
    IsClientInChannel {
        nickname: String,
        channel: String,
        respond_to: Sender<bool>,
    },
    GetChannelsForClient {
        nickname: String,
        respond_to: Sender<Vec<String>>,
    },
    GetClientsFromChannel {
        channel: String,
        respond_to: Sender<Vec<String>>,
    },
    GetAllClients {
        respond_to: Sender<Vec<ClientInfo>>,
    },
    GetAllChannels {
        respond_to: Sender<Vec<String>>,
    },
    GetClientsForMask {
        mask: String,
        respond_to: Sender<Vec<ClientInfo>>,
    },
    GetClientsForNickMask {
        nickmask: String,
        respond_to: Sender<Vec<ClientInfo>>,
    },
    UpdateNickname {
        old_nickname: String,
        new_nickname: String,
    },
    AreCredentialsValid {
        username: String,
        password: String,
        respond_to: Sender<bool>,
    },
    SetAwayMessage {
        message: Option<String>,
        nickname: String,
    },
    GetAwayMessage {
        nickname: String,
        respond_to: Sender<Option<String>>,
    },
    SetChannelTopic {
        channel: String,
        topic: String,
    },
    GetChannelTopic {
        channel: String,
        respond_to: Sender<Option<String>>,
    },
    IsChannelOperator {
        channel: String,
        nickname: String,
        respond_to: Sender<bool>,
    },
    SetChannelKey {
        channel: String,
        key: Option<String>,
    },
    GetChannelKey {
        channel: String,
        respond_to: Sender<Option<String>>,
    },
    SetChannelMode {
        channel: String,
        mode: char,
    },
    UnsetChannelMode {
        channel: String,
        mode: char,
    },
    ChannelHasMode {
        channel: String,
        mode: char,
        respond_to: Sender<bool>,
    },
    SetLimit {
        channel: String,
        limit: Option<usize>,
    },
    GetLimit {
        channel: String,
        respond_to: Sender<Option<usize>>,
    },
    AddChanop {
        channel: String,
        nickname: String,
    },
    RemoveChanop {
        channel: String,
        nickname: String,
    },
    AddSpeaker {
        channel: String,
        nickname: String,
    },
    RemoveSpeaker {
        channel: String,
        nickname: String,
    },
    IsChannelSpeaker {
        channel: String,
        nickname: String,
        respond_to: Sender<bool>,
    },
    AddChannelBanMask {
        channel: String,
        mask: String,
    },
    GetChannelBanMask {
        channel: String,
        respond_to: Sender<Vec<String>>,
    },
    RemoveChannelBanMask {
        channel: String,
        mask: String,
    },
    ClientMatchesBanmask {
        nickname: String,
        mask: String,
        respond_to: Sender<bool>,
    },
    // GetAllChannelModes {
    //     channel: String,
    //     respond_to: Sender<Vec<char>>,
    // },
    ContainsServer {
        servername: String,
        respond_to: Sender<bool>,
    },
    AddServer {
        server: ExternalServer<C>,
    },
    AddExternalClient {
        server: String,
        client: ExternalClient,
    },
    GetServerName {
        respond_to: Sender<String>,
    },
    GetServerInfo {
        respond_to: Sender<String>,
    },
}
