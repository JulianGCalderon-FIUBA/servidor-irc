use std::sync::mpsc::Sender;

use crate::server::{
    connection::Connection, consts::modes::ChannelFlag, data_structures::ChannelConfiguration,
    database::Database, debug_print,
};

impl<C: Connection> Database<C> {
    pub fn handle_get_channel_topic(&self, channel: String, respond_to: Sender<Option<String>>) {
        let topic = self.get_channel_topic(&channel);
        respond_to.send(topic).unwrap();
    }

    pub fn handle_set_channel_key(&mut self, channel_name: String, key: Option<String>) {
        if let Some(channel) = self.channels.get_mut(&channel_name) {
            debug_print!("Setting {channel_name}'s key to {key:?}");

            channel.set_key(key);
        }
    }

    pub fn handle_channel_has_mode(
        &self,
        channel: String,
        flag: ChannelFlag,
        respond_to: Sender<bool>,
    ) {
        let has_mode = self.channel_has_mode(&channel, &flag);
        respond_to.send(has_mode).unwrap();
    }

    pub fn handle_get_channel_key(&self, channel: String, respond_to: Sender<Option<String>>) {
        let key = self.get_channel_key(channel);
        respond_to.send(key).unwrap();
    }

    pub fn handle_set_mode(&mut self, channel_name: String, flag: ChannelFlag) {
        if let Some(channel) = self.channels.get_mut(&channel_name) {
            debug_print!("Setting {channel_name}'s mode {flag:?}");

            channel.set_mode(flag);
        }
    }

    pub fn handle_unset_mode(&mut self, channel_name: String, flag: ChannelFlag) {
        if let Some(channel) = self.channels.get_mut(&channel_name) {
            debug_print!("Unsetting {channel_name}'s mode {flag:?}");

            channel.unset_mode(&flag);
        }
    }

    pub fn handle_set_channel_limit(&mut self, channel_name: String, limit: Option<usize>) {
        if let Some(channel) = self.channels.get_mut(&channel_name) {
            debug_print!("Setting {channel_name}'s limit to {limit:?}");

            channel.set_limit(limit)
        }
    }

    pub fn handle_get_channel_limit(&self, channel: String, respond_to: Sender<Option<usize>>) {
        let limit = self.get_channel_limit(&channel);
        respond_to.send(limit).unwrap();
    }

    pub fn handle_add_channop(&mut self, channel_name: String, nickname: String) {
        if let Some(channel) = self.channels.get_mut(&channel_name) {
            debug_print!("Setting {nickname} as operator of {channel_name}");

            channel.add_operator(nickname);
        }
    }

    pub fn handle_remove_channop(&mut self, channel_name: String, nickname: String) {
        if let Some(channel) = self.channels.get_mut(&channel_name) {
            debug_print!("Setting {nickname} as operator of {channel_name}");

            channel.remove_operator(&nickname);
        }
    }

    pub fn handle_add_speaker(&mut self, channel_name: String, nickname: String) {
        if let Some(channel) = self.channels.get_mut(&channel_name) {
            debug_print!("Setting {nickname} as speaker of {channel_name}");

            channel.add_speaker(nickname);
        }
    }

    pub fn handle_remove_speaker(&mut self, channel_name: String, nickname: String) {
        if let Some(channel) = self.channels.get_mut(&channel_name) {
            debug_print!("Unsetting {nickname} as speaker of {channel_name}");

            channel.remove_speaker(&nickname);
        }
    }

    pub fn handle_add_channel_banmask(&mut self, channel_name: String, mask: String) {
        if let Some(channel) = self.channels.get_mut(&channel_name) {
            debug_print!("Adding banmask {mask} to {channel_name}");

            channel.add_banmask(mask);
        }
    }

    pub fn handle_get_channel_banmask(&self, channel: String, respond_to: Sender<Vec<String>>) {
        let banmask = self.get_channel_banmask(&channel);
        respond_to.send(banmask).unwrap();
    }

    pub fn handle_remove_channel_banmask(&mut self, channel_name: String, mask: String) {
        if let Some(channel) = self.channels.get_mut(&channel_name) {
            debug_print!("Removing banmask {mask} from {channel_name}");

            channel.remove_banmask(&mask);
        }
    }

    pub fn handle_get_channel_config(
        &self,
        channel: String,
        respond_to: Sender<Option<ChannelConfiguration>>,
    ) {
        let channel_config = self.get_channel_config(&channel);
        respond_to.send(channel_config).unwrap();
    }
}

impl<C: Connection> Database<C> {
    pub fn get_channel_topic(&self, channel: &str) -> Option<String> {
        if let Some(channel) = self.channels.get(channel) {
            return channel.get_topic();
        }
        None
    }

    pub fn get_channel_key(&self, channel: String) -> Option<String> {
        if let Some(channel) = self.channels.get(&channel) {
            return channel.get_key();
        }
        None
    }

    pub fn get_channel_limit(&self, channel: &str) -> Option<usize> {
        if let Some(channel) = self.channels.get(channel) {
            return channel.get_limit();
        }
        None
    }

    pub fn get_channel_banmask(&self, channel: &str) -> Vec<String> {
        if let Some(channel) = self.channels.get(channel) {
            return channel.get_banmasks();
        }
        vec![]
    }

    pub fn get_channel_config(&self, channel: &str) -> Option<ChannelConfiguration> {
        if let Some(channel) = self.channels.get(channel) {
            return Some(channel.get_config());
        }
        None
    }

    pub fn set_channel_topic(&mut self, channel_name: String, topic: String) {
        if let Some(channel) = self.channels.get_mut(&channel_name) {
            debug_print!("Setting {channel_name}'s topic to {topic}");

            channel.set_topic(topic);
        }
    }

    fn channel_has_mode(&self, channel: &str, mode: &ChannelFlag) -> bool {
        if let Some(channel) = self.channels.get(channel) {
            return channel.has_mode(mode);
        }
        false
    }
}
