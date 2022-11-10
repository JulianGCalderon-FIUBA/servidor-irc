use std::io;
use std::sync::mpsc::Sender;

use super::{Client, ClientInfo};
use crate::server::client_trait::Connection;

/// Possible messages or requests a Database can receive.
pub enum DatabaseMessage<C: Connection> {
    AddClient {
        client: Client<C>,
    },
    GetStream {
        nickname: String,
        respond_to: Sender<io::Result<C>>,
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
    _IsClientInChannel {
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
}
