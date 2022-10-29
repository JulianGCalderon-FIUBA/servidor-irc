use std::io;

use crate::server::{client_handler::ClientHandler, client_trait::ClientTrait};

pub const WHOIS_COMMAND: &str = "WHOIS";

impl<T: ClientTrait> ClientHandler<T> {
    pub fn whois_command(&mut self, parameters: Vec<String>) -> io::Result<()> {
        Ok(())
    }
}
