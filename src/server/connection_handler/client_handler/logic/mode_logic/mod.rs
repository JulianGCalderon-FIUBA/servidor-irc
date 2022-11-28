use std::io;

use crate::server::{
    connection::Connection,
    connection_handler::client_handler::ClientHandler,
    consts::modes::{ChannelFlag, ADD_MODE, REMOVE_MODE},
};

mod mode_request;
pub use mode_request::ModeRequest;

impl<C: Connection> ClientHandler<C> {
    pub(super) fn handle_mode_request(
        &mut self,
        channel: &str,
        request: ModeRequest,
    ) -> io::Result<()> {
        match request {
            ModeRequest::AddBanmask(banmask) => self.add_banmask_request(channel, banmask),
            ModeRequest::AddOperator(operator) => self.add_operator_request(channel, operator),
            ModeRequest::AddSpeaker(speaker) => self.add_speaker_request(channel, speaker),
            ModeRequest::RemoveBanmask(banmask) => self.remove_banmask_request(channel, banmask),
            ModeRequest::RemoveOperator(operator) => {
                self.remove_operator_request(channel, operator)
            }
            ModeRequest::RemoveSpeaker(speaker) => self.remove_speaker_request(channel, speaker),
            ModeRequest::SetFlag(flag) => self.set_flag_request(channel, flag),
            ModeRequest::SetKey(key) => self.set_key_request(channel, key),
            ModeRequest::SetLimit(limit) => self.set_limit_request(channel, limit),
            ModeRequest::UnsetLimit() => self.unset_limit_request(channel),
            ModeRequest::UnsetKey() => self.unset_key_request(channel),
            ModeRequest::UnsetFlag(flag) => self.unset_flag_request(channel, flag),

            ModeRequest::UnknownMode(character) => self.unknown_mode_request(character),
            ModeRequest::NeedArgument(character) => self.need_argument_request(character),
            ModeRequest::InvalidArgument(character, argument) => {
                self.invalid_argument_request(character, argument)
            }
            ModeRequest::GetBanmasks => self.get_banmasks_request(channel),
        }
    }

    fn add_banmask_request(&mut self, channel: &str, banmask: String) -> io::Result<()> {
        self.database.add_channel_banmask(channel, &banmask);

        Ok(())
    }

    fn get_banmasks_request(&mut self, channel: &str) -> io::Result<()> {
        self.send_banlist_response(channel)
    }

    fn add_operator_request(&mut self, channel: &str, operator: String) -> io::Result<()> {
        if let Err(error) = self.assert_can_modify_client_status_in_channel(channel, &operator) {
            return self.stream.send(&error);
        }

        self.database.add_channop(channel, &operator);

        Ok(())
    }
    fn add_speaker_request(&mut self, channel: &str, speaker: String) -> io::Result<()> {
        if let Err(error) = self.assert_can_modify_client_status_in_channel(channel, &speaker) {
            return self.stream.send(&error);
        }
        self.database.add_speaker(channel, &speaker);

        Ok(())
    }
    fn remove_banmask_request(&mut self, channel: &str, banmask: String) -> io::Result<()> {
        Ok(())
    }
    fn remove_operator_request(&mut self, channel: &str, operator: String) -> io::Result<()> {
        Ok(())
    }
    fn remove_speaker_request(&mut self, channel: &str, speaker: String) -> io::Result<()> {
        Ok(())
    }
    fn set_flag_request(&mut self, channel: &str, flag: ChannelFlag) -> io::Result<()> {
        self.database.set_channel_mode(channel, flag);

        Ok(())
    }
    fn set_key_request(&mut self, channel: &str, key: String) -> io::Result<()> {
        if let Err(error) = self.assert_can_set_key(channel) {
            return self.stream.send(&error);
        }
        self.database.set_channel_key(channel, Some(key));

        Ok(())
    }
    fn set_limit_request(&mut self, channel: &str, limit: usize) -> io::Result<()> {
        self.database.set_channel_limit(channel, Some(limit));

        Ok(())
    }
    fn unset_limit_request(&mut self, channel: &str) -> io::Result<()> {
        Ok(())
    }
    fn unset_key_request(&mut self, channel: &str) -> io::Result<()> {
        Ok(())
    }
    fn unset_flag_request(&mut self, channel: &str, flag: ChannelFlag) -> io::Result<()> {
        Ok(())
    }
    fn unknown_mode_request(&mut self, character: char) -> io::Result<()> {
        Ok(())
    }
    fn need_argument_request(&mut self, character: char) -> io::Result<()> {
        Ok(())
    }
    fn invalid_argument_request(&mut self, character: char, argument: String) -> io::Result<()> {
        Ok(())
    }
}

pub fn parse_mode_string(mode_string: String, mut mode_arguments: Vec<String>) -> Vec<ModeRequest> {
    let mut add: bool = Default::default();

    let mut requests = Vec::new();
    for char in mode_string.chars() {
        match char {
            ADD_MODE => add = true,
            REMOVE_MODE => add = false,
            char => requests.push(ModeRequest::from(char, add, &mut mode_arguments)),
        }
    }
    requests
}
