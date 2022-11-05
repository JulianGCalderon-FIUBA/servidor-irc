use std::io;
mod utils;
mod validations;

use crate::server::client_handler::responses::replies::CommandResponse;
use crate::server::client_handler::{responses::errors::ErrorReply, ClientHandler};
use crate::server::client_trait::ClientTrait;
use crate::server::database::ClientInfo;

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

        for mask in nickmasks.split(',') {
            let mut clients: Vec<ClientInfo> = self.database.get_clients_for_nickmask(mask);

            if clients.is_empty() {
                self.send_response_for_error(ErrorReply::NoSuchNickname401 {
                    nickname: mask.to_string(),
                })?;
                continue;
            }
            clients.sort();
            for client in clients {
                self.send_whois_responses(client)?;
            }
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
            self.database.get_clients_for_mask(&parameters[0])
        };

        clients.sort();

        for client_info in clients {
            self.send_whoreply_for_client(client_info)?;
        }

        let name = parameters.get(0).map(|string| string.to_owned());
        self.send_response_for_reply(CommandResponse::EndOfWho315 { name })
    }
}