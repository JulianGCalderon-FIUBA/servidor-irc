use std::io;

use crate::server::{
    connection::Connection,
    connection_handler::{
        client_handler::ClientHandler,
        connection_handler_trait::ConnectionHandlerUtils,
        consts::modes::{
            SET_BANMASK, SET_KEY, SET_OPERATOR, SET_SPEAKER, SET_USER_LIMIT, VALID_MODES,
        },
        responses::{CommandResponse, ErrorReply, Notification},
    },
};

impl<C: Connection> ClientHandler<C> {
    pub(super) fn send_privmsg_to_target(
        &mut self,
        target: &str,
        content: &str,
    ) -> Result<(), std::io::Error> {
        let nickname = self.nickname.clone();
        let notification = Notification::Privmsg {
            sender: nickname,
            target: target.to_string(),
            message: content.to_owned(),
        };

        self.send_message_to_target(&notification, target)?;

        if self.database.contains_client(target) {
            if let Some(message) = self.database.get_away_message(target) {
                let nickname = target.to_string();
                let reply = CommandResponse::Away { nickname, message };
                self.send_response(&reply)?;
            }
        }

        Ok(())
    }

    pub(super) fn send_notice_to_target(
        &mut self,
        target: &str,
        content: &str,
    ) -> Result<(), std::io::Error> {
        let nickname = self.nickname.clone();
        let notification = Notification::Notice {
            sender: nickname,
            target: target.to_string(),
            message: content.to_owned(),
        };

        self.send_message_to_target(&notification, target)?;

        Ok(())
    }

    pub(super) fn kick_client_from_channel(
        &mut self,
        nickname: &str,
        channel: &str,
        comment: &Option<String>,
    ) {
        let notification = Notification::Kick {
            kicker: self.nickname.clone(),
            channel: channel.to_string(),
            kicked: nickname.to_string(),
            comment: comment.clone(),
        };

        self.send_message_to_channel(&notification, channel);
        self.database.remove_client_from_channel(nickname, channel);
    }

    pub(super) fn add_modes(&mut self, add: Vec<char>, parameters: Vec<String>) -> io::Result<()> {
        let channel = &parameters[0];
        let argument = parameters.get(2);

        for mode in add {
            match mode {
                SET_OPERATOR => self.add_channops(channel, argument)?,
                SET_USER_LIMIT => {
                    self.set_limit(channel, argument)?;
                }
                SET_BANMASK => {
                    self.add_banmasks(channel, argument)?;
                }
                SET_SPEAKER => {
                    self.add_speakers(channel, argument)?;
                }
                SET_KEY => {
                    self.set_key(channel, argument)?;
                }
                mode if VALID_MODES.contains(&mode) => {
                    self.database.set_channel_mode(channel, mode as char)
                }
                mode => self.send_response(&ErrorReply::UnknownMode472 { mode })?,
            }
        }

        Ok(())
    }

    pub(super) fn remove_modes(
        &mut self,
        remove: Vec<char>,
        parameters: Vec<String>,
    ) -> io::Result<()> {
        let channel = &parameters[0];
        let argument = parameters.get(2);

        for mode in remove {
            match mode {
                SET_OPERATOR => {
                    self.remove_channops(channel, argument)?;
                }
                SET_USER_LIMIT => self.database.set_channel_limit(channel, None),
                SET_BANMASK => {
                    self.remove_banmasks(channel, argument)?;
                }
                SET_SPEAKER => {
                    self.remove_speakers(channel, argument)?;
                }
                SET_KEY => self.database.set_channel_key(channel, None),
                mode if VALID_MODES.contains(&mode) => {
                    if self.database.channel_has_mode(channel, mode) {
                        self.database.unset_channel_mode(channel, mode)
                    }
                }
                mode => self.send_response(&ErrorReply::UnknownMode472 { mode })?,
            }
        }

        Ok(())
    }
}
