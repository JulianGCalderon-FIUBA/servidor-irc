use std::io;

use crate::server::{
    connection::Connection,
    connection_handler::{
        client_handler::ClientHandler, connection_handler_trait::ConnectionHandlerUtils,
        consts::commands::MODE_COMMAND, responses::ErrorReply,
    },
};

impl<C: Connection> ClientHandler<C> {
    pub(super) fn add_channops(
        &mut self,
        channel: &str,
        operators: Option<&String>,
    ) -> io::Result<()> {
        let operators = match operators {
            Some(operators) => operators,
            None => {
                return self.send_response(&ErrorReply::NeedMoreParameters461 {
                    command: MODE_COMMAND.to_string(),
                })
            }
        };
        for (i, nickname) in operators.split(',').enumerate() {
            if i == 3 {
                break;
            }
            if let Err(error) = self.assert_can_modify_client_status_in_channel(channel, nickname) {
                self.send_response(&error)?;
                continue;
            }
            self.database.add_channop(channel, nickname);
        }
        Ok(())
    }

    pub(super) fn remove_channops(
        &mut self,
        channel: &str,
        operators: Option<&String>,
    ) -> io::Result<()> {
        let operators = match operators {
            Some(operators) => operators,
            None => {
                return self.send_response(&ErrorReply::NeedMoreParameters461 {
                    command: MODE_COMMAND.to_string(),
                })
            }
        };
        for (i, nickname) in operators.split(',').enumerate() {
            if i == 3 {
                break;
            }
            if let Err(error) = self.assert_can_modify_client_status_in_channel(channel, nickname) {
                self.send_response(&error)?;
                continue;
            }
            if !self.database.is_channel_operator(channel, nickname) {
                continue;
            }
            self.database.remove_channop(channel, nickname);
        }
        Ok(())
    }

    pub(super) fn set_limit(&mut self, channel: &str, limit: Option<&String>) -> io::Result<()> {
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

    pub(super) fn add_banmasks(
        &mut self,
        channel: &str,
        banmasks: Option<&String>,
    ) -> io::Result<()> {
        if banmasks.is_none() {
            return self.send_banlist_response(channel);
        }
        let masks = banmasks.unwrap().split(',');
        for (i, banmask) in masks.enumerate() {
            if i == 3 {
                break;
            }
            self.database.set_channel_banmask(channel, banmask)
        }
        Ok(())
    }

    pub(super) fn remove_banmasks(
        &mut self,
        channel: &str,
        banmasks: Option<&String>,
    ) -> io::Result<()> {
        let banmasks = match banmasks {
            Some(banmasks) => banmasks,
            None => {
                return self.send_response(&ErrorReply::NeedMoreParameters461 {
                    command: MODE_COMMAND.to_string(),
                })
            }
        };
        for banmask in banmasks.split(',') {
            self.database.unset_channel_banmask(channel, banmask)
        }
        Ok(())
    }

    pub(super) fn add_speakers(
        &mut self,
        channel: &str,
        speakers: Option<&String>,
    ) -> io::Result<()> {
        let speakers = match speakers {
            Some(speakers) => speakers,
            None => {
                return self.send_response(&ErrorReply::NeedMoreParameters461 {
                    command: MODE_COMMAND.to_string(),
                })
            }
        };

        for nickname in speakers.split(',') {
            if let Err(error) = self.assert_can_modify_client_status_in_channel(channel, nickname) {
                self.send_response(&error)?;
                continue;
            }
            self.database.add_speaker(channel, nickname);
        }
        Ok(())
    }

    pub(super) fn remove_speakers(
        &mut self,
        channel: &str,
        speakers: Option<&String>,
    ) -> io::Result<()> {
        let speakers = match speakers {
            Some(speakers) => speakers,
            None => {
                return self.send_response(&ErrorReply::NeedMoreParameters461 {
                    command: MODE_COMMAND.to_string(),
                });
            }
        };
        for nickname in speakers.split(',') {
            if let Err(error) = self.assert_can_modify_client_status_in_channel(channel, nickname) {
                self.send_response(&error)?;
                continue;
            }
            if !self.database.is_channel_speaker(channel, nickname) {
                continue;
            }
            self.database.remove_speaker(channel, nickname);
        }
        Ok(())
    }

    pub(super) fn set_key(&mut self, channel: &str, key: Option<&String>) -> io::Result<()> {
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
        self.database
            .set_channel_key(channel, Some(key.to_string()));

        Ok(())
    }
}
