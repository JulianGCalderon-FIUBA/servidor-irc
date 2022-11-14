mod client_builder;
mod client_info;

pub use client_builder::ClientBuilder;
pub use client_info::ClientInfo;
use std::io;

use crate::server::client_trait::Connection;

/// Represents a Client that is connected to the Server.
pub struct Client<C: Connection> {
    stream: C,
    _password: Option<String>,
    nicknames: Vec<String>,
    username: String,
    hostname: String,
    servername: String,
    realname: String,
    operator: bool,
    away_message: Option<String>,
}

impl<C: Connection> Client<C> {
    /// Sets Client as server operator.
    pub fn set_server_operator(&mut self) {
        self.operator = true;
    }

    /// Returns true if Client is server operator.
    pub fn is_server_operator(&mut self) -> bool {
        self.operator
    }

    /// Gets stream for Client. Returns error if cannot clone stream.
    pub fn get_stream(&self) -> io::Result<C> {
        self.stream.try_clone()
    }

    /// Returns ClientInfo with relevant information.
    pub fn get_info(&self) -> ClientInfo {
        ClientInfo {
            nickname: self.nickname(),
            username: self.username.clone(),
            hostname: self.hostname.clone(),
            servername: self.servername.clone(),
            realname: self.realname.clone(),
            operator: self.operator,
        }
    }

    /// Updates nickname.
    pub fn update_nickname(&mut self, nickname: String) {
        self.nicknames.push(nickname);
    }

    /// Returns current nickname.
    pub fn nickname(&self) -> String {
        self.nicknames.last().unwrap().to_string()
    }

    /// Returns true if Client has or had received nickname.
    pub fn had_nickname(&self, nickname: &str) -> bool {
        self.nicknames.contains(&nickname.to_string())
    }

    pub fn set_away_message(&mut self, message: Option<String>) {
        self.away_message = message;
    }

    pub fn away_message(&self) -> Option<String> {
        self.away_message.clone()
    }
}
