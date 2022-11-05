use std::{
    io,
    sync::mpsc::{self, Sender},
};

use crate::server::client_trait::ClientTrait;

use super::database_message::DatabaseMessage;
use super::{Client, ClientInfo};

pub struct DatabaseHandle<T: ClientTrait> {
    sender: Sender<DatabaseMessage<T>>,
}

impl<T: ClientTrait> DatabaseHandle<T> {
    pub fn new(sender: Sender<DatabaseMessage<T>>) -> Self {
        Self { sender }
    }

    pub fn add_client(&self, client: Client<T>) {
        let request = DatabaseMessage::AddClient { client };
        self.sender.send(request).unwrap();
    }

    pub fn set_server_operator(&self, nickname: &str) {
        let request = DatabaseMessage::SetServerOperator {
            nickname: nickname.to_string(),
        };
        self.sender.send(request).unwrap();
    }

    pub fn is_server_operator(&self, nickname: &str) -> bool {
        let (sender, receiver) = mpsc::channel();
        let request = DatabaseMessage::IsServerOperator {
            nickname: nickname.to_string(),
            respond_to: sender,
        };
        self.sender.send(request).unwrap();
        receiver.recv().unwrap()
    }

    pub fn disconnect_client(&self, nickname: &str) {
        let request = DatabaseMessage::DisconnectClient {
            nickname: nickname.to_string(),
        };
        self.sender.send(request).unwrap();
    }

    pub fn get_stream(&self, nickname: &str) -> io::Result<T> {
        let (sender, receiver) = mpsc::channel();
        let request = DatabaseMessage::GetStream {
            nickname: nickname.to_string(),
            respond_to: sender,
        };
        self.sender.send(request).unwrap();
        receiver.recv().unwrap()
    }

    pub fn add_client_to_channel(&self, nickname: &str, channel_name: &str) {
        let request = DatabaseMessage::AddClientToChannel {
            nickname: nickname.to_string(),
            channel: channel_name.to_string(),
        };
        self.sender.send(request).unwrap();
    }

    pub fn remove_client_from_channel(&self, nickname: &str, channel_name: &str) {
        let request = DatabaseMessage::RemoveClientFromChannel {
            nickname: nickname.to_string(),
            channel: channel_name.to_string(),
        };
        self.sender.send(request).unwrap();
    }

    pub fn contains_client(&self, nickname: &str) -> bool {
        let (sender, receiver) = mpsc::channel();
        let request = DatabaseMessage::ContainsClient {
            nickname: nickname.to_string(),
            respond_to: sender,
        };
        self.sender.send(request).unwrap();
        receiver.recv().unwrap()
    }

    pub fn contains_channel(&self, channel: &str) -> bool {
        let (sender, receiver) = mpsc::channel();
        let request = DatabaseMessage::ContainsChannel {
            channel: channel.to_string(),
            respond_to: sender,
        };
        self.sender.send(request).unwrap();
        receiver.recv().unwrap()
    }

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

    pub fn get_clients_for_channel(&self, channel: &str) -> Vec<String> {
        let (sender, receiver) = mpsc::channel();
        let request = DatabaseMessage::GetClientsFromChannel {
            channel: channel.to_string(),
            respond_to: sender,
        };
        self.sender.send(request).unwrap();
        receiver.recv().unwrap()
    }

    pub fn get_all_clients(&self) -> Vec<ClientInfo> {
        let (sender, receiver) = mpsc::channel();
        let request = DatabaseMessage::GetAllClients { respond_to: sender };
        self.sender.send(request).unwrap();
        receiver.recv().unwrap()
    }

    pub fn get_clients_for_mask(&self, mask: &str) -> Vec<ClientInfo> {
        let (sender, receiver) = mpsc::channel();
        let request = DatabaseMessage::GetClientsForMask {
            mask: mask.to_string(),
            respond_to: sender,
        };
        self.sender.send(request).unwrap();
        receiver.recv().unwrap()
    }

    pub fn get_clients_for_nickmask(&self, mask: &str) -> Vec<ClientInfo> {
        let (sender, receiver) = mpsc::channel();
        let request = DatabaseMessage::GetClientsForNickMask {
            nickmask: mask.to_string(),
            respond_to: sender,
        };
        self.sender.send(request).unwrap();
        receiver.recv().unwrap()
    }

    pub fn get_channels(&self) -> Vec<String> {
        let (sender, receiver) = mpsc::channel();
        let request = DatabaseMessage::GetAllChannels { respond_to: sender };
        self.sender.send(request).unwrap();
        receiver.recv().unwrap()
    }

    pub fn get_channels_for_client(&self, nickname: &str) -> Vec<String> {
        let (sender, receiver) = mpsc::channel();
        let request = DatabaseMessage::GetChannelsForClient {
            nickname: nickname.to_string(),
            respond_to: sender,
        };
        self.sender.send(request).unwrap();
        receiver.recv().unwrap()
    }

    pub fn update_nickname(&self, old_nickname: &str, new_nickname: &str) {
        let request = DatabaseMessage::UpdateNickname {
            old_nickname: old_nickname.to_string(),
            new_nickname: new_nickname.to_string(),
        };
        self.sender.send(request).unwrap();
    }
}

impl<T: ClientTrait> Clone for DatabaseHandle<T> {
    fn clone(&self) -> Self {
        Self {
            sender: self.sender.clone(),
        }
    }
}
