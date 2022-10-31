use std::io;
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
            self.send_whois_responses(client_info, nick, nickname)?;
        }

        Ok(())
    }

    fn send_whois_responses(
        &mut self,
        client_info: crate::server::database::ClientInfo,
        nick: &str,
        nickname: String,
    ) -> Result<(), io::Error> {
        self.send_response_for_reply(CommandResponse::WhoisUser311 { client_info })?;
        if self.database._is_server_operator(nick) {
            self.send_response_for_reply(CommandResponse::WhoisOperator313 { nickname })?;
        }
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
        Ok(())
    }

    pub fn who_command(&mut self, parameters: Vec<String>) -> io::Result<()> {
        if let Some(error) = self.assert_who_is_valid(&parameters) {
            return self.send_response_for_error(error);
        }

        let mut clients = if parameters.is_empty() {
            let clients = self.database.get_all_clients();

            clients
                .into_iter()
                .filter(|client_info| {
                    self.database
                        .get_channels_for_client(&client_info.nickname)
                        .iter()
                        .all(|channel| {
                            !self
                                .database
                                .get_channels_for_client(&self.registration.nickname().unwrap())
                                .contains(channel)
                        })
                })
                .collect()
        } else {
            self.database.get_clients_for_query(&parameters[0])
        };

        clients.sort();

        for client_info in clients {
            self.send_response_for_reply(CommandResponse::WhoReply352 { client_info })?;
        }

        let name = parameters.get(0).map(|s| s.to_owned());
        self.send_response_for_reply(CommandResponse::EndOfWho315 { name })
    }
}
