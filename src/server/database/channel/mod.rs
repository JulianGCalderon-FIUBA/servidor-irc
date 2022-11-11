use std::{cell::RefCell, rc::Rc};

use crate::server::client_trait::Connection;

use super::Client;
/// A Channel has clients and a name.
pub struct Channel<C: Connection> {
    _name: String,
    //vector de nicknames
    clients: Vec<Rc<RefCell<Client<C>>>>,
    //nickname del operador
    operator: String,
    topic: Option<String>,
}

impl<C: Connection> Channel<C> {
    /// Creates a new [`Channel`].
    pub fn new(_name: String, creator: Rc<RefCell<Client<C>>>) -> Self {
        let operator = creator.borrow().nickname();
        let clients = vec![creator];

        Self {
            _name,
            clients,
            operator,
            topic: None,
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

    pub fn operator(&self) -> String {
        self.operator.clone()
    }
}
