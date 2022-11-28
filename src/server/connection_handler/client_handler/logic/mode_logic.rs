use std::io;

use crate::server::{
    connection::Connection,
    connection_handler::{
        client_handler::ClientHandler,
        connection_handler_trait::ConnectionHandlerUtils,
        mode_requests::{ChannelModeRequest, UserModeRequest},
    },
    consts::{
        commands::MODE_COMMAND,
        modes::{ChannelFlag, UserFlag},
    },
    responses::{ErrorReply, Notification},
};

impl<C: Connection> ClientHandler<C> {
    pub(super) fn handle_channel_mode_request(
        &mut self,
        channel: &str,
        request: ChannelModeRequest,
    ) -> io::Result<()> {
        let result = match request.clone() {
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
            ChannelModeRequest::SetFlag(flag) => self.set_channel_flag_request(channel, flag),
            ChannelModeRequest::SetKey(key) => self.set_key_request(channel, key),
            ChannelModeRequest::SetLimit(limit) => self.set_limit_request(channel, limit),
            ChannelModeRequest::UnsetLimit() => self.unset_limit_request(channel),
            ChannelModeRequest::UnsetKey() => self.unset_key_request(channel),
            ChannelModeRequest::UnsetFlag(flag) => self.unset_channel_flag_request(channel, flag),

            ChannelModeRequest::UnknownMode(character) => {
                return self.unknown_channel_mode_request(character)
            }
            ChannelModeRequest::NeedArgument(character) => {
                return self.need_argument_request(character)
            }
            ChannelModeRequest::InvalidArgument(character, argument) => {
                return self.invalid_argument_request(character, argument)
            }
            ChannelModeRequest::GetBanmasks => return self.get_banmasks_request(channel),
        };

        let sender = &self.nickname;
        let target = channel;
        let request = request.to_string();

        let notification = Notification::mode(sender, target, &request);
        self.send_message_to_local_clients_on_channel(&notification, channel);
        self.send_message_to_all_servers(&notification);

        result
    }

    fn add_banmask_request(&mut self, channel: &str, banmask: String) -> io::Result<()> {
        self.database.add_channel_banmask(channel, &banmask);

        Ok(())
    }

    fn get_banmasks_request(&mut self, channel: &str) -> io::Result<()> {
        self.send_banlist_response(channel)
    }

    fn add_operator_request(&mut self, channel: &str, operator: String) -> io::Result<()> {
        if let Err(error) = self.assert_is_client_in_channel(channel, &operator) {
            return self.stream.send(&error);
        }

        self.database.add_channop(channel, &operator);

        Ok(())
    }

    fn add_speaker_request(&mut self, channel: &str, speaker: String) -> io::Result<()> {
        if let Err(error) = self.assert_is_client_in_channel(channel, &speaker) {
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
        if let Err(error) = self.assert_is_client_in_channel(channel, &operator) {
            return self.stream.send(&error);
        }
        self.database.remove_channop(channel, &operator);
        Ok(())
    }
    fn remove_speaker_request(&mut self, channel: &str, speaker: String) -> io::Result<()> {
        if let Err(error) = self.assert_is_client_in_channel(channel, &speaker) {
            return self.stream.send(&error);
        }
        self.database.remove_speaker(channel, &speaker);
        Ok(())
    }
    fn set_channel_flag_request(&mut self, channel: &str, flag: ChannelFlag) -> io::Result<()> {
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
    fn unset_channel_flag_request(&mut self, channel: &str, flag: ChannelFlag) -> io::Result<()> {
        if self.database.channel_has_mode(channel, &flag) {
            self.database.unset_channel_mode(channel, flag)
        }
        Ok(())
    }
    fn unknown_channel_mode_request(&mut self, character: char) -> io::Result<()> {
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
            UserModeRequest::SetFlag(flag) => self.set_user_flag_request(flag, user),
            UserModeRequest::UnsetFlag(flag) => self.unset_user_flag_request(flag, user),
            UserModeRequest::UnknownRequest(character) => self.unknown_user_mode_request(character),
        }
    }

    fn set_user_flag_request(&mut self, flag: UserFlag, user: &str) -> io::Result<()> {
        self.database.set_user_mode(user, flag);

        Ok(())
    }
    fn unset_user_flag_request(&mut self, flag: UserFlag, user: &str) -> io::Result<()> {
        self.database.unset_user_mode(user, flag);

        Ok(())
    }
    fn unknown_user_mode_request(&mut self, _character: char) -> io::Result<()> {
        let error = ErrorReply::UserModeUnknownFlag501;
        self.stream.send(&error)
    }
}
