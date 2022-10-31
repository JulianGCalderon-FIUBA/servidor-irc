use std::io;
mod utils;
mod validations;

use crate::server::client_handler::responses::replies::CommandResponse;
use crate::server::client_handler::{responses::errors::ErrorReply, ClientHandler};
use crate::server::client_trait::ClientTrait;

pub const WHOIS_COMMAND: &str = "WHOIS";
pub const WHO_COMMAND: &str = "WHO";

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
            let client_info = self.database.get_client_info(nick).unwrap();
            let nickname = client_info.nickname.clone();

            self.send_response_for_reply(CommandResponse::WhoisUser311 { client_info })?;
            if self.database._is_server_operator(nick) {
                self.send_response_for_reply(CommandResponse::WhoisOperator313 { nickname })?;
            }

            // self.send_response_for_reply(CommandResponse::WhoisIdle317 {
            //     nickname: nick.to_string(),
            //     seconds: (),
            // })?;
            let channels = self.database.get_channels_for_client(nick);
            if !channels.is_empty() {
                self.send_response_for_reply(CommandResponse::WhoisChannels319 {
                    nickname: nick.to_string(),
                    channels,
                })?;
            }

            self.send_response_for_reply(CommandResponse::EndOfWhois318 {
                nickname: nick.to_string(),
            })?;
        }

        Ok(())
    }

    pub fn who_command(&mut self, parameters: Vec<String>) -> io::Result<()> {
        if let Some(error) = self.assert_who_is_valid(&parameters) {
            return self.send_response_for_error(error);
        }

        let mut clients = if parameters.is_empty() {
            self.filtered_clients_for_default_who_command(self.database.get_all_clients())
        } else {
            self.database.get_clients_for_query(&parameters[0])
        };

        clients.sort();

        for client_info in clients {
            self.send_whoreply_for_client(client_info)?;
        }

        let name = parameters.get(0).map(|string| string.to_owned());
        self.send_response_for_reply(CommandResponse::EndOfWho315 { name })
    }
}
