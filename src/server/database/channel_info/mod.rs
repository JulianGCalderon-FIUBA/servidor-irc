pub struct ChannelInfo {
    pub name: String,
    //vector de nicknames
    _clients: Vec<String>,
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

    pub fn _add_client(&mut self, _client: String) {
        todo!()
    }

    pub fn _remove_client(&mut self, _client: String) {
        let index = self
            ._clients
            .iter()
            .position(|client| *client == _client)
            .unwrap();
        self._clients.remove(index);
    }
}
