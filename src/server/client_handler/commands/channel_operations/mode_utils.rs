use crate::server::client_handler::commands::MODE_COMMAND;

use self::validations::{ADD_MODE, REMOVE_MODE};
use super::*;

pub fn parse_modes(modes: Vec<char>) -> (Vec<char>, Vec<char>) {
    let mut add_modes: Vec<char> = vec![];
    let mut remove_modes: Vec<char> = vec![];
    let mut add: bool = false;
    for char in modes {
        match char {
            ADD_MODE => {
                add = true;
            }
            REMOVE_MODE => {
                add = false;
            }
            char => {
                if add {
                    add_modes.push(char);
                } else {
                    remove_modes.push(char);
                }
            }
        }
    }
    (add_modes, remove_modes)
}

impl<C: Connection> ClientHandler<C> {
    pub fn remove_modes(
        &mut self,
        remove: Vec<char>,
        parameters: Vec<String>,
    ) -> Result<(), io::Error> {
        let channel = &parameters[0];
        let argument = parameters.get(2);

        for mode in remove {
            if !self.database.channel_has_mode(channel, mode) {
                continue;
            }
            match mode {
                OPER_CONFIG => {
                    self.remove_channop(channel, argument)?;
                }
                LIMIT_CONFIG => self.database.set_channel_limit(channel, None),
                BAN_CONFIG => {
                    self.remove_banmask(channel, argument)?;
                }
                SPEAKING_ABILITY_CONFIG => {
                    self.remove_speaker(channel, argument)?;
                }
                KEY_CONFIG => self.database.set_channel_key(channel, None),
                mode if VALID_MODES.contains(&mode) => {
                    self.database.unset_channel_mode(channel, mode)
                }
                mode => self.send_response_for_error(ErrorReply::UnknownMode472 { mode })?,
            }
        }
        Ok(())
    }

    pub fn add_modes(&mut self, add: Vec<char>, parameters: Vec<String>) -> Result<(), io::Error> {
        let channel = &parameters[0];
        let argument = parameters.get(2);

        for mode in add {
            match mode {
                OPER_CONFIG => self.add_channop(channel, argument)?,
                LIMIT_CONFIG => {
                    self.set_limit(channel, argument)?;
                }
                BAN_CONFIG => {
                    if parameters.len() >= 3 {
                        self.set_banmask(channel, argument)
                    } else {
                        self.send_ban_reply(channel)?;
                    }
                }
                SPEAKING_ABILITY_CONFIG => {
                    self.add_speaker(channel, argument)?;
                }
                KEY_CONFIG => {
                    self.set_key(channel, argument)?;
                }
                mode if VALID_MODES.contains(&mode) => {
                    self.database.set_channel_mode(channel, mode)
                }
                mode => self.send_response_for_error(ErrorReply::UnknownMode472 { mode })?,
            }
        }
        Ok(())
    }

    fn send_ban_reply(&mut self, channel: &String) -> io::Result<()> {
        let bans = self.database.get_channel_banmask(channel);
        for b in bans {
            self.send_response_for_reply(CommandResponse::BanList367 {
                channel: channel.to_string(),
                banmask: b,
            })?;
        }
        self.send_response_for_reply(CommandResponse::EndOfBanList368 {
            channel: channel.to_string(),
        })?;
        Ok(())
    }

    pub fn add_channop(&mut self, channel: &str, operators: Option<&String>) -> io::Result<()> {
        let operators = match operators {
            Some(operators) => operators,
            None => {
                return self.send_response_for_error(ErrorReply::NeedMoreParameters461 {
                    command: MODE_COMMAND.to_string(),
                })
            }
        };
        for (i, nickname) in operators.split(',').enumerate() {
            if i == 3 {
                break;
            }
            self.database.add_channop(channel, nickname);
        }
        Ok(())
    }

    pub fn remove_channop(&mut self, channel: &str, operators: Option<&String>) -> io::Result<()> {
        let operators = match operators {
            Some(operators) => operators,
            None => {
                return self.send_response_for_error(ErrorReply::NeedMoreParameters461 {
                    command: MODE_COMMAND.to_string(),
                })
            }
        };
        for (i, nickname) in operators.split(',').enumerate() {
            if i == 3 {
                break;
            }
            self.database.remove_channop(channel, nickname);
        }
        Ok(())
    }

    pub fn set_limit(&mut self, channel: &str, limit: Option<&String>) -> io::Result<()> {
        let limit = match limit {
            Some(limit) => limit,
            None => {
                return self.send_response_for_error(ErrorReply::NeedMoreParameters461 {
                    command: MODE_COMMAND.to_string(),
                })
            }
        };

        if let Ok(limit) = limit.parse::<isize>() {
            self.database.set_channel_limit(channel, Some(limit));
        }
        Ok(())
    }

    pub fn set_banmask(&mut self, channel: &str, banmasks: Option<&String>) {
        let masks = banmasks.unwrap().split(',');
        for (i, banmask) in masks.enumerate() {
            if i == 3 {
                break;
            }
            self.database.set_channel_banmask(channel, banmask)
        }
    }

    pub fn remove_banmask(&mut self, channel: &str, banmasks: Option<&String>) -> io::Result<()> {
        let banmasks = match banmasks {
            Some(banmasks) => banmasks,
            None => {
                return self.send_response_for_error(ErrorReply::NeedMoreParameters461 {
                    command: MODE_COMMAND.to_string(),
                })
            }
        };
        for banmask in banmasks.split(',') {
            self.database.unset_channel_banmask(channel, banmask)
        }
        Ok(())
    }

    pub fn add_speaker(&mut self, channel: &str, speakers: Option<&String>) -> io::Result<()> {
        let speakers = match speakers {
            Some(speakers) => speakers,
            None => {
                return self.send_response_for_error(ErrorReply::NeedMoreParameters461 {
                    command: MODE_COMMAND.to_string(),
                })
            }
        };

        for nickname in speakers.split(',') {
            self.database.add_speaker(channel, nickname);
        }
        Ok(())
    }

    pub fn remove_speaker(&mut self, channel: &str, speakers: Option<&String>) -> io::Result<()> {
        let speakers = match speakers {
            Some(speakers) => speakers,
            None => {
                return self.send_response_for_error(ErrorReply::NeedMoreParameters461 {
                    command: MODE_COMMAND.to_string(),
                });
            }
        };
        for nickname in speakers.split(',') {
            self.database.remove_speaker(channel, nickname);
        }
        Ok(())
    }

    pub fn set_key(&mut self, channel: &str, key: Option<&String>) -> io::Result<()> {
        let key = match key {
            Some(key) => key,
            None => {
                return self.send_response_for_error(ErrorReply::NeedMoreParameters461 {
                    command: MODE_COMMAND.to_string(),
                });
            }
        };

        if let Some(error) = self.assert_can_set_key(channel) {
            return self.send_response_for_error(error);
        }
        self.database
            .set_channel_key(channel, Some(key.to_string()));

        Ok(())
    }
}
