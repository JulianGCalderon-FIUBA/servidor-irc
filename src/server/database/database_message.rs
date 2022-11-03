use std::io;
use std::sync::mpsc::Sender;

use super::{Client, ClientInfo};
use crate::server::client_trait::ClientTrait;

pub enum DatabaseMessage<T: ClientTrait> {
    AddClient {
        client: Client<T>,
    },
    GetStream {
        nickname: String,
        respond_to: Sender<io::Result<T>>,
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
}
