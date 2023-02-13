use std::collections::HashMap;

use crate::{
    controller::{NAMES_ERROR_TEXT, OPEN_WARNING_ERROR_TEXT},
    server::consts::commands::NAMES_COMMAND,
};

use super::{names_message_intention::NamesMessageIntention, InterfaceController};

use crate::controller::controller_message::ControllerMessage::OpenWarningView;

impl InterfaceController {
    /// Returns all clients to add.
    ///
    /// Receives a HashMap<String, Vec<String>> and a String, returns a Vec<String>
    pub fn all_clients_except_me(
        &mut self,
        channels_and_clients: HashMap<String, Vec<String>>,
    ) -> Vec<String> {
        let mut all_clients = all_clients(channels_and_clients);
        self.remove_myself(&mut all_clients);
        all_clients
    }

    /// Returns clients that are not from the current user.
    ///
    /// Receives a Vec<String> and a Vec<String>, returns a Vec<String>
    pub fn clients_not_mine(
        &mut self,
        channels_and_clients: HashMap<String, Vec<String>>,
    ) -> Vec<String> {
        let all_clients = self.all_clients_except_me(channels_and_clients);
        let my_clients = self.main_view.get_my_clients();
        let mut not_mine: Vec<String> = vec![];

        for element in &all_clients {
            let element_without_operator_indicator: String = remove_operator_indicator(element);
            push_if_absent(
                &my_clients,
                &mut not_mine,
                element_without_operator_indicator,
            );
        }

        not_mine
    }

    pub fn current_conv_channels(
        &mut self,
        channels_and_clients: HashMap<String, Vec<String>>,
    ) -> Vec<String> {
        client_channels(channels_and_clients, self.current_conv.clone())
    }

    pub fn remove_myself(&mut self, all_clients: &mut Vec<String>) {
        let current_nickname = self.current_nickname.clone();
        remove_element(all_clients, &current_nickname);
        remove_element(all_clients, &format!("@{current_nickname}"));
    }

    pub fn send_names_message(
        &mut self,
        intention: NamesMessageIntention,
        parameter: Option<String>,
    ) {
        self.names_message_intention = intention;
        let parameter_value = parameter.unwrap_or_default();
        let message = format!("{NAMES_COMMAND} {}", parameter_value);
        self.client.send(&message).expect(NAMES_ERROR_TEXT);
    }

    pub fn send_open_warning_view(&mut self, warning_text: &str) {
        let to_send = OpenWarningView {
            message: warning_text.to_string(),
        };
        self.sender.send(to_send).expect(OPEN_WARNING_ERROR_TEXT);
    }
}

/// Returns all clients.
///
/// Receives a HashMap<String, Vec<String>>, returns a Vec<String>
pub fn all_clients(channels_and_clients: HashMap<String, Vec<String>>) -> Vec<String> {
    let mut clients_set: Vec<String> = vec![];
    for clients_of_channel in channels_and_clients.values() {
        for client in clients_of_channel {
            push_if_absent(&clients_set.clone(), &mut clients_set, client.to_string());
        }
    }
    clients_set
}

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

pub fn remove_operator_indicator(element: &str) -> String {
    if let Some(stripped) = element.strip_prefix('@') {
        stripped.to_string()
    } else {
        element.to_string()
    }
}
