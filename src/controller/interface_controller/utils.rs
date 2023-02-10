use std::collections::HashMap;

use gtk4::glib::Sender;

use crate::controller::{
    controller_message::ControllerMessage::{self, OpenWarningView},
    OPEN_WARNING_ERROR_TEXT,
};

/// Returns channels that are not from the current user.
///
/// Receives a Vec<String> and a Vec<String>, returns a Vec<String>
pub fn channels_not_mine(all: Vec<String>, mine: Vec<String>) -> Vec<String> {
    let mut not_mine: Vec<String> = vec![];
    for element in &all {
        push_if_absent(&mine, &mut not_mine, element.to_string());
    }
    not_mine
}

/// Returns all channels from a client.
///
/// Receives a HashMap<String, Vec<String>> and a String, returns a Vec<String>
pub fn client_channels(
    channels_and_clients: HashMap<String, Vec<String>>,
    client: String,
) -> Vec<String> {
    let mut client_channels_set: Vec<String> = vec![];
    for channel in channels_and_clients.keys() {
        let mut clients: Vec<String> = vec![];
        for element in channels_and_clients.get(channel).unwrap() {
            let element_without_operator_indicator: String = remove_operator_indicator(element);
            clients.push(element_without_operator_indicator);
        }
        if clients.contains(&client) {
            client_channels_set.push((&channel).to_string());
        }
    }
    client_channels_set
}

/// Returns clients that are not from the current user.
///
/// Receives a Vec<String> and a Vec<String>, returns a Vec<String>
pub fn clients_not_mine(all: Vec<String>, mine: Vec<String>) -> Vec<String> {
    let mut not_mine: Vec<String> = vec![];
    for element in &all {
        let element_without_operator_indicator: String = remove_operator_indicator(element);
        push_if_absent(&mine, &mut not_mine, element_without_operator_indicator);
    }
    not_mine
}

/// Returns all clients to add.
///
/// Receives a HashMap<String, Vec<String>> and a String, returns a Vec<String>
pub fn clients_to_add(
    channels_and_clients: HashMap<String, Vec<String>>,
    current_nickname: String,
) -> Vec<String> {
    let mut all_clients = server_clients(channels_and_clients);
    remove_myself(&mut all_clients, current_nickname);
    all_clients
}

pub fn is_not_empty(vector: &Vec<String>) -> bool {
    !vector.is_empty()
}

pub fn push_if_absent(original_vector: &[String], new_vector: &mut Vec<String>, element: String) {
    if !original_vector.contains(&element) {
        new_vector.push(element);
    }
}

pub fn remove_element(vector: &mut Vec<String>, element: &String) {
    if vector.contains(element) {
        vector.remove(vector.iter().position(|x| x == element).unwrap());
    }
}

pub fn remove_myself(all_clients: &mut Vec<String>, current_nickname: String) {
    remove_element(all_clients, &current_nickname);
    remove_element(all_clients, &format!("@{current_nickname}"));
}

pub fn remove_operator_indicator(element: &str) -> String {
    if let Some(stripped) = element.strip_prefix('@') {
        stripped.to_string()
    } else {
        element.to_string()
    }
}

pub fn send_open_warning_view(sender: Sender<ControllerMessage>, warning_text: &str) {
    sender
        .send(OpenWarningView {
            message: warning_text.to_string(),
        })
        .expect(OPEN_WARNING_ERROR_TEXT);
}

/// Returns all server clients.
///
/// Receives a HashMap<String, Vec<String>>, returns a Vec<String>
pub fn server_clients(channels_and_clients: HashMap<String, Vec<String>>) -> Vec<String> {
    let mut clients_set: Vec<String> = vec![];
    for clients_of_channel in channels_and_clients.values() {
        for client in clients_of_channel {
            push_if_absent(&clients_set.clone(), &mut clients_set, client.to_string());
        }
    }
    clients_set
}
