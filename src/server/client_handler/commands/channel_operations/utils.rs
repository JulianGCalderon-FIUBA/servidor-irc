use crate::server::{client_handler::ClientHandler, client_trait::Connection};

impl<C: Connection> ClientHandler<C> {
    /// Gets all channels that meet query.
    pub fn get_channels_for_query(&mut self, channels: Option<&String>) -> Vec<String> {
        if channels.is_none() {
            let mut channels = self.database.get_all_channels();
            channels.sort();
            return channels;
        }

        collect_parameters(channels.unwrap())
    }
}

pub fn collect_parameters(parameters: &str) -> Vec<String> {
    parameters
        .split(',')
        .map(|string| string.to_string())
        .collect()
}
