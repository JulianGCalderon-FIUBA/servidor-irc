mod channel_configuration;
pub use channel_configuration::ChannelConfiguration;

use crate::server::consts::modes::ChannelFlag;

/// A Channel has clients and a name.
pub struct Channel {
    pub name: String,
    pub clients: Vec<String>,
    pub config: ChannelConfiguration,
    pub invites: Vec<String>,
}

impl Channel {
    /// Creates a new [`Channel`].
    pub fn new(name: String, creator: String) -> Self {
        let clients = vec![creator.clone()];

        let mut config = ChannelConfiguration::new();
        config.operators.push(creator);

        Self {
            name,
            clients,
            config,
            invites: Default::default(),
        }
    }

    pub fn add_banmask(&mut self, banmask: String) {
        self.config.banmasks.push(banmask)
    }

    pub fn add_member(&mut self, nickname: String) {
        self.clients.push(nickname)
    }

    pub fn add_operator(&mut self, nickname: String) {
        self.config.operators.push(nickname)
    }

    pub fn add_speaker(&mut self, nickname: String) {
        self.config.speakers.push(nickname)
    }

    pub fn get_banmasks(&self) -> Vec<String> {
        self.config.banmasks.clone()
    }

    pub fn get_clients(&self) -> Vec<String> {
        self.clients.clone()
    }

    pub fn get_config(&self) -> ChannelConfiguration {
        self.config.clone()
    }

    pub fn get_key(&self) -> Option<String> {
        self.config.key.clone()
    }

    pub fn get_limit(&self) -> Option<usize> {
        self.config.user_limit
    }

    pub fn get_topic(&self) -> Option<String> {
        self.config.topic.clone()
    }

    pub fn has_mode(&self, flag: ChannelFlag) -> bool {
        self.config.flags.contains(&flag)
    }

    pub fn is_member(&self, nickname: &str) -> bool {
        self.clients.iter().any(|n| n == nickname)
    }

    pub fn is_operator(&self, nickname: &str) -> bool {
        self.config.operators.iter().any(|n| n == nickname)
    }

    pub fn is_speaker(&self, nickname: &str) -> bool {
        self.config.speakers.iter().any(|n| n == nickname)
    }

    pub fn remove_banmask(&mut self, mask: &str) {
        remove(&mut self.config.banmasks, &mask.to_string())
    }

    pub fn remove_client(&mut self, nickname: &str) {
        remove_string(&mut self.clients, nickname);
    }

    pub fn remove_operator(&mut self, nickname: &str) {
        remove_string(&mut self.config.operators, nickname);
    }

    pub fn remove_speaker(&mut self, nickname: &str) {
        remove_string(&mut self.config.speakers, nickname)
    }

    pub fn set_key(&mut self, key: Option<String>) {
        self.config.key = key
    }

    pub fn set_limit(&mut self, limit: Option<usize>) {
        self.config.user_limit = limit
    }

    pub fn set_mode(&mut self, flag: ChannelFlag) {
        self.config.flags.push(flag)
    }

    pub fn set_topic(&mut self, topic: String) {
        self.config.topic = Some(topic)
    }

    pub fn unset_mode(&mut self, flag: ChannelFlag) {
        remove(&mut self.config.flags, &flag);
    }

    pub fn update_nickname(&mut self, old_nickname: &str, new_nickname: &str) {
        for client in &mut self.clients {
            if client == old_nickname {
                *client = new_nickname.to_string()
            }
        }
    }

    pub fn add_client_invite(&mut self, client: String) {
        self.invites.push(client);
    }

    pub fn has_invite(&self, client: &str) -> bool {
        self.invites.iter().any(|c| c == client)
    }
}

fn remove<T: Eq>(elements: &mut Vec<T>, element: &T) {
    elements
        .iter()
        .position(|e| e == element)
        .map(|index| elements.remove(index));
}

fn remove_string(elements: &mut Vec<String>, element: &str) {
    elements
        .iter()
        .position(|e| e == element)
        .map(|index| elements.remove(index));
}
