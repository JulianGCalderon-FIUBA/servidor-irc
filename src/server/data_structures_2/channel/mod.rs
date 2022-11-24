mod channel_configuration;
pub use channel_configuration::ChannelConfiguration;

/// A Channel has clients and a name.
pub struct Channel {
    pub name: String,
    pub clients: Vec<String>,
    pub topic: Option<String>,
    pub config: ChannelConfiguration,
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
            topic: None,
            config,
        }
    }
}
