mod channel;
mod client;
mod utils;

#[cfg(test)]
mod tests;

use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};

pub use channel::Channel;
pub use client::{Client, ClientBuilder};

use super::client_trait::ClientTrait;
pub struct Database<T: ClientTrait> {
    clients: RwLock<HashMap<String, Client<T>>>,
    channels: RwLock<HashMap<String, Channel>>,
}

impl<T: ClientTrait> Database<T> {
    pub fn new() -> Self {
        Self {
            clients: RwLock::new(HashMap::new()),
            channels: RwLock::new(HashMap::new()),
        }
    }

    pub fn add_client(&self, client: Client<T>) {
        let mut clients_lock = self.clients.write().unwrap();

        println!(
            "Client registered: \npassword: {:?}\nnickname: {}\nrealname: {}",
            client.password(),
            client.nickname(),
            client.realname()
        );

        clients_lock.insert(client.nickname(), client);
    }

    pub fn set_server_operator(&self, nickname: &str) {
        println!("Setting {} as operator", nickname);

        let mut clients_lock = self.clients.write().unwrap();
        if let Some(client) = clients_lock.get_mut(&nickname.to_string()) {
            client.set_server_operator();
        }
    }

    pub fn _is_server_operator(&self, nickname: &str) -> bool {
        let mut clients_lock = self.clients.write().unwrap();
        if let Some(client) = clients_lock.get_mut(&nickname.to_string()) {
            return client._is_server_operator();
        }

        false
    }

    pub fn disconnect_client(&self, nickname: &str) {
        println!("Disconnecting {} ", nickname);

        if let Some(client) = self.clients.write().unwrap().get_mut(nickname) {
            client.disconnect();
        }
    }

    pub fn get_stream(&self, nickname: &str) -> Option<Arc<Mutex<T>>> {
        let clients_rlock = self.clients.read().unwrap();
        let client = clients_rlock.get(nickname)?;

        client.get_stream()
    }

    pub fn add_client_to_channel(&self, nickname: &str, channel_name: &str) {
        println!("Adding {} to channel {}", nickname, channel_name);

        let mut channels_lock = self.channels.write().unwrap();
        let channel: Option<&mut Channel> = channels_lock.get_mut(&channel_name.to_string());
        match channel {
            Some(channel) => channel.add_client(nickname.to_string()),
            None => {
                let new_channel = Channel::new(channel_name.to_string(), nickname.to_string());
                channels_lock.insert(channel_name.to_string(), new_channel);
            }
        }
    }

    pub fn remove_client_of_channel(&self, nickname: &str, channel_name: &str) {
        println!("Removing {} from channel {}", nickname, channel_name);

        let mut channels_lock = self.channels.write().unwrap();
        if let Some(channel) = channels_lock.get_mut(&channel_name.to_string()) {
            channel.remove_client(nickname.to_string());
            if channel.get_clients().is_empty() {
                channels_lock.remove(channel_name);
            }
        }
    }

    pub fn contains_client(&self, nickname: &str) -> bool {
        let clients_lock = self.clients.read().unwrap();

        clients_lock.contains_key(nickname)
    }

    pub fn contains_channel(&self, channel: &str) -> bool {
        let channels_lock = self.channels.read().unwrap();

        channels_lock.contains_key(channel)
    }

    pub fn is_client_in_channel(&self, nickname: &str, channel: &str) -> bool {
        self.get_clients(channel).contains(&nickname.to_string())
    }

    pub fn get_clients(&self, channel: &str) -> Vec<String> {
        let channels_lock = self.channels.read().unwrap();

        let client_info = channels_lock.get(channel);

        match client_info {
            Some(client_info) => client_info.get_clients(),
            None => vec![],
        }
    }

    pub fn get_all_clients(&self) -> Vec<String> {
        let clients_lock = self.clients.read().unwrap();

        let queried = clients_lock.values();

        queried.map(|client| client.nickname()).collect()
    }

    pub fn get_clients_for_query(&self, query: &str) -> Vec<String> {
        let clients_lock = self.clients.read().unwrap();

        let queried = clients_lock
            .values()
            .filter(|client| client_matches_query(client, query));

        queried.map(|client| client.nickname()).collect()
    }

    pub fn get_channels(&self) -> Vec<String> {
        let channels_lock = self.channels.read().unwrap();

        channels_lock.keys().cloned().collect()
    }

    pub fn get_channels_for_client(&self, nickname: &str) -> Vec<String> {
        let channels_lock = self.channels.read().unwrap();
        let mut channels = vec![];

        for (channel_name, channel) in channels_lock.iter() {
            let clients = channel.get_clients();
            if clients.contains(&nickname.to_string()) {
                channels.push(channel_name.clone());
            }
        }
        channels
    }
}

impl<T: ClientTrait> Default for Database<T> {
    fn default() -> Self {
        Self::new()
    }
}

fn client_matches_query<T: ClientTrait>(client: &Client<T>, query: &str) -> bool {
    if matches(&client.nickname(), query) {
        return true;
    }
    if matches(&client.realname(), query) {
        return true;
    }
    if matches(&client.username(), query) {
        return true;
    }
    if matches(&client.hostname(), query) {
        return true;
    }
    if matches(&client.servername(), query) {
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

// fn matches(stack: &str, needle: &str) -> bool {
//     let splits = needle.split('*');

//     let mut temp_stack = &stack.to_owned()[..];

//     let has_starting_glob = needle.starts_with('*');
//     let has_ending_glob = needle.ends_with('*');

//     for (i, split) in splits.enumerate() {
//         let index = temp_stack.find(split);
//         let index = match index {
//             Some(index) => index,
//             None => return false,
//         };
//         temp_stack = &temp_stack[index + split.len()..];

//         if i == 0 && !has_starting_glob && index != 0 {
//             return false;
//         }
//     }

//     if !has_ending_glob && !temp_stack.is_empty() {
//         return false;
//     }

//     true
// }
