use std::{
    io,
    sync::mpsc::{self, Sender},
};

use crate::server::client_trait::Connection;

use super::database_message::DatabaseMessage;
use super::{Client, ClientInfo};
/// A DatabaseHandle handles and makes request to the main Database. Works as an intermediary so external structures cannot acces the Database directly.
pub struct DatabaseHandle<C: Connection> {
    sender: Sender<DatabaseMessage<C>>,
}

impl<C: Connection> DatabaseHandle<C> {
    /// Creates new DatabaseHandle
    pub fn new(sender: Sender<DatabaseMessage<C>>) -> Self {
        Self { sender }
    }

    /// Sends AddClient request.
    pub fn add_client(&self, client: Client<C>) {
        let request = DatabaseMessage::AddClient { client };
        self.sender.send(request).unwrap();
    }

    /// Sends SetServerOperator request.
    pub fn set_server_operator(&self, nickname: &str) {
        let request = DatabaseMessage::SetServerOperator {
            nickname: nickname.to_string(),
        };
        self.sender.send(request).unwrap();
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

    /// Sends DisconnectClient request.
    pub fn disconnect_client(&self, nickname: &str) {
        let request = DatabaseMessage::DisconnectClient {
            nickname: nickname.to_string(),
        };
        self.sender.send(request).unwrap();
    }

    /// Sends GetStream request and returns answer.
    pub fn get_stream(&self, nickname: &str) -> io::Result<C> {
        let (sender, receiver) = mpsc::channel();
        let request = DatabaseMessage::GetStream {
            nickname: nickname.to_string(),
            respond_to: sender,
        };
        self.sender.send(request).unwrap();
        receiver.recv().unwrap()
    }

    /// Sends AddClientToChannel request.
    pub fn add_client_to_channel(&self, nickname: &str, channel_name: &str) {
        let request = DatabaseMessage::AddClientToChannel {
            nickname: nickname.to_string(),
            channel: channel_name.to_string(),
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

    /// Sends GetClientsForChannel request and returns answer.
    pub fn get_clients_for_channel(&self, channel: &str) -> Vec<String> {
        let (sender, receiver) = mpsc::channel();
        let request = DatabaseMessage::GetClientsFromChannel {
            channel: channel.to_string(),
            respond_to: sender,
        };
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

    /// Sends GetClientsForMask request and returns answer.
    pub fn get_clients_for_mask(&self, mask: &str) -> Vec<ClientInfo> {
        let (sender, receiver) = mpsc::channel();
        let request = DatabaseMessage::GetClientsForMask {
            mask: mask.to_string(),
            respond_to: sender,
        };
        self.sender.send(request).unwrap();
        receiver.recv().unwrap()
    }

    /// Sends GetClientsForNickMask request and returns answer.
    pub fn get_clients_for_nickmask(&self, mask: &str) -> Vec<ClientInfo> {
        let (sender, receiver) = mpsc::channel();
        let request = DatabaseMessage::GetClientsForNickMask {
            nickmask: mask.to_string(),
            respond_to: sender,
        };
        self.sender.send(request).unwrap();
        receiver.recv().unwrap()
    }

    /// Sends GetAllChannels request and returns answer.
    pub fn get_all_channels(&self) -> Vec<String> {
        let (sender, receiver) = mpsc::channel();
        let request = DatabaseMessage::GetAllChannels { respond_to: sender };
        self.sender.send(request).unwrap();
        receiver.recv().unwrap()
    }

    /// Sends GetChannelsForClient request and returns answer.
    pub fn get_channels_for_client(&self, nickname: &str) -> Vec<String> {
        let (sender, receiver) = mpsc::channel();
        let request = DatabaseMessage::GetChannelsForClient {
            nickname: nickname.to_string(),
            respond_to: sender,
        };
        self.sender.send(request).unwrap();
        receiver.recv().unwrap()
    }

    /// Sends UpdateNickname request and returns answer.
    pub fn update_nickname(&self, old_nickname: &str, new_nickname: &str) {
        let request = DatabaseMessage::UpdateNickname {
            old_nickname: old_nickname.to_string(),
            new_nickname: new_nickname.to_string(),
        };
        self.sender.send(request).unwrap();
    }

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
}

impl<C: Connection> Clone for DatabaseHandle<C> {
    fn clone(&self) -> Self {
        Self {
            sender: self.sender.clone(),
        }
    }
}
