mod validations;

use super::ClientHandler;

use std::io;

pub const INVITE_COMMAND: &str = "INVITE";
pub const JOIN_COMMAND: &str = "JOIN";
pub const LIST_COMMAND: &str = "LIST";
pub const NAMES_COMMAND: &str = "NAMES";
pub const PART_COMMAND: &str = "PART";

impl ClientHandler {
    pub fn invite_command(&mut self, parameters: Vec<String>) -> io::Result<()> {
        if !self.validate_invite_command(&parameters)? {
            self.need_more_params_error(INVITE_COMMAND)?;
            return Ok(());
        }

        let nickname = &parameters[0];
        let channel = &parameters[1];

        if !self.validate_nickname_exits(nickname)? {
            self.no_such_nickname_error(nickname)?;
            return Ok(());
        }

        if self.validate_channel_exists(channel)? {
            if !self.validate_user_is_in_channel(channel)? {
                self.not_on_channel_error(channel)?;
                return Ok(());
            }
            if !self.validate_can_join_channel(channel, nickname)? {
                self.user_on_channel_error(nickname, channel)?;
                return Ok(());
            }
            // si el canal es invite only: validar que el usuario es operador del canal
        }

        self.invite_reply(channel, nickname)
    }

    pub fn join_command(&mut self, parameters: Vec<String>) -> io::Result<()> {
        if !self.validate_join_command(&parameters)? {
            return Ok(());
        }
        let nickname = self.connection.nickname.clone().unwrap();

        let channels = &parameters[0];
        //let keys = &parameters[1];

        for channel in channels.split(',') {
            if !self.validate_can_join_channel(channel, &nickname)? {
                continue;
            }
            self.database.add_client_to_channel(&nickname, channel);
            self.no_topic_reply(channel)?;
            self.names_reply(channel.to_string(), self.database.get_clients(channel))?
        }

        Ok(())
    }

    pub fn list_command(&mut self) -> io::Result<()> {
        if !self.validate_list_command()? {
            return Ok(());
        }

        let channels = self.database.get_channels();

        self.list_reply(channels)
    }

    pub fn names_command(&mut self, mut parameters: Vec<String>) -> io::Result<()> {
        if !self.validate_names_command()? {
            return Ok(());
        }

        if parameters.is_empty() {
            parameters = self.database.get_channels();
        } else {
            parameters = parameters[0]
                .split(',')
                .map(|string| string.to_string())
                .collect();
        }

        for channel in parameters {
            if self.database.contains_channel(&channel) {
                let clients = self.database.get_clients(&channel);
                self.names_reply(channel, clients)?;
            } else {
                self.no_such_channel_error(&channel)?;
            }
        }
        Ok(())
    }

    pub fn part_command(&mut self, parameters: Vec<String>) -> io::Result<()> {
        let nickname = self.connection.nickname.clone().unwrap();
        if !self.validate_part_command(&parameters, &nickname)? {
            return Ok(());
        }

        let channels = &parameters[0];

        for channel in channels.split(',') {
            if !self.validate_channel_exists(channel)? {
                return self.no_such_channel_error(channel);
            }
            let clients = self.database.get_clients(channel);
            if !clients.contains(&nickname.to_string()) {
                return self.not_on_channel_error(channel);
            }
            self.database.remove_client_of_channel(&nickname, channel)
        }
        Ok(())
    }
}
