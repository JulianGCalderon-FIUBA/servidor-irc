use std::io;

use crate::server::consts::{commands::MODE_COMMAND, modes::*};
use crate::server::{connection::Connection, connection_handler::client_handler::ClientHandler};

impl<C: Connection> ClientHandler<C> {
    fn unset_mode(&mut self, channel: &str, mode: char) {
        let flag = ChannelFlag::from_char(mode);
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

    fn remove_banmasks(&mut self, channel: &str, banmask: Option<String>) -> io::Result<()> {
        if let Err(error) = self.assert_has_enough_params(&banmask, MODE_COMMAND) {
            return self.stream.send(&error);
        }

        self.database
            .remove_channel_banmask(channel, &banmask.unwrap());
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
}
