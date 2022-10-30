use std::io;
mod validations;

use crate::server::client_handler::responses::replies::CommandResponse;
use crate::server::{client_handler::ClientHandler, client_trait::ClientTrait};

pub const WHOIS_COMMAND: &str = "WHOIS";
pub const WHO_COMMAND: &str = "WHO";

impl<T: ClientTrait> ClientHandler<T> {
    pub fn whois_command(&mut self, parameters: Vec<String>) -> io::Result<()> {
        if let Some(error) = self.assert_whois_is_valid(&parameters) {
            return self.send_response_for_error(error);
        }
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
                .filter(|client| {
                    self.database
                        .get_channels_for_client(client)
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

        for client in clients {
            self.send_response_for_reply(CommandResponse::WhoReply352 { client })?;
        }

        let name = parameters.get(0).map(|s| s.to_owned());
        self.send_response_for_reply(CommandResponse::EndOfWho315 { name })
    }
}
