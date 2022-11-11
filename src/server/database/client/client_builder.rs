use crate::server::client_trait::Connection;

use super::Client;

/// A ClientBuilder is used to build a Client using a Builder pattern.
pub struct ClientBuilder<C: Connection> {
    stream: Option<C>,
    password: Option<String>,
    nickname: Option<String>,
    username: Option<String>,
    hostname: Option<String>,
    servername: Option<String>,
    realname: Option<String>,
}

impl<C: Connection> ClientBuilder<C> {
    /// Creates empty [`ClientBuilder`].
    pub fn new() -> Self {
        Self {
            stream: None,
            password: None,
            nickname: None,
            username: None,
            hostname: None,
            servername: None,
            realname: None,
        }
    }

    /// Sets stream.
    pub fn stream(mut self, stream: C) -> Self {
        self.stream = Some(stream);
        self
    }

    /// Sets password.
    pub fn password(mut self, password: Option<String>) -> Self {
        self.password = password;
        self
    }

    /// Sets nickname.
    pub fn nickname(mut self, nickname: String) -> Self {
        self.nickname = Some(nickname);
        self
    }

    /// Sets username.
    pub fn username(mut self, username: String) -> Self {
        self.username = Some(username);
        self
    }

    /// Sets hostname.
    pub fn hostname(mut self, hostname: String) -> Self {
        self.hostname = Some(hostname);
        self
    }

    /// Sets servername.
    pub fn servername(mut self, servername: String) -> Self {
        self.servername = Some(servername);
        self
    }

    /// Sets realname.
    pub fn realname(mut self, realname: String) -> Self {
        self.realname = Some(realname);
        self
    }

    /// Builds and returns new [`Client`] with previously received fields.
    pub fn build(self) -> Option<Client<C>> {
        let client_info = Client {
            stream: self.stream?,
            _password: self.password,
            nicknames: vec![self.nickname?],
            username: self.username?,
            hostname: self.hostname?,
            servername: self.servername?,
            realname: self.realname?,
            operator: false,
            away_message: None,
        };

        Some(client_info)
    }
}
