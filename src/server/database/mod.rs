mod channel;
mod client;
mod database_request;
mod interface_wrapper;
mod utils;

#[cfg(test)]
mod tests;

use std::collections::HashMap;
use std::sync::mpsc::{self, Receiver, Sender};
use std::{io, thread};

pub use channel::Channel;
pub use client::{Client, ClientBuilder, ClientInfo};
pub use database_request::DatabaseRequest;

use database_request::DatabaseRequest::{
    AddClient, AddClientToChannel, ContainsChannel, ContainsClient, DisconnectClient,
    GetAllChannels, GetAllClients, GetChannelsForClient, GetClientsForMask, GetClientsForNickMask,
    GetClientsFromChannel, GetStream, IsClientInChannel, IsServerOperator, RemoveClientFromChannel,
    SetServerOperator,
};

use super::client_trait::ClientTrait;
pub struct Database<T: ClientTrait> {
    receiver: Receiver<DatabaseRequest<T>>,
    clients: HashMap<String, Client<T>>,
    channels: HashMap<String, Channel>,
}

impl<T: ClientTrait> Database<T> {
    pub fn start() -> Sender<DatabaseRequest<T>> {
        let (sender, receiver) = mpsc::channel();

        let database = Self {
            receiver,
            clients: HashMap::new(),
            channels: HashMap::new(),
        };

        thread::spawn(|| database.run());

        sender
    }

    fn run(mut self) {
        while let Ok(request) = self.receiver.recv() {
            match request {
                AddClient { client } => self.add_client(client),
                GetStream { nickname, response } => self.get_stream_request(&nickname, response),
                DisconnectClient { nickname } => self.disconnect_client(&nickname),
                SetServerOperator { nickname } => self.set_server_operator(&nickname),
                IsServerOperator { nickname, response } => {
                    self.is_server_operator_request(&nickname, response)
                }
                //IsOnline { nickname, response } => self.is_online_request(&nickname, response),
                ContainsClient { nickname, response } => {
                    self.contains_client_request(&nickname, response)
                }
                ContainsChannel { channel, response } => {
                    self.contains_channel_request(&channel, response)
                }
                AddClientToChannel { nickname, channel } => {
                    self.add_client_to_channel(&nickname, &channel)
                }
                RemoveClientFromChannel { nickname, channel } => {
                    self.remove_client_from_channel(&nickname, &channel)
                }
                IsClientInChannel {
                    nickname,
                    channel,
                    response,
                } => self.is_client_in_channel_request(&nickname, &channel, response),
                GetChannelsForClient { nickname, response } => {
                    self.get_channels_for_client_request(&nickname, response)
                }
                GetClientsFromChannel { channel, response } => {
                    self.get_clients_for_channel_request(&channel, response)
                }
                GetAllClients { response } => self.get_all_clients_request(response),
                GetAllChannels { response } => self.get_channels_request(response),
                GetClientsForMask { mask, response } => {
                    self.get_clients_for_mask_request(&mask, response)
                }
                GetClientsForNickMask { nickmask, response } => {
                    self.get_clients_for_nickmask_request(&nickmask, response)
                }
            }
        }
    }

    fn add_client(&mut self, client: Client<T>) {
        let clientinfo = client.get_info();

        println!("Client registered: {clientinfo:?}",);

        self.clients.insert(clientinfo.nickname, client);
    }

    fn set_server_operator(&mut self, nickname: &str) {
        println!("Setting {} as operator", nickname);

        if let Some(client) = self.clients.get_mut(nickname) {
            client.set_server_operator();
        }
    }

    fn is_server_operator(&mut self, nickname: &str) -> bool {
        if let Some(client) = self.clients.get_mut(&nickname.to_string()) {
            return client.is_server_operator();
        }

        false
    }

    fn disconnect_client(&mut self, nickname: &str) {
        println!("Disconnecting {} ", nickname);

        if let Some(client) = self.clients.get_mut(nickname) {
            client.disconnect();
        }
    }

    // fn is_online(&self, nickname: &str) -> bool {
    //     if let Some(client) = self.clients.get(nickname) {
    //         return client.is_online();
    //     }

    //     false
    // }

    fn get_stream(&self, nickname: &str) -> io::Result<T> {
        if let Some(client) = self.clients.get(nickname) {
            return client.get_stream();
        }

        Err(io::Error::new(
            io::ErrorKind::Other,
            "Could not find client",
        ))
    }

    fn add_client_to_channel(&mut self, nickname: &str, channel_name: &str) {
        println!("Adding {} to channel {}", nickname, channel_name);

        let channel: Option<&mut Channel> = self.channels.get_mut(&channel_name.to_string());
        match channel {
            Some(channel) => channel.add_client(nickname.to_string()),
            None => {
                let new_channel = Channel::new(channel_name.to_string(), nickname.to_string());
                self.channels.insert(channel_name.to_string(), new_channel);
            }
        }
    }

    fn remove_client_from_channel(&mut self, nickname: &str, channel_name: &str) {
        println!("Removing {} from channel {}", nickname, channel_name);

        if let Some(channel) = self.channels.get_mut(&channel_name.to_string()) {
            channel.remove_client(nickname.to_string());
            if channel.get_clients().is_empty() {
                self.channels.remove(channel_name);
            }
        }
    }

    fn contains_client(&self, nickname: &str) -> bool {
        self.clients.contains_key(nickname)
    }

    fn contains_channel(&self, channel: &str) -> bool {
        self.channels.contains_key(channel)
    }

    fn is_client_in_channel(&self, nickname: &str, channel: &str) -> bool {
        self.get_clients_for_channel(channel)
            .contains(&nickname.to_string())
    }

    fn get_clients_for_channel(&self, channel: &str) -> Vec<String> {
        let channel_info = self.channels.get(channel);

        match channel_info {
            Some(channel_info) => channel_info.get_clients(),
            None => vec![],
        }
    }

    fn get_all_clients(&self) -> Vec<ClientInfo> {
        self.clients
            .values()
            .map(|client| client.get_info())
            .collect()
    }

    fn get_clients_for_mask(&self, mask: &str) -> Vec<ClientInfo> {
        self.filtered_clients(mask, client_matches_mask)
    }

    fn get_clients_for_nickmask(&self, mask: &str) -> Vec<ClientInfo> {
        self.filtered_clients(mask, client_matches_nickmask)
    }

    fn get_channels(&self) -> Vec<String> {
        self.channels.keys().cloned().collect()
    }

    fn get_channels_for_client(&self, nickname: &str) -> Vec<String> {
        let mut channels = vec![];

        for (channel_name, channel) in self.channels.iter() {
            let clients = channel.get_clients();
            if clients.contains(&nickname.to_string()) {
                channels.push(channel_name.clone());
            }
        }
        channels
    }

    fn filtered_clients(&self, mask: &str, f: fn(&ClientInfo, &str) -> bool) -> Vec<ClientInfo> {
        self.clients
            .values()
            .map(|client| client.get_info())
            .filter(|client| f(client, mask))
            .collect()
    }
}

fn client_matches_nickmask(client: &ClientInfo, mask: &str) -> bool {
    matches(&client.nickname, mask)
}

fn client_matches_mask(client: &ClientInfo, query: &str) -> bool {
    if matches(&client.nickname, query) {
        return true;
    }
    if matches(&client.realname, query) {
        return true;
    }
    if matches(&client.username, query) {
        return true;
    }
    if matches(&client.hostname, query) {
        return true;
    }
    if matches(&client.servername, query) {
        return true;
    }

    false
}

fn matches(base: &str, pattern: &str) -> bool {
    if pattern.is_empty() {
        return base.is_empty();
    }

    let base = base.as_bytes();
    let pattern = pattern.as_bytes();

    let mut base_index = 0;
    let mut pattern_index = 0;
    let mut glob_base_index = -1;
    let mut glob_pattern_index = -1;

    while base_index < base.len() {
        if pattern_index < pattern.len() {
            if base[base_index] == pattern[pattern_index] || pattern[pattern_index] == b'?' {
                base_index += 1;
                pattern_index += 1;
                continue;
            }

            if pattern[pattern_index] == b'*' {
                glob_base_index = base_index as isize;
                glob_pattern_index = pattern_index as isize;
                pattern_index += 1;
                continue;
            }
        }

        if glob_pattern_index != -1 {
            base_index = (glob_base_index + 1) as usize;
            pattern_index = (glob_pattern_index + 1) as usize;
            glob_base_index += 1;
            continue;
        }

        return false;
    }

    while pattern_index < pattern.len() && pattern[pattern_index] == b'*' {
        pattern_index += 1;
    }

    pattern_index == pattern.len()
}
