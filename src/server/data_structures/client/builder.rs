use std::collections::HashMap;

use crate::server::connection::Connection;

use super::{ClientInfo, ExternalClient, LocalClient};

pub struct ClientBuilder<C: Connection> {
    immediate: Option<String>,
    stream: Option<C>,
    nickname: Option<String>,
    password: Option<String>,
    username: Option<String>,
    hostname: Option<String>,
    servername: Option<String>,
    realname: Option<String>,
    hopcount: usize,
}

impl<C: Connection> ClientBuilder<C> {
    pub fn new() -> Self {
        Self {
            immediate: None,
            stream: None,
            nickname: None,
            password: None,
            username: None,
            hostname: None,
            servername: None,
            realname: None,
            hopcount: 0,
        }
    }

    pub fn nickname(mut self, nickname: &str) -> Self {
        self.nickname = Some(nickname.to_string());
        self
    }

    pub fn stream(mut self, stream: C) -> Self {
        self.stream = Some(stream);
        self
    }

    pub fn password(mut self, password: Option<&String>) -> Self {
        self.password = password.map(|s| s.to_string());
        self
    }

    pub fn username(mut self, username: &str) -> Self {
        self.username = Some(username.to_string());
        self
    }

    pub fn hostname(mut self, hostname: &str) -> Self {
        self.hostname = Some(hostname.to_string());
        self
    }

    pub fn servername(mut self, servername: &str) -> Self {
        self.servername = Some(servername.to_string());
        self
    }

    pub fn realname(mut self, realname: &str) -> Self {
        self.realname = Some(realname.to_string());
        self
    }

    pub fn hopcount(mut self, hopcount: usize) -> Self {
        self.hopcount = hopcount;
        self
    }

    pub fn immediate(mut self, immediate: &str) -> Self {
        self.immediate = Some(immediate.to_string());
        self
    }

    fn build_info(&mut self) -> Option<ClientInfo> {
        let info = ClientInfo {
            nicknames: vec![self.nickname.take()?],
            username: self.username.take()?,
            hostname: self.hostname.take()?,
            servername: self.servername.take()?,
            realname: self.realname.take()?,
            hopcount: self.hopcount,
            away: None,
            flags: HashMap::new(),
        };
        Some(info)
    }

    pub fn build_local_client(mut self) -> Option<LocalClient<C>> {
        let info = self.build_info()?;

        let client = LocalClient::new(self.stream?, self.password, info);

        Some(client)
    }

    pub fn build_external_client(mut self) -> Option<ExternalClient> {
        let info = self.build_info()?;

        let client = ExternalClient::new(self.immediate?, info);

        Some(client)
    }
}
