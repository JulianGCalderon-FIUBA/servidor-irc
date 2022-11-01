mod channel;
mod client;
mod utils;

#[cfg(test)]
mod tests;

use std::collections::HashMap;
use std::sync::mpsc::{Sender, Receiver, self};
use std::sync::{Arc, Mutex, RwLock};
use std::thread;

pub use channel::Channel;
pub use client::{Client, ClientBuilder, ClientInfo};

use super::client_trait::ClientTrait;
pub struct Database<T: ClientTrait> {
    receiver: Receiver<DatabaseEvent>
    clients: HashMap<String, Client<T>>,
    channels: HashMap<String, Channel>,
}

impl<T: ClientTrait> Database<T> {
    pub fn start() -> Sender<DatabaseEvent> {
        let (sender, receiver) = mpsc::channel();

        let database = Self {
            receiver,
            clients: HashMap::new(),
            channels: HashMap::new(),
        };

        thread::spawn( || database.run());

        sender
    }

    fn run(self) {
        while let Ok(event) = self.receiver.recv() {
            // HANDLE EVENT
        }
    }

    fn add_client(&self, client: Client<T>) {
        let clientinfo = client.get_info();

        println!("Client registered: {clientinfo:?}",);

        self.clients.insert(clientinfo.nickname, client);

    }

    fn set_server_operator(&self, nickname: &str) {
        println!("Setting {} as operator", nickname);

        if let Some(client) = self.clients.get_mut(&nickname.to_string()) {
            client.set_server_operator();
        }
    }

    fn is_server_operator(&self, nickname: &str) -> bool {
        if let Some(client) = self.clients.get_mut(&nickname.to_string()) {
            return client.is_server_operator();
        }

        false
    }

    fn disconnect_client(&self, nickname: &str) {
        println!("Disconnecting {} ", nickname);

        if let Some(client) = self.clients.get_mut(nickname) {
            client.disconnect();
        }
    }

    fn get_stream(&self, nickname: &str) -> Option<Arc<Mutex<T>>> {
        let client = self.clients.get(nickname)?;

        client.get_stream()
    }

    fn add_client_to_channel(&self, nickname: &str, channel_name: &str) {
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

    fn remove_client_of_channel(&self, nickname: &str, channel_name: &str) {
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
        self.get_clients(channel).contains(&nickname.to_string())
    }

    fn get_clients(&self, channel: &str) -> Vec<String> {
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
