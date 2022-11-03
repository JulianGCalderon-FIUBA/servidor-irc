use crate::server::{client_handler::ClientHandler, client_trait::ClientTrait};

impl<T: ClientTrait> ClientHandler<T> {
    pub fn get_channels_for_query(&mut self, channels: Option<&String>) -> Vec<String> {
        if channels.is_none() {
            let mut channels = self.database.get_channels();
            channels.sort();
            return channels;
        }

        channels
            .unwrap()
            .split(',')
            .map(|string| string.to_string())
            .collect()
    }
}
