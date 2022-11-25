mod channel_configuration;
pub use channel_configuration::ChannelConfiguration;

use crate::server::consts::modes::ChannelFlag;

/// A Channel has clients and a name.
pub struct Channel {
    pub name: String,
    pub clients: Vec<String>,
    pub topic: Option<String>,
    pub config: ChannelConfiguration,
}

impl Channel {
    /// Creates a new [`Channel`].
    pub fn new(name: &str, creator: &str) -> Self {
        let clients = vec![creator.to_string()];

        let mut config = ChannelConfiguration::new();
        config.operators.push(creator.to_string());

        Self {
            name: name.to_string(),
            clients,
            topic: None,
            config,
        }
    }

    pub fn add_client(&mut self, nickname: &str) {
        self.clients.push(nickname.to_string())
    }

    pub fn remove_client(&mut self, nickname: &str) {
        if let Some(index) = self.clients.iter().position(|nick| nick == nickname) {
            self.clients.remove(index);
        }
    }

    pub fn get_clients(&self) -> Vec<String> {
        self.clients.clone()
    }

    pub fn contains_client(&self, nickname: &str) -> bool {
        self.clients.contains(&nickname.to_string())
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

    pub fn get_config(&self) -> Option<ChannelConfiguration> {
        Some(self.config.clone())
    }

    pub fn update_nickname(&mut self, old_nickname: &str, new_nickname: &str) {
        for client in &mut self.clients {
            if client == old_nickname {
                *client = new_nickname.to_string()
            }
        }
    }
}
