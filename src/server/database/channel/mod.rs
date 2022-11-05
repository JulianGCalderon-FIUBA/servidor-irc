pub struct Channel {
    _name: String,
    //vector de nicknames
    clients: Vec<String>,
    //nickname del operador
    _operator: Option<String>,
}

impl Channel {
    /// Creates a new [`_ChannelInfo`].
    pub fn new(_name: String, creator: String) -> Self {
        let clients = vec![creator];

        Self {
            _name,
            clients,
            _operator: None,
        }
    }

    pub fn get_clients(&self) -> Vec<String> {
        self.clients.clone()
    }

    pub fn add_client(&mut self, client: &str) {
        self.clients.push(client.to_string());
    }

    pub fn remove_client(&mut self, client: &str) {
        let index = self.clients.iter().position(|c| *c == client).unwrap();
        self.clients.remove(index);
    }
}
