use super::connection_info::RegistrationState;
use super::ClientHandler;

use std::io;

mod responses;
mod validations;

pub const PASS_COMMAND: &str = "PASS";
pub const NICK_COMMAND: &str = "NICK";
pub const USER_COMMAND: &str = "USER";
pub const QUIT_COMMAND: &str = "QUIT";
pub const PART_COMMAND: &str = "PART";
pub const JOIN_COMMAND: &str = "JOIN";

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
            let clients = self.database._get_clients(channel);
            if !clients.contains(&nickname.to_string()) {
                return self.not_on_channel_error(channel);
            }
            self.database.remove_client_of_channel(&nickname, channel)
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
            self.no_topic_reply(channel)?
            //self.names_reply(channel, self.database.get_clients(channel))?
        }

        Ok(())
    }
}
