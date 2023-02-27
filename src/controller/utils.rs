use gtk4::{
    glib::{MainContext, Receiver, Sender, PRIORITY_DEFAULT},
    traits::ButtonExt,
    Button,
};
use std::collections::HashMap;

const CHANNEL_FIRST_CHARACTER: &str = "#";
pub const OPERATOR_CHARACTER: char = '@';

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

/// Returns first word of a button.
/// 
/// Receives a reference to a Button and returns a String. 
pub fn first_word_of_button(button: &Button) -> String {
    let text = button.label().unwrap().to_string();
    text.split_whitespace().collect::<Vec<&str>>()[0].to_string()
}

/// Returns a sender and a receiver.
/// 
/// Receives nothing, returns a String Sender and a String Receiver.
pub fn get_sender_and_receiver() -> (Sender<String>, Receiver<String>) {
    MainContext::channel(PRIORITY_DEFAULT)
}

/// Returns a bool indicating if the conversation is a channel or not.
///
/// Receives a String, returns a bool
pub fn is_channel(parameter: &str) -> bool {
    parameter.starts_with(CHANNEL_FIRST_CHARACTER)
}

/// Returns a bool indicating if the string is empty.
/// 
/// Receives a reference to a string, returns a bool.
pub fn is_not_empty(text: &str) -> bool {
    !text.is_empty()
}

/// Pushes an element to a vector only if the element is not in the vector.
/// 
/// Receives two references to a string vector and a String element, returns nothing.
pub fn push_if_absent(original_vector: &[String], new_vector: &mut Vec<String>, element: String) {
    if !original_vector.contains(&element) {
        new_vector.push(element);
    }
}

/// Removes element from a vector only if it is present.
/// 
/// Receives a reference to a string vector and a reference to a string, returns nothing.
pub fn remove_element(vector: &mut Vec<String>, element: &String) {
    if vector.contains(element) {
        vector.remove(vector.iter().position(|x| x == element).unwrap());
    }
}

/// Removes the operator indicator from a string.
/// 
/// Receives a string reference, returns a string.
pub fn remove_operator_indicator(element: &str) -> String {
    if let Some(stripped) = element.strip_prefix(OPERATOR_CHARACTER) {
        stripped.to_string()
    } else {
        element.to_string()
    }
}

/// Returns a bool indicating if the vector is not empty.
/// 
/// Receives a reference to a vector, returns a bool.
pub fn vec_is_not_empty(vector: &Vec<String>) -> bool {
    !vector.is_empty()
}
