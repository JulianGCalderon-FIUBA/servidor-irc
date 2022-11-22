use self::channel_config::ChannelConfig;

use super::*;
use crate::server::{connection::Connection, consts::modes::ChannelFlag};
use std::{cell::RefCell, rc::Rc};

pub mod channel_config;

/// A Channel has clients and a name.
pub struct Channel<C: Connection> {
    _name: String,
    clients: Vec<Rc<RefCell<Client<C>>>>,
    topic: Option<String>,
    config: ChannelConfig,
}

impl<C: Connection> Channel<C> {
    /// Creates a new [`Channel`].
    pub fn new(_name: String, creator: Rc<RefCell<Client<C>>>) -> Self {
        let operator = creator.borrow().nickname();
        let clients = vec![creator];

        let mut config = ChannelConfig::new();
        config.operators.push(operator);

        Self {
            _name,
            clients,
            topic: None,
            config,
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
        if let Some(index) = self
            .clients
            .iter()
            .position(|c| c.borrow().had_nickname(client))
        {
            self.clients.remove(index);
        }
    }

    pub fn set_topic(&mut self, topic: &str) {
        self.topic = Some(topic.to_string())
    }

    pub fn get_topic(&self) -> Option<String> {
        self.topic.clone()
    }

    pub fn set_key(&mut self, key: Option<String>) {
        self.config.key = key
    }

    pub fn get_key(&self) -> Option<String> {
        self.config.key.clone()
    }

    pub fn set_mode(&mut self, flag: ChannelFlag) {
        self.config.flags.push(flag)
    }

    pub fn unset_mode(&mut self, flag: ChannelFlag) {
        self.config
            .flags
            .iter()
            .position(|f| f == &flag)
            .map(|index| self.config.flags.remove(index));
    }

    pub fn has_mode(&self, flag: ChannelFlag) -> bool {
        self.config.flags.contains(&flag)
    }

    pub fn get_limit(&self) -> Option<usize> {
        self.config.user_limit
    }

    pub fn set_limit(&mut self, limit: Option<usize>) {
        self.config.user_limit = limit
    }

    pub fn add_operator(&mut self, nickname: String) {
        self.config.operators.push(nickname)
    }

    pub fn remove_operator(&mut self, nickname: String) {
        self.config
            .operators
            .iter()
            .position(|nick| nick == &nickname)
            .map(|index| self.config.operators.remove(index));
    }

    pub fn add_speaker(&mut self, nickname: String) {
        self.config.speakers.push(nickname)
    }

    pub fn remove_speaker(&mut self, nickname: String) {
        self.config
            .speakers
            .iter()
            .position(|nick| nick == &nickname)
            .map(|index| self.config.speakers.remove(index));
    }

    pub fn is_speaker(&self, nickname: String) -> bool {
        self.config.speakers.contains(&nickname)
    }

    pub fn add_banmask(&mut self, mask: String) {
        self.config.banmasks.push(mask)
    }

    pub fn get_banmasks(&self) -> Vec<String> {
        self.config.banmasks.clone()
    }

    pub fn remove_banmask(&mut self, mask: String) {
        self.config
            .banmasks
            .iter()
            .position(|m| m == &mask)
            .map(|index| self.config.banmasks.remove(index));
    }

    pub fn is_operator(&self, nickname: &str) -> bool {
        self.config.operators.contains(&nickname.to_string())
    }
}
