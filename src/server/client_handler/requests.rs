use std::{io, sync::mpsc};

use crate::server::{
    client_trait::ClientTrait,
    database::{Client, ClientInfo, DatabaseMessage},
};

use super::ClientHandler;

impl<T: ClientTrait> ClientHandler<T> {
    pub fn add_client(&self, client: Client<T>) {
        let request = DatabaseMessage::AddClient { client };
        self.database.send(request).unwrap();
    }

    pub fn set_server_operator(&self, nickname: &str) {
        let request = DatabaseMessage::SetServerOperator {
            nickname: nickname.to_string(),
        };
        self.database.send(request).unwrap();
    }

    pub fn is_server_operator(&self, nickname: &str) -> bool {
        let (sender, receiver) = mpsc::channel();
        let request = DatabaseMessage::IsServerOperator {
            nickname: nickname.to_string(),
            respond_to: sender,
        };
        self.database.send(request).unwrap();
        receiver.recv().unwrap()
    }

    pub fn disconnect_client(&self, nickname: &str) {
        println!("Disconnecting {} ", nickname);

        let request = DatabaseMessage::DisconnectClient {
            nickname: nickname.to_string(),
        };
        self.database.send(request).unwrap();
    }

    // pub fn is_online(&self, nickname: &str) -> bool {
    //     let (sender, receiver) = mpsc::channel();
    //     let request = DatabaseRequest::IsOnline {
    //         nickname: nickname.to_string(),
    //         response: sender,
    //     };
    //     self.database.send(request).unwrap();
    //     receiver.recv().unwrap()
    // }

    pub fn get_stream(&self, nickname: &str) -> io::Result<T> {
        let (sender, receiver) = mpsc::channel();
        let request = DatabaseMessage::GetStream {
            nickname: nickname.to_string(),
            respond_to: sender,
        };
        self.database.send(request).unwrap();
        receiver.recv().unwrap()
    }

    pub fn add_client_to_channel(&self, nickname: &str, channel_name: &str) {
        let request = DatabaseMessage::AddClientToChannel {
            nickname: nickname.to_string(),
            channel: channel_name.to_string(),
        };

        self.database.send(request).unwrap();
    }

    pub fn remove_client_from_channel(&self, nickname: &str, channel_name: &str) {
        let request = DatabaseMessage::RemoveClientFromChannel {
            nickname: nickname.to_string(),
            channel: channel_name.to_string(),
        };

        self.database.send(request).unwrap();
    }

    pub fn contains_client(&self, nickname: &str) -> bool {
        let (sender, receiver) = mpsc::channel();
        let request = DatabaseMessage::ContainsClient {
            nickname: nickname.to_string(),
            respond_to: sender,
        };
        self.database.send(request).unwrap();

        receiver.recv().unwrap()
    }

    pub fn contains_channel(&self, channel: &str) -> bool {
        let (sender, receiver) = mpsc::channel();
        let request = DatabaseMessage::ContainsChannel {
            channel: channel.to_string(),
            respond_to: sender,
        };
        self.database.send(request).unwrap();

        receiver.recv().unwrap()
    }

    pub fn is_client_in_channel(&self, nickname: &str, channel: &str) -> bool {
        let (sender, receiver) = mpsc::channel();
        let request = DatabaseMessage::IsClientInChannel {
            nickname: nickname.to_string(),
            channel: channel.to_string(),
            respond_to: sender,
        };
        self.database.send(request).unwrap();

        receiver.recv().unwrap()
    }

    pub fn get_clients_for_channel(&self, channel: &str) -> Vec<String> {
        let (sender, receiver) = mpsc::channel();
        let request = DatabaseMessage::GetClientsFromChannel {
            channel: channel.to_string(),
            respond_to: sender,
        };
        self.database.send(request).unwrap();

        receiver.recv().unwrap()
    }

    pub fn get_all_clients(&self) -> Vec<ClientInfo> {
        let (sender, receiver) = mpsc::channel();
        let request = DatabaseMessage::GetAllClients { respond_to: sender };
        self.database.send(request).unwrap();
        receiver.recv().unwrap()
    }

    pub fn get_clients_for_mask(&self, mask: &str) -> Vec<ClientInfo> {
        let (sender, receiver) = mpsc::channel();
        let request = DatabaseMessage::GetClientsForMask {
            mask: mask.to_string(),
            respond_to: sender,
        };
        self.database.send(request).unwrap();

        receiver.recv().unwrap()
    }

    pub fn get_clients_for_nickmask(&self, mask: &str) -> Vec<ClientInfo> {
        let (sender, receiver) = mpsc::channel();
        let request = DatabaseMessage::GetClientsForNickMask {
            nickmask: mask.to_string(),
            respond_to: sender,
        };
        self.database.send(request).unwrap();

        receiver.recv().unwrap()
    }

    pub fn get_channels(&self) -> Vec<String> {
        let (sender, receiver) = mpsc::channel();
        let request = DatabaseMessage::GetAllChannels { respond_to: sender };
        self.database.send(request).unwrap();

        receiver.recv().unwrap()
    }

    pub fn get_channels_for_client(&self, nickname: &str) -> Vec<String> {
        let (sender, receiver) = mpsc::channel();
        let request = DatabaseMessage::GetChannelsForClient {
            nickname: nickname.to_string(),
            respond_to: sender,
        };
        self.database.send(request).unwrap();

        receiver.recv().unwrap()
    }
}
