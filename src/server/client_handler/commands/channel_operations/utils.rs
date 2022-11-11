use std::io;

use crate::server::{
    client_handler::{
        responses::{notifications::Notification, replies::CommandResponse},
        ClientHandler,
    },
    client_trait::Connection,
};

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
    pub fn send_topic_reply(&mut self, channel: String) -> Result<(), io::Error> {
        match self.database.get_topic_for_channel(&channel) {
            Some(topic) => {
                self.send_response_for_reply(CommandResponse::Topic332 { channel, topic })?
            }
            None => self.send_response_for_reply(CommandResponse::NoTopic331 { channel })?,
        };
        Ok(())
    }

    pub fn kick_client_from_channel(
        &mut self,
        nickname: &str,
        channel: &str,
        comment: &Option<String>,
    ) {
        self.database.remove_client_from_channel(nickname, channel);
        let notification = Notification::Kick {
            kicker: self.registration.nickname().unwrap(),
            channel: channel.to_string(),
            nickname: nickname.to_string(),
            comment: comment.clone(),
        };

        self.send_message_to_client(nickname, &notification.to_string())
            .ok();
    }
}

pub fn collect_parameters(parameters: &str) -> Vec<String> {
    parameters
        .split(',')
        .map(|string| string.to_string())
        .collect()
}
