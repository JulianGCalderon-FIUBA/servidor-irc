mod client_builder;
mod client_info;

pub use client_builder::ClientBuilder;
pub use client_info::ClientInfo;
use std::io;

use crate::server::connection::Connection;

/// Represents a Client that is connected to the Server.
pub struct Client<C: Connection> {
    stream: Option<C>,
    _password: Option<String>,
    nicknames: Vec<String>,
    username: String,
    hostname: String,
    servername: String,
    realname: String,
    operator: bool,
    away_message: Option<String>,
    online: bool,
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
    pub fn get_stream(&self) -> Option<io::Result<C>> {
        Some(self.stream.as_ref()?.try_clone())
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
            hopcount: 1,
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

    pub fn matches_banmask(&self, query: &str) -> bool {
        if matches(&self.nickname(), query) {
            return true;
        }

        if matches(&self.username, query) {
            return true;
        }
        if matches(&self.hostname, query) {
            return true;
        }

        false
    }

    pub fn matches_mask(&self, query: &str) -> bool {
        if matches(&self.nickname(), query) {
            return true;
        }
        if matches(&self.username, query) {
            return true;
        }
        if matches(&self.hostname, query) {
            return true;
        }
        if matches(&self.realname, query) {
            return true;
        }
        if matches(&self.servername, query) {
            return true;
        }

        false
    }

    pub fn matches_nickmask(&self, query: &str) -> bool {
        matches(&self.nickname(), query)
    }

    pub fn disconnect(&mut self) {
        self.online = false
    }
}

/// Checks for pattern in base and returns true if it matches in some way.
pub fn matches(base: &str, pattern: &str) -> bool {
    if pattern.is_empty() {
        return base.is_empty();
    }

    let base = base.as_bytes();
    let pattern = pattern.as_bytes();

    let mut base_index = 0;
    let mut pattern_index = 0;
    let mut glob_base_index = -1;
    let mut glob_pattern_index = -1;

    while base_index < base.len() {
        if pattern_index < pattern.len() {
            if base[base_index] == pattern[pattern_index] || pattern[pattern_index] == b'?' {
                base_index += 1;
                pattern_index += 1;
                continue;
            }

            if pattern[pattern_index] == b'*' {
                glob_base_index = base_index as isize;
                glob_pattern_index = pattern_index as isize;
                pattern_index += 1;
                continue;
            }
        }

        if glob_pattern_index != -1 {
            base_index = (glob_base_index + 1) as usize;
            pattern_index = (glob_pattern_index + 1) as usize;
            glob_base_index += 1;
            continue;
        }

        return false;
    }

    while pattern_index < pattern.len() && pattern[pattern_index] == b'*' {
        pattern_index += 1;
    }

    pattern_index == pattern.len()
}
