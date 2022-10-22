use super::connection_info::RegistrationState;
use super::ClientHandler;

use std::io;

mod responses;
mod utils;
mod validations;

pub const PASS_COMMAND: &str = "PASS";
pub const NICK_COMMAND: &str = "NICK";
pub const USER_COMMAND: &str = "USER";
pub const QUIT_COMMAND: &str = "QUIT";
pub const PRIVMSG_COMMAND: &str = "PRIVMSG";
pub const NOTICE_COMMAND: &str = "NOTICE";
pub const PART_COMMAND: &str = "PART";
pub const JOIN_COMMAND: &str = "JOIN";
pub const NAMES_COMMAND: &str = "NAMES";
pub const LIST_COMMAND: &str = "LIST";

impl ClientHandler {
    pub fn pass_command(&mut self, mut parameters: Vec<String>) -> io::Result<()> {
        if !self.validate_pass_command(&parameters)? {
            return Ok(());
        }

        let password = parameters.pop().unwrap();
        self.connection.password = Some(password);

        self.ok_reply()
    }

    pub fn nick_command(&mut self, mut parameters: Vec<String>) -> io::Result<()> {
        if !self.validate_nick_command(&parameters)? {
            return Ok(());
        }

        let nickname = parameters.pop().unwrap();
        self.connection.nickname = Some(nickname);

        if self.connection.registration_state == RegistrationState::NotInitialized {
            self.connection.advance_state();
        }

        self.ok_reply()
    }

    pub fn user_command(
        &mut self,
        mut parameters: Vec<String>,
        trailing: Option<String>,
    ) -> io::Result<()> {
        if !self.validate_user_command(&parameters, &trailing)? {
            return Ok(());
        }

        let realname = trailing.unwrap();
        let username = parameters.pop().unwrap();
        let hostname = parameters.pop().unwrap();
        let servername = parameters.pop().unwrap();

        self.connection.username = Some(username);
        self.connection.hostname = Some(hostname);
        self.connection.servername = Some(servername);
        self.connection.realname = Some(realname);

        self.connection.advance_state();

        let client_info = self.connection.build_client_info().unwrap();
        self.database.add_client(client_info);

        self.ok_reply()
    }

    pub fn quit_command(&mut self, trailing: Option<String>) -> io::Result<()> {
        if let Some(trailing) = trailing {
            return self.quit_reply(&trailing);
        }

        let nickname = self.connection.nickname.clone().unwrap_or_default();
        self.quit_reply(&nickname)
    }

    pub fn privmsg_command(
        &mut self,
        parameters: Vec<String>,
        trailing: Option<String>,
    ) -> io::Result<()> {
        if !self.validate_privmsg_command(&parameters, &trailing)? {
            return Ok(());
        }

        let content = trailing.unwrap();

        let targets = &parameters[0];
        for target in targets.split(',') {
            let message = self.build_text_message(PRIVMSG_COMMAND, target, &content);
            self.send_message_to(target, &message)?;
            if self.database.contains_client(target) {
                // let away = self.database.away_message_from_client(target);
                // if let Some(away) = away {
                //     self.away_reply(target, away)?;
                // }
            }
        }

        Ok(())
    }

    pub fn notice_command(
        &mut self,
        parameters: Vec<String>,
        trailing: Option<String>,
    ) -> io::Result<()> {
        if !self.validate_privmsg_command(&parameters, &trailing)? {
            return Ok(());
        }

        let content = trailing.unwrap();

        let targets = &parameters[0];
        for target in targets.split(',') {
            let message = self.build_text_message(NOTICE_COMMAND, target, &content);
            self.send_message_to(target, &message)?;
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
}
