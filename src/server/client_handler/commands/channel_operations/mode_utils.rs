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
                continue;
            }
            REMOVE_MODE => {
                add = false;
                continue;
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

        for mode in remove {
            if !self.database.channel_has_mode(channel, mode) {
                continue;
            }
            match mode {
                OPER_CONFIG => {
                    if let Some(error) = self.remove_channop(parameters.clone()) {
                        self.send_response_for_error(error)?;
                        continue;
                    }
                }
                LIMIT_CONFIG => self.database.set_channel_limit(channel, None),
                BAN_CONFIG => {
                    if let Some(error) = self.remove_banmask(parameters.clone()) {
                        self.send_response_for_error(error)?;
                        continue;
                    }
                }
                SPEAKING_ABILITY_CONFIG => {
                    if let Some(error) = self.remove_speaker(parameters.clone()) {
                        self.send_response_for_error(error)?;
                        continue;
                    }
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

        for mode in add {
            match mode {
                OPER_CONFIG => {
                    if let Some(error) = self.add_channop(parameters.clone()) {
                        self.send_response_for_error(error)?;
                        continue;
                    }
                }
                LIMIT_CONFIG => {
                    if let Some(error) = self.set_limit(parameters.clone()) {
                        self.send_response_for_error(error)?;
                        continue;
                    }
                }
                BAN_CONFIG => {
                    if parameters.len() >= 3 {
                        self.set_banmask(parameters.clone())
                    } else {
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
                    }
                }
                SPEAKING_ABILITY_CONFIG => {
                    if let Some(error) = self.add_speaker(parameters.clone()) {
                        self.send_response_for_error(error)?;
                        continue;
                    }
                }
                KEY_CONFIG => {
                    if let Some(error) = self.set_key(parameters.clone()) {
                        self.send_response_for_error(error)?;
                        continue;
                    }
                }
                mode if VALID_MODES.contains(&mode) => {
                    self.database.set_channel_mode(channel, mode)
                }
                mode => self.send_response_for_error(ErrorReply::UnknownMode472 { mode })?,
            }
        }
        Ok(())
    }

    pub fn add_channop(&mut self, parameters: Vec<String>) -> Option<ErrorReply> {
        if let Some(error) = self.assert_enough_parameters(&parameters) {
            return Some(error);
        }
        let channel = &parameters[0];
        for nickname in parameters[2].split(',') {
            self.database.add_channop(channel, nickname);
        }
        None
    }

    pub fn remove_channop(&mut self, parameters: Vec<String>) -> Option<ErrorReply> {
        if let Some(error) = self.assert_enough_parameters(&parameters) {
            return Some(error);
        }
        let channel = &parameters[0];
        for nickname in parameters[2].split(',') {
            self.database.remove_channop(channel, nickname);
        }
        None
    }

    pub fn set_limit(&mut self, parameters: Vec<String>) -> Option<ErrorReply> {
        if let Some(error) = self.assert_enough_parameters(&parameters) {
            return Some(error);
        }

        let channel = &parameters[0];

        if let Ok(limit) = parameters[2].parse::<isize>() {
            self.database.set_channel_limit(channel, Some(limit));
        }
        None
    }

    pub fn set_banmask(&mut self, parameters: Vec<String>) {
        let channel = &parameters[0];
        for banmask in parameters[2].split(',') {
            self.database.set_channel_banmask(channel, banmask)
        }
    }

    pub fn remove_banmask(&mut self, parameters: Vec<String>) -> Option<ErrorReply> {
        if let Some(error) = self.assert_enough_parameters(&parameters) {
            return Some(error);
        }
        let channel = &parameters[0];
        for banmask in parameters[2].split(',') {
            self.database.unset_channel_banmask(channel, banmask)
        }
        None
    }

    pub fn add_speaker(&mut self, parameters: Vec<String>) -> Option<ErrorReply> {
        if let Some(error) = self.assert_enough_parameters(&parameters) {
            return Some(error);
        }
        let channel = &parameters[0];
        for nickname in parameters[2].split(',') {
            self.database.add_speaker(channel, nickname);
        }
        None
    }

    pub fn remove_speaker(&mut self, parameters: Vec<String>) -> Option<ErrorReply> {
        if let Some(error) = self.assert_enough_parameters(&parameters) {
            return Some(error);
        }
        let channel = &parameters[0];
        for nickname in parameters[2].split(',') {
            self.database.remove_speaker(channel, nickname);
        }
        None
    }

    pub fn set_key(&mut self, parameters: Vec<String>) -> Option<ErrorReply> {
        if let Some(error) = self.assert_enough_parameters(&parameters) {
            return Some(error);
        }
        let channel = &parameters[0];
        if let Some(error) = self.assert_can_set_key(channel) {
            return Some(error);
        }
        let key = &*parameters[2];
        self.database
            .set_channel_key(channel, Some(key.to_string()));

        None
    }
}
