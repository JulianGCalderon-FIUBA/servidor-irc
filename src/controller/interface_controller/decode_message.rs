use std::collections::HashMap;

use crate::message::Message;

use super::{utils::is_channel, InterfaceController};

pub fn get_message_prefix(message: &Message) -> String {
    message.get_prefix().clone().unwrap()
}

pub fn get_message_trailing(message: &Message) -> String {
    message.get_trailing().clone().unwrap()
}

pub fn get_message_parameter(message: &Message, index: usize) -> String {
    message.get_parameters()[index].clone()
}

impl InterfaceController {
    pub fn decode_invite_message(&mut self, message: Message) -> (String, String) {
        let channel = get_message_parameter(&message, 1);
        let nickname = get_message_prefix(&message);

        (channel, nickname)
    }

    pub fn decode_join_message(&mut self, message: Message) -> String {
        message.get_parameters()[0].clone() // channel
    }

    pub fn decode_kick_message(&mut self, message: Message) -> (String, String) {
        let channel = get_message_parameter(&message, 0);
        let kicked = get_message_parameter(&message, 1);

        (channel, kicked)
    }

    pub fn process_list_end_message(&mut self) -> Vec<String> {
        let channels: Vec<String> = self.accumulated_channels_from_list.clone();
        self.accumulated_channels_from_list = vec![];

        channels
    }

    pub fn decode_list_line_message(&mut self, message: Message) -> String {
        get_message_parameter(&message, 0) // channel
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

    pub fn decode_names_line_message(&mut self, message: Message) -> (String, Vec<String>) {
        let channels = get_message_parameter(&message, 0);
        let trailing: String = get_message_trailing(&message);
        let clients: Vec<String> = trailing.split(' ').map(|s| s.to_string()).collect();

        (channels, clients)
    }

    pub fn decode_priv_message(&mut self, message: Message) -> (Option<String>, String, String) {
        let message_text = get_message_trailing(&message);
        let sender_nickname = get_message_prefix(&message);
        let channel_value = get_message_parameter(&message, 0);
        let channel = if is_channel(channel_value.clone()) {
            Some(channel_value)
        } else {
            None
        };

        (channel, message_text, sender_nickname)
    }

    pub fn decode_registration(&mut self, message: Message) -> (String, String, String, String) {
        let trailing_text = get_message_trailing(&message);
        let trailing_strings = trailing_text.split(' ').collect::<Vec<&str>>();
        println!("{trailing_strings:?}");
        let mut username = trailing_strings[5].to_string();
        username.remove(0);
        let realname = get_message_parameter(&message, 0);
        let servername = trailing_strings[2].to_string();
        let nickname = trailing_strings[4].to_string();

        (nickname, realname, servername, username)
    }
}
