use std::io;

use crate::server::consts::{commands::MODE_COMMAND, modes::*};
use crate::server::responses::ErrorReply;
use crate::server::{connection::Connection, connection_handler::client_handler::ClientHandler};

impl<C: Connection> ClientHandler<C> {
    pub(super) fn handle_add_mode(
        &mut self,
        mode: char,
        channel: &str,
        arguments: &mut Vec<String>,
    ) -> Result<(), io::Error> {
        match mode {
            SET_USER_LIMIT => {
                self.set_limit(channel, arguments.pop())?;
            }
            SET_BANMASK => {
                self.add_banmasks(channel, arguments.pop())?;
            }
            SET_SPEAKER => {
                self.add_speakers(channel, arguments.pop())?;
            }
            SET_KEY => {
                self.set_key(channel, arguments.pop())?;
            }
            SET_OPERATOR => self.add_channops(channel, arguments.pop())?,
            mode if VALID_MODES.contains(&mode) => {
                let flag = self.mode_to_flag(mode);
                self.database.set_channel_mode(channel, flag)
            }
            mode => self.stream.send(&ErrorReply::UnknownMode472 { mode })?,
        };
        Ok(())
    }

    pub(super) fn handle_remove_mode(
        &mut self,
        mode: char,
        channel: &str,
        arguments: &mut Vec<String>,
    ) -> Result<(), io::Error> {
        match mode {
            SET_OPERATOR => {
                self.remove_channops(channel, arguments.pop())?;
            }
            SET_BANMASK => {
                self.remove_banmasks(channel, arguments.pop())?;
            }
            SET_SPEAKER => {
                self.remove_speakers(channel, arguments.pop())?;
            }
            SET_USER_LIMIT => self.database.set_channel_limit(channel, None),
            SET_KEY => self.database.set_channel_key(channel, None),
            mode if VALID_MODES.contains(&mode) => self.unset_mode(channel, mode),
            mode => self.stream.send(&ErrorReply::UnknownMode472 { mode })?,
        };
        Ok(())
    }

    fn unset_mode(&mut self, channel: &str, mode: char) {
        let flag = self.mode_to_flag(mode);
        if self.database.channel_has_mode(channel, &flag) {
            self.database.unset_channel_mode(channel, flag)
        }
    }

    fn add_channops(&mut self, channel: &str, operator: Option<String>) -> io::Result<()> {
        if let Err(error) = self.assert_has_enough_params(&operator, MODE_COMMAND) {
            return self.stream.send(&error);
        }

        let operator = operator.unwrap();

        if let Err(error) = self.assert_can_modify_client_status_in_channel(channel, &operator) {
            return self.stream.send(&error);
        }
        self.database.add_channop(channel, &operator);

        Ok(())
    }

    fn remove_channops(&mut self, channel: &str, operator: Option<String>) -> io::Result<()> {
        if let Err(error) = self.assert_has_enough_params(&operator, MODE_COMMAND) {
            return self.stream.send(&error);
        }

        let operator = operator.unwrap();

        if let Err(error) = self.assert_can_modify_client_status_in_channel(channel, &operator) {
            return self.stream.send(&error);
        }

        self.database.remove_channop(channel, &operator);

        Ok(())
    }

    fn set_limit(&mut self, channel: &str, limit: Option<String>) -> io::Result<()> {
        if let Err(error) = self.assert_has_enough_params(&limit, MODE_COMMAND) {
            return self.stream.send(&error);
        }

        if let Ok(limit) = limit.unwrap().parse::<usize>() {
            self.database.set_channel_limit(channel, Some(limit));
        }
        Ok(())
    }

    fn add_banmasks(&mut self, channel: &str, banmasks: Option<String>) -> io::Result<()> {
        if banmasks.is_none() {
            return self.send_banlist_response(channel);
        }
        self.database
            .add_channel_banmask(channel, &banmasks.unwrap());

        Ok(())
    }

    fn remove_banmasks(&mut self, channel: &str, banmask: Option<String>) -> io::Result<()> {
        if let Err(error) = self.assert_has_enough_params(&banmask, MODE_COMMAND) {
            return self.stream.send(&error);
        }

        self.database
            .remove_channel_banmask(channel, &banmask.unwrap());
        Ok(())
    }

    fn add_speakers(&mut self, channel: &str, speaker: Option<String>) -> io::Result<()> {
        if let Err(error) = self.assert_has_enough_params(&speaker, MODE_COMMAND) {
            return self.stream.send(&error);
        }

        let speaker = speaker.unwrap();

        if let Err(error) = self.assert_can_modify_client_status_in_channel(channel, &speaker) {
            return self.stream.send(&error);
        }
        self.database.add_speaker(channel, &speaker);

        Ok(())
    }

    fn remove_speakers(&mut self, channel: &str, speaker: Option<String>) -> io::Result<()> {
        if let Err(error) = self.assert_has_enough_params(&speaker, MODE_COMMAND) {
            return self.stream.send(&error);
        }

        let speaker = speaker.unwrap();

        if let Err(error) = self.assert_can_modify_client_status_in_channel(channel, &speaker) {
            return self.stream.send(&error);
        }

        self.database.remove_speaker(channel, &speaker);

        Ok(())
    }

    fn set_key(&mut self, channel: &str, key: Option<String>) -> io::Result<()> {
        if let Err(error) = self.assert_has_enough_params(&key, MODE_COMMAND) {
            return self.stream.send(&error);
        }

        if let Err(error) = self.assert_can_set_key(channel) {
            return self.stream.send(&error);
        }
        self.database.set_channel_key(channel, key);

        Ok(())
    }
    fn mode_to_flag(&self, mode: char) -> ChannelFlag {
        match mode {
            PRIVATE => ChannelFlag::Private,
            SECRET => ChannelFlag::Secret,
            INVITE_ONLY => ChannelFlag::InviteOnly,
            TOPIC_SETTABLE => ChannelFlag::TopicByOperatorOnly,
            NO_OUTSIDE_MESSAGES => ChannelFlag::NoOutsideMessages,
            MODERATED => ChannelFlag::Moderated,
            _ => panic!("Invalid mode"),
        }
    }
}
