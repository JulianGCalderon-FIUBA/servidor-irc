pub struct ChannelInfo {
    pub name: String,
    //vector de nicknames
    clients: Vec<String>,
    //nickname del operador
    _operator: Option<String>,
}

impl ChannelInfo {
    /// Creates a new [`_ChannelInfo`].
    pub fn _new(_name: String, _creator: String) -> Self {
        todo!()
    }

    pub fn _get_clients(&self) -> Vec<String> {
        todo!()
    }

    pub fn add_client(&mut self, client: String) {
        self.clients.push(client);
    }

    pub fn remove_client(&mut self, _client: String) {
        let index = self
            .clients
            .iter()
            .position(|client| *client == _client)
            .unwrap();
        self.clients.remove(index);
    }
}
