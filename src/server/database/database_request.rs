use std::{io, sync::mpsc::Sender};

use crate::server::client_trait::ClientTrait;

use super::{Client, ClientInfo};

pub enum DatabaseRequest<T: ClientTrait> {
    AddClient {
        client: Client<T>,
    },
    GetStream {
        nickname: String,
        response: Sender<io::Result<T>>,
    },
    DisconnectClient {
        nickname: String,
    },
    SetServerOperator {
        nickname: String,
    },
    IsServerOperator {
        nickname: String,
        response: Sender<bool>,
    },
    // IsOnline {
    //     nickname: String,
    //     response: Sender<bool>,
    // },
    ContainsClient {
        nickname: String,
        response: Sender<bool>,
    },
    ContainsChannel {
        channel: String,
        response: Sender<bool>,
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
        response: Sender<bool>,
    },

    GetChannelsForClient {
        nickname: String,
        response: Sender<Vec<String>>,
    },
    GetClientsFromChannel {
        channel: String,
        response: Sender<Vec<String>>,
    },

    GetAllClients {
        response: Sender<Vec<ClientInfo>>,
    },
    GetAllChannels {
        response: Sender<Vec<String>>,
    },
    GetClientsForMask {
        mask: String,
        response: Sender<Vec<ClientInfo>>,
    },
    GetClientsForNickMask {
        nickmask: String,
        response: Sender<Vec<ClientInfo>>,
    },
}
