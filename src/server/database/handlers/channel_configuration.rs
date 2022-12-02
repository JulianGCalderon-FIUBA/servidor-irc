use std::sync::mpsc::Sender;

use crate::macros::{debug_print, some_or_return};
use crate::server::database::database_error::DatabaseError;
use crate::server::{
    connection::Connection, consts::modes::ChannelFlag, data_structures::ChannelConfiguration,
    database::Database,
};

impl<C: Connection> Database<C> {
    pub fn handle_get_channel_topic(
        &self,
        channel: String,
        respond_to: Sender<Result<Option<String>, DatabaseError>>,
    ) {
        let topic = self.get_channel_topic(&channel);
        respond_to
            .send(topic)
            .expect("Handler receiver should not be dropped");
    }
    pub fn handle_set_channel_topic(&mut self, channel_name: String, topic: String) {
        self.set_channel_topic(channel_name, topic);
    }

    pub fn handle_set_channel_key(&mut self, channel_name: String, key: Option<String>) {
        self.set_channel_key(channel_name, key);
    }

    pub fn handle_get_channel_key(
        &self,
        channel: String,
        respond_to: Sender<Result<Option<String>, DatabaseError>>,
    ) {
        let key = self.get_channel_key(channel);
        respond_to
            .send(key)
            .expect("Handler receiver should not be dropped");
    }

    pub fn handle_channel_has_flag(
        &self,
        channel: String,
        flag: ChannelFlag,
        respond_to: Sender<bool>,
    ) {
        let has_mode = self.channel_has_flag(channel, flag);
        respond_to
            .send(has_mode)
            .expect("Handler receiver should not be dropped");
    }

    pub fn handle_set_channel_flag(&mut self, channel_name: String, flag: ChannelFlag) {
        self.set_channel_flag(channel_name, flag);
    }

    pub fn handle_unset_channel_flag(&mut self, channel_name: String, flag: ChannelFlag) {
        self.unset_channel_flag(channel_name, flag);
    }

    pub fn handle_set_channel_limit(&mut self, channel_name: String, limit: Option<usize>) {
        self.set_channel_limit(channel_name, limit);
    }

    pub fn handle_get_channel_limit(
        &self,
        channel: String,
        respond_to: Sender<Result<Option<usize>, DatabaseError>>,
    ) {
        let limit = self.get_channel_limit(channel);
        respond_to
            .send(limit)
            .expect("Handler receiver should not be dropped");
    }

    pub fn handle_add_channop(&mut self, channel_name: String, nickname: String) {
        self.add_channop(channel_name, nickname);
    }

    pub fn handle_remove_channop(&mut self, channel_name: String, nickname: String) {
        self.remove_channop(channel_name, nickname);
    }

    pub fn handle_add_channel_speaker(&mut self, channel_name: String, nickname: String) {
        self.add_channel_speaker(channel_name, nickname);
    }

    pub fn handle_remove_channel_speaker(&mut self, channel_name: String, nickname: String) {
        self.remove_channel_speaker(channel_name, nickname);
    }

    pub fn handle_add_channel_banmask(&mut self, channel_name: String, mask: String) {
        self.add_channel_banmask(channel_name, mask);
    }

    pub fn handle_get_channel_banmask(
        &self,
        channel: String,
        respond_to: Sender<Result<Vec<String>, DatabaseError>>,
    ) {
        let banmask = self.get_channel_banmask(channel);
        respond_to
            .send(Ok(banmask))
            .expect("Handler receiver should not be dropped");
    }

    pub fn handle_remove_channel_banmask(&mut self, channel_name: String, mask: String) {
        self.remove_channel_banmask(channel_name, mask);
    }

    pub fn handle_get_channel_config(
        &self,
        channel: String,
        respond_to: Sender<Result<ChannelConfiguration, DatabaseError>>,
    ) {
        let channel_config = self.get_channel_config(&channel);
        respond_to
            .send(channel_config)
            .expect("Handler receiver should not be dropped");
    }
}

impl<C: Connection> Database<C> {
    fn remove_channel_banmask(&mut self, channel_name: String, mask: String) {
        let channel = some_or_return!(self.channels.get_mut(&channel_name));
        debug_print!("Removing banmask {mask} from {channel_name}");

        channel.remove_banmask(&mask);
    }
    fn add_channel_banmask(&mut self, channel_name: String, mask: String) {
        let channel = some_or_return!(self.channels.get_mut(&channel_name));
        debug_print!("Adding banmask {mask} to {channel_name}");

        channel.add_banmask(mask);
    }
    pub fn get_channel_banmask(&self, channel: String) -> Vec<String> {
        let channel = some_or_return!(self.channels.get(&channel), vec![]);

        channel.get_banmasks()
    }

    fn remove_channel_speaker(&mut self, channel_name: String, nickname: String) {
        let channel = some_or_return!(self.channels.get_mut(&channel_name));
        debug_print!("Unsetting {nickname} as speaker of {channel_name}");

        channel.remove_speaker(&nickname);
    }
    fn add_channel_speaker(&mut self, channel_name: String, nickname: String) {
        let channel = some_or_return!(self.channels.get_mut(&channel_name));
        debug_print!("Setting {nickname} as speaker of {channel_name}");

        channel.add_speaker(nickname);
    }

    pub fn get_channel_limit(&self, channel: String) -> Result<Option<usize>, DatabaseError> {
        let channel = some_or_return!(
            self.channels.get(&channel),
            Err(DatabaseError::NoSuchChannel)
        );
        Ok(channel.get_limit())
    }
    fn set_channel_limit(&mut self, channel: String, limit: Option<usize>) {
        let channel = some_or_return!(self.channels.get_mut(&channel));
        debug_print!("Setting {}'s limit to {limit:?}", channel.name);
        channel.set_limit(limit);
    }

    fn add_channop(&mut self, channel_name: String, nickname: String) {
        let channel = some_or_return!(self.channels.get_mut(&channel_name));
        debug_print!("Setting {nickname} as operator of {channel_name}");

        channel.add_operator(nickname);
    }
    fn remove_channop(&mut self, channel_name: String, nickname: String) {
        let channel = some_or_return!(self.channels.get_mut(&channel_name));
        debug_print!("Removing {nickname} as operator of {channel_name}");

        channel.remove_operator(&nickname);
    }

    fn set_channel_flag(&mut self, channel_name: String, flag: ChannelFlag) {
        let channel = some_or_return!(self.channels.get_mut(&channel_name));
        debug_print!("Setting {channel_name}'s mode {flag:?}");

        channel.set_mode(flag);
    }
    fn unset_channel_flag(&mut self, channel_name: String, flag: ChannelFlag) {
        let channel = some_or_return!(self.channels.get_mut(&channel_name));
        debug_print!("Unsetting {channel_name}'s mode {flag:?}");

        channel.unset_mode(flag);
    }
    fn channel_has_flag(&self, channel: String, mode: ChannelFlag) -> bool {
        let channel = some_or_return!(self.channels.get(&channel), false);
        channel.has_mode(mode)
    }

    pub fn get_channel_key(&self, channel: String) -> Result<Option<String>, DatabaseError> {
        let channel = some_or_return!(
            self.channels.get(&channel),
            Err(DatabaseError::NoSuchChannel)
        );
        Ok(channel.get_key())
    }
    fn set_channel_key(&mut self, channel_name: String, key: Option<String>) {
        let channel = some_or_return!(self.channels.get_mut(&channel_name));
        debug_print!("Setting {channel_name}'s key to {key:?}");
        channel.set_key(key)
    }

    fn set_channel_topic(&mut self, channel_name: String, topic: String) {
        let channel = some_or_return!(self.channels.get_mut(&channel_name));
        debug_print!("Setting {channel_name}'s topic to {topic}");
        channel.set_topic(topic)
    }
    pub fn get_channel_topic(&self, channel: &str) -> Result<Option<String>, DatabaseError> {
        let channel = some_or_return!(
            self.channels.get(channel),
            Err(DatabaseError::NoSuchChannel)
        );
        Ok(channel.get_topic())
    }

    pub fn get_channel_config(&self, channel: &str) -> Result<ChannelConfiguration, DatabaseError> {
        let channel = some_or_return!(
            self.channels.get(channel),
            Err(DatabaseError::NoSuchChannel)
        );
        Ok(channel.get_config())
    }
}
