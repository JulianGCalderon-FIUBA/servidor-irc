use std::{cell::RefCell, rc::Rc};

use crate::server::client_trait::ClientTrait;

use super::Client;

pub struct Channel<T: ClientTrait> {
    _name: String,
    //vector de nicknames
    clients: Vec<Rc<RefCell<Client<T>>>>,
    //nickname del operador
    _operator: Option<String>,
}

impl<T: ClientTrait> Channel<T> {
    /// Creates a new [`_ChannelInfo`].
    pub fn new(_name: String, creator: Rc<RefCell<Client<T>>>) -> Self {
        let clients = vec![creator];

        Self {
            _name,
            clients,
            _operator: None,
        }
    }

    pub fn get_clients(&self) -> Vec<String> {
        let mut names = vec![];
        for client in self.clients.iter() {
            names.push(client.borrow().nickname());
        }
        names
    }

    pub fn add_client(&mut self, client: Rc<RefCell<Client<T>>>) {
        self.clients.push(client);
    }

    pub fn contains_client(&self, nickname: &str) -> bool {
        self.clients
            .iter()
            .any(|c| c.borrow().nickname() == nickname)
    }

    pub fn remove_client(&mut self, client: &str) {
        let index = self
            .clients
            .iter()
            .position(|c| c.borrow().had_nickname(client))
            .unwrap();
        self.clients.remove(index);
    }
}
