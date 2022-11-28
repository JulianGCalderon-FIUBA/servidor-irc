use std::io;

use crate::server::{
    connection::Connection,
    connection_handler::client_handler::ClientHandler,
    consts::{
        commands::MODE_COMMAND,
        modes::{ChannelFlag, UserFlag, ADD_MODE, REMOVE_MODE},
    },
    responses::ErrorReply,
};

mod channel_mode_request;
mod user_mode_request;
pub use channel_mode_request::ChannelModeRequest;

use self::user_mode_request::UserModeRequest;

impl<C: Connection> ClientHandler<C> {
    pub(super) fn handle_channel_mode_request(
        &mut self,
        channel: &str,
        request: ChannelModeRequest,
    ) -> io::Result<()> {
        match request {
            ChannelModeRequest::AddBanmask(banmask) => self.add_banmask_request(channel, banmask),
            ChannelModeRequest::AddOperator(operator) => {
                self.add_operator_request(channel, operator)
            }
            ChannelModeRequest::AddSpeaker(speaker) => self.add_speaker_request(channel, speaker),
            ChannelModeRequest::RemoveBanmask(banmask) => {
                self.remove_banmask_request(channel, banmask)
            }
            ChannelModeRequest::RemoveOperator(operator) => {
                self.remove_operator_request(channel, operator)
            }
            ChannelModeRequest::RemoveSpeaker(speaker) => {
                self.remove_speaker_request(channel, speaker)
            }
            ChannelModeRequest::SetFlag(flag) => self.set_flag_request(channel, flag),
            ChannelModeRequest::SetKey(key) => self.set_key_request(channel, key),
            ChannelModeRequest::SetLimit(limit) => self.set_limit_request(channel, limit),
            ChannelModeRequest::UnsetLimit() => self.unset_limit_request(channel),
            ChannelModeRequest::UnsetKey() => self.unset_key_request(channel),
            ChannelModeRequest::UnsetFlag(flag) => self.unset_flag_request(channel, flag),

            ChannelModeRequest::UnknownMode(character) => self.unknown_mode_request(character),
            ChannelModeRequest::NeedArgument(character) => self.need_argument_request(character),
            ChannelModeRequest::InvalidArgument(character, argument) => {
                self.invalid_argument_request(character, argument)
            }
            ChannelModeRequest::GetBanmasks => self.get_banmasks_request(channel),
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
        self.database.remove_channel_banmask(channel, &banmask);
        Ok(())
    }
    fn remove_operator_request(&mut self, channel: &str, operator: String) -> io::Result<()> {
        if let Err(error) = self.assert_can_modify_client_status_in_channel(channel, &operator) {
            return self.stream.send(&error);
        }
        self.database.remove_channop(channel, &operator);
        Ok(())
    }
    fn remove_speaker_request(&mut self, channel: &str, speaker: String) -> io::Result<()> {
        if let Err(error) = self.assert_can_modify_client_status_in_channel(channel, &speaker) {
            return self.stream.send(&error);
        }
        self.database.remove_speaker(channel, &speaker);
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
        self.database.set_channel_limit(channel, None);
        Ok(())
    }
    fn unset_key_request(&mut self, channel: &str) -> io::Result<()> {
        self.database.set_channel_key(channel, None);
        Ok(())
    }
    fn unset_flag_request(&mut self, channel: &str, flag: ChannelFlag) -> io::Result<()> {
        if self.database.channel_has_mode(channel, &flag) {
            self.database.unset_channel_mode(channel, flag)
        }
        Ok(())
    }
    fn unknown_mode_request(&mut self, character: char) -> io::Result<()> {
        self.stream
            .send(&ErrorReply::UnknownMode472 { mode: character })
    }
    fn need_argument_request(&mut self, _character: char) -> io::Result<()> {
        self.stream.send(&ErrorReply::NeedMoreParameters461 {
            command: MODE_COMMAND.to_string(),
        })
    }
    fn invalid_argument_request(&mut self, _character: char, _argument: String) -> io::Result<()> {
        Ok(())
    }
}

impl<C: Connection> ClientHandler<C> {
    pub(super) fn handle_user_mode_request(
        &mut self,
        user: &str,
        request: UserModeRequest,
    ) -> io::Result<()> {
        match request {
            UserModeRequest::SetFlag(flag) => self.set_flag(flag, user),
            UserModeRequest::UnsetFlag(flag) => self.unset_flag(flag, user),
            UserModeRequest::UnknownRequest(character) => self.unknown_request(character),
        }
    }

    fn set_flag(&mut self, flag: UserFlag, user: &str) -> io::Result<()> {
        self.database.set_user_mode(user, flag);

        Ok(())
    }
    fn unset_flag(&mut self, flag: UserFlag, user: &str) -> io::Result<()> {
        self.database.unset_user_mode(user, flag);

        Ok(())
    }
    fn unknown_request(&mut self, _character: char) -> io::Result<()> {
        let error = ErrorReply::UserModeUnknownFlag501;
        self.stream.send(&error)
    }
}

pub fn parse_channel_mode_string(
    mode_string: String,
    mut mode_arguments: Vec<String>,
) -> Vec<ChannelModeRequest> {
    let mut add: bool = Default::default();

    let mut requests = Vec::new();
    for char in mode_string.chars() {
        match char {
            ADD_MODE => add = true,
            REMOVE_MODE => add = false,
            char => requests.push(ChannelModeRequest::from(char, add, &mut mode_arguments)),
        }
    }
    requests
}

pub fn parse_user_mode_string(mode_string: String) -> Vec<UserModeRequest> {
    let mut add: bool = Default::default();

    let mut requests = Vec::new();
    for char in mode_string.chars() {
        match char {
            ADD_MODE => add = true,
            REMOVE_MODE => add = false,
            char => requests.push(UserModeRequest::from(char, add)),
        }
    }
    requests
}
