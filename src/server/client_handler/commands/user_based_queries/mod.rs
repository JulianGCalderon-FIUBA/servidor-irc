use std::io;
mod validations;

use crate::server::{client_handler::ClientHandler, client_trait::ClientTrait};

pub const WHOIS_COMMAND: &str = "WHOIS";

impl<T: ClientTrait> ClientHandler<T> {
    pub fn whois_command(&mut self, parameters: Vec<String>) -> io::Result<()> {
        if let Some(error) = self.assert_whois_is_valid(&parameters) {
            return self.send_response_for_error(error);
        }
        Ok(())
    }
}
