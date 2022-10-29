use std::io;
mod validations;

use crate::server::{
    client_handler::{
        responses::{errors::ErrorReply, replies::CommandResponse},
        ClientHandler,
    },
    client_trait::ClientTrait,
};

pub const WHOIS_COMMAND: &str = "WHOIS";

impl<T: ClientTrait> ClientHandler<T> {
    pub fn whois_command(&mut self, parameters: Vec<String>) -> io::Result<()> {
        if let Some(error) = self.assert_whois_is_valid(&parameters) {
            return self.send_response_for_error(error);
        }
        let mut nickmasks = &parameters[0];
        if parameters.len() == 2 {
            //let server = &parameters[0];
            nickmasks = &parameters[1];
        }

        for nick in nickmasks.split(',') {
            if !self.database.contains_client(nick) {
                self.send_response_for_error(ErrorReply::NoSuchNickname401 {
                    nickname: nick.to_string(),
                })?;
                continue;
            }
            match self.database.get_client_info(nick) {
                Some(info) => {
                    self.send_response_for_reply(CommandResponse::WhoisUser311 {
                        nickname: info.nickname(),
                        username: info.username(),
                        host: info.hostname(),
                        real_name: info.realname(),
                    })?;
                    if self.database._is_server_operator(nick) {
                        self.send_response_for_reply(CommandResponse::WhoisOperator313 {
                            nickname: info.nickname(),
                        })?;
                    }
                }
                None => {
                    eprintln!("Error consiguiendo info del cliente")
                }
            }

            // self.send_response_for_reply(CommandResponse::WhoisIdle317 {
            //     nickname: nick.to_string(),
            //     seconds: (),
            // })?;
            self.send_response_for_reply(CommandResponse::WhoisChannels319 {
                nickname: nick.to_string(),
                channels: self.database.get_channels_for_client(nick),
            })?;
            self.send_response_for_reply(CommandResponse::EndOfWhois318 {
                nickname: nick.to_string(),
            })?;
        }

        Ok(())
    }
}
