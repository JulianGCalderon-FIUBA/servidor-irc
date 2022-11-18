use std::io;

use crate::server::{
    connection::Connection,
    connection_handler::{
        client_handler::ClientHandler,
        connection_handler_trait::ConnectionHandlerUtils,
        consts::{commands::MODE_COMMAND, modes::*},
        responses::ErrorReply,
    },
};

impl<C: Connection> ClientHandler<C> {
    fn add_channops(&mut self, channel: &str, operator: Option<String>) -> io::Result<()> {
        let operator = match operator {
            Some(operator) => operator,
            None => {
                return self.send_response(&ErrorReply::NeedMoreParameters461 {
                    command: MODE_COMMAND.to_string(),
                })
            }
        };

        if let Err(error) = self.assert_can_modify_client_status_in_channel(channel, &operator) {
            return self.send_response(&error);
        }
        self.database.add_channop(channel, &operator);

        Ok(())
    }

    fn remove_channops(&mut self, channel: &str, operator: Option<String>) -> io::Result<()> {
        let operator = match operator {
            Some(operator) => operator,
            None => {
                return self.send_response(&ErrorReply::NeedMoreParameters461 {
                    command: MODE_COMMAND.to_string(),
                })
            }
        };

        if let Err(error) = self.assert_can_modify_client_status_in_channel(channel, &operator) {
            return self.send_response(&error);
        }
        if !self.database.is_channel_operator(channel, &operator) {}
        self.database.remove_channop(channel, &operator);

        Ok(())
    }

    fn set_limit(&mut self, channel: &str, limit: Option<String>) -> io::Result<()> {
        let limit = match limit {
            Some(limit) => limit,
            None => {
                return self.send_response(&ErrorReply::NeedMoreParameters461 {
                    command: MODE_COMMAND.to_string(),
                })
            }
        };

        if let Ok(limit) = limit.parse::<usize>() {
            self.database.set_channel_limit(channel, Some(limit));
        }
        Ok(())
    }

    fn add_banmasks(&mut self, channel: &str, banmasks: Option<String>) -> io::Result<()> {
        if banmasks.is_none() {
            return self.send_banlist_response(channel);
        }
        self.database
            .set_channel_banmask(channel, &banmasks.unwrap());

        Ok(())
    }

    fn remove_banmasks(&mut self, channel: &str, banmask: Option<String>) -> io::Result<()> {
        let banmask = match banmask {
            Some(banmask) => banmask,
            None => {
                return self.send_response(&ErrorReply::NeedMoreParameters461 {
                    command: MODE_COMMAND.to_string(),
                })
            }
        };
        self.database.unset_channel_banmask(channel, &banmask);
        Ok(())
    }

    fn add_speakers(&mut self, channel: &str, speaker: Option<String>) -> io::Result<()> {
        let speaker = match speaker {
            Some(speaker) => speaker,
            None => {
                return self.send_response(&ErrorReply::NeedMoreParameters461 {
                    command: MODE_COMMAND.to_string(),
                })
            }
        };

        if let Err(error) = self.assert_can_modify_client_status_in_channel(channel, &speaker) {
            return self.send_response(&error);
        }
        self.database.add_speaker(channel, &speaker);

        Ok(())
    }

    fn remove_speakers(&mut self, channel: &str, speaker: Option<String>) -> io::Result<()> {
        let speaker = match speaker {
            Some(speaker) => speaker,
            None => {
                return self.send_response(&ErrorReply::NeedMoreParameters461 {
                    command: MODE_COMMAND.to_string(),
                });
            }
        };

        if let Err(error) = self.assert_can_modify_client_status_in_channel(channel, &speaker) {
            return self.send_response(&error);
        }
        if !self.database.is_channel_speaker(channel, &speaker) {}
        self.database.remove_speaker(channel, &speaker);

        Ok(())
    }

    fn set_key(&mut self, channel: &str, key: Option<String>) -> io::Result<()> {
        let key = match key {
            Some(key) => key,
            None => {
                return self.send_response(&ErrorReply::NeedMoreParameters461 {
                    command: MODE_COMMAND.to_string(),
                });
            }
        };

        if let Err(error) = self.assert_can_set_key(channel) {
            return self.send_response(&error);
        }
        self.database.set_channel_key(channel, Some(key));

        Ok(())
    }

    pub(super) fn handle_add_mode(
        &mut self,
        mode: char,
        channel: &str,
        arguments: &mut Vec<String>,
    ) -> Result<(), io::Error> {
        match mode {
            SET_OPERATOR => self.add_channops(channel, arguments.pop())?,
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
            mode if VALID_MODES.contains(&mode) => self.database.set_channel_mode(channel, mode),
            mode => self.send_response(&ErrorReply::UnknownMode472 { mode })?,
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
            SET_USER_LIMIT => self.database.set_channel_limit(channel, None),
            SET_BANMASK => {
                self.remove_banmasks(channel, arguments.pop())?;
            }
            SET_SPEAKER => {
                self.remove_speakers(channel, arguments.pop())?;
            }
            SET_KEY => self.database.set_channel_key(channel, None),
            mode if VALID_MODES.contains(&mode) => {
                if self.database.channel_has_mode(channel, mode) {
                    self.database.unset_channel_mode(channel, mode)
                }
            }
            mode => self.send_response(&ErrorReply::UnknownMode472 { mode })?,
        };
        Ok(())
    }
}
