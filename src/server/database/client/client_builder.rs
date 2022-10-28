use crate::server::client_trait::ClientTrait;

use super::Client;
use std::sync::{Arc, Mutex};

pub struct ClientBuilder<T: ClientTrait> {
    stream: Option<T>,
    password: Option<String>,
    nickname: Option<String>,
    username: Option<String>,
    hostname: Option<String>,
    servername: Option<String>,
    realname: Option<String>,
}

impl<T: ClientTrait> ClientBuilder<T> {
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

    pub fn stream(mut self, stream: T) -> Self {
        self.stream = Some(stream);
        self
    }

    pub fn password(mut self, password: Option<String>) -> Self {
        self.password = password;
        self
    }

    pub fn nickname(mut self, nickname: String) -> Self {
        self.nickname = Some(nickname);
        self
    }

    pub fn username(mut self, username: String) -> Self {
        self.username = Some(username);
        self
    }

    pub fn hostname(mut self, hostname: String) -> Self {
        self.hostname = Some(hostname);
        self
    }

    pub fn servername(mut self, servername: String) -> Self {
        self.servername = Some(servername);
        self
    }

    pub fn realname(mut self, realname: String) -> Self {
        self.realname = Some(realname);
        self
    }

    pub fn build(self) -> Option<Client<T>> {
        let stream = Arc::new(Mutex::new(self.stream?));

        let client_info = Client {
            stream: Some(stream),
            password: self.password,
            nickname: self.nickname?,
            _username: self.username?,
            _hostname: self.hostname?,
            _servername: self.servername?,
            realname: self.realname?,
            operator: false,
        };

        Some(client_info)
    }
}
