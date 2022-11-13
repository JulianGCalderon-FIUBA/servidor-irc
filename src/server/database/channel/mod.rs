use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::server::client_trait::Connection;

use super::Client;
/// A Channel has clients and a name.
pub struct Channel<C: Connection> {
    _name: String,
    //vector de nicknames
    clients: Vec<Rc<RefCell<Client<C>>>>,
    //nickname del operador
    _operator: Option<String>,
    topic: Option<String>,
    key: Option<String>,
    modes: HashMap<char, bool>,
    limit: Option<isize>,
    operators: Vec<String>,
    speakers: Vec<String>,
    banmasks: Vec<String>,
}

impl<C: Connection> Channel<C> {
    /// Creates a new [`Channel`].
    pub fn new(_name: String, creator: Rc<RefCell<Client<C>>>) -> Self {
        let clients = vec![creator];

        Self {
            _name,
            clients,
            _operator: None,
            topic: None,
            key: None,
            modes: initialize_modes(),
            limit: None,
            operators: vec![],
            speakers: vec![],
            banmasks: vec![],
        }
    }

    /// Returns clients in Channel.
    pub fn get_clients(&self) -> Vec<String> {
        let mut names = vec![];
        for client in self.clients.iter() {
            names.push(client.borrow().nickname());
        }
        names
    }

    /// Adds client to Channel.
    pub fn add_client(&mut self, client: Rc<RefCell<Client<C>>>) {
        self.clients.push(client);
    }

    /// Returns true if the client is in Channel.
    pub fn contains_client(&self, nickname: &str) -> bool {
        self.clients
            .iter()
            .any(|c| c.borrow().nickname() == nickname)
    }

    /// Removes client from Channel.
    pub fn remove_client(&mut self, client: &str) {
        let index = self
            .clients
            .iter()
            .position(|c| c.borrow().had_nickname(client))
            .unwrap();
        self.clients.remove(index);
    }

    pub fn set_topic(&mut self, topic: &str) {
        self.topic = Some(topic.to_string())
    }

    pub fn get_topic(&self) -> Option<String> {
        self.topic.clone()
    }

    pub fn set_key(&mut self, key: Option<String>) {
        self.key = key
    }

    pub fn get_key(&self) -> Option<String> {
        self.key.clone()
    }

    pub fn set_mode(&mut self, mode: char) {
        self.modes.entry(mode).and_modify(|value| *value = true);
    }

    pub fn unset_mode(&mut self, mode: char) {
        self.modes.entry(mode).and_modify(|value| *value = false);
    }

    pub fn has_mode(&self, mode: char) -> bool {
        let (_key, value) = self.modes.get_key_value(&mode).unwrap();

        *value
    }

    pub fn get_limit(&self) -> Option<isize> {
        self.limit
    }

    pub fn set_limit(&mut self, limit: Option<isize>) {
        self.limit = limit
    }

    pub fn add_operator(&mut self, nickname: String) {
        self.operators.push(nickname)
    }

    pub fn remove_operator(&mut self, nickname: String) {
        self.operators
            .iter()
            .position(|nick| nick == &nickname)
            .map(|index| self.operators.remove(index));
    }

    pub fn add_speaker(&mut self, nickname: String) {
        self.speakers.push(nickname)
    }

    pub fn remove_speaker(&mut self, nickname: String) {
        self.speakers
            .iter()
            .position(|nick| nick == &nickname)
            .map(|index| self.speakers.remove(index));
    }

    pub fn is_speaker(&self, nickname: String) -> bool {
        self.speakers.contains(&nickname)
    }

    pub fn set_banmask(&mut self, mask: String) {
        self.banmasks.push(mask)
    }

    pub fn get_banmasks(&self) -> Vec<String> {
        self.banmasks.clone()
    }

    pub fn unset_banmask(&mut self, mask: String) {
        self.banmasks
            .iter()
            .position(|m| m == &mask)
            .map(|index| self.speakers.remove(index));
    }
}

fn initialize_modes() -> HashMap<char, bool> {
    let mut modes = HashMap::new();
    modes.insert('p', false);
    modes.insert('s', false);
    modes.insert('i', false);
    modes.insert('t', false);
    modes.insert('n', false);
    modes.insert('m', false);

    modes
}
