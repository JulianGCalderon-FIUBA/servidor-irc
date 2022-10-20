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

    pub fn get_clients(&self) -> Vec<String> {
        self.clients.clone()
    }

    pub fn _add_client(&mut self, _client: String) {
        todo!()
    }

    pub fn _remove_client(&mut self, _client: String) {
        todo!()
    }
}
