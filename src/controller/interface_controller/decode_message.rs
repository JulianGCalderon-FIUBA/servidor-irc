use std::collections::HashMap;

use crate::message::Message;

use super::{utils::is_channel, InterfaceController};

impl InterfaceController {
    pub fn decode_invite_message(&mut self, message: Message) -> (String, String) {
        let channel = message.get_parameters()[1].clone();
        let nickname = message.get_prefix().clone().unwrap();

        (channel, nickname)
    }

    pub fn decode_kick_message(&mut self, message: Message) -> (String, String) {
        let channel = message.get_parameters()[0].clone();
        let kicked = message.get_parameters()[1].clone();

        (channel, kicked)
    }

    pub fn decode_registration(&mut self, message: Message) -> (String, String, String, String) {
        let trailing_text = message.get_trailing().clone().unwrap();
        let trailing_strings = trailing_text.split(' ').collect::<Vec<&str>>();
        println!("{trailing_strings:?}");
        let mut username = trailing_strings[5].to_string();
        username.remove(0);
        let realname = message.get_parameters()[0].clone();
        let servername = trailing_strings[2].to_string();
        let nickname = trailing_strings[4].to_string();

        (nickname, realname, servername, username)
    }

    pub fn decode_priv_message(&mut self, message: Message) -> (Option<String>, String, String) {
        let message_text = message.get_trailing().clone().unwrap();
        let sender_nickname = message.get_prefix().clone().unwrap();
        let channel = if is_channel(message.get_parameters()[0].clone()) {
            Some(message.get_parameters()[0].clone())
        } else {
            None
        };

        (channel, message_text, sender_nickname)
    }

    pub fn decode_list_line_message(&mut self, message: Message) -> String {
        let channel = message.get_parameters()[0].clone();

        channel
    }

    pub fn decode_names_line_message(&mut self, message: Message) -> (String, Vec<String>) {
        let channels = message.get_parameters()[0].clone();
        let trailing: String = message.get_trailing().clone().unwrap();
        let clients: Vec<String> = trailing.split(' ').map(|s| s.to_string()).collect();

        (channels, clients)
    }

    pub fn process_list_end_message(&mut self) -> Vec<String> {
        let channels: Vec<String> = self.accumulated_channels_from_list.clone();
        self.accumulated_channels_from_list = vec![];

        channels
    }

    pub fn process_names_end_message(&mut self) -> HashMap<String, Vec<String>> {
        let mut channels_and_clients: HashMap<String, Vec<String>> = HashMap::new();
        for i in 0..self.accumulated_channels_from_names.len() {
            let channel = self.accumulated_channels_from_names[i].clone();
            let clients_in_channel = self.accumulated_clients_from_names[i].clone();
            channels_and_clients.insert(channel, clients_in_channel);
        }
        self.accumulated_channels_from_names = vec![];
        self.accumulated_clients_from_names = vec![];

        channels_and_clients
    }
}
