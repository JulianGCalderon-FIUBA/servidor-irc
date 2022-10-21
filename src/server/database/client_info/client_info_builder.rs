use super::ClientInfo;
use std::{
    net::TcpStream,
    sync::{Arc, Mutex},
};

pub struct ClientInfoBuilder {
    stream: Option<Arc<Mutex<TcpStream>>>,
    password: Option<String>,
    nickname: String,
    username: String,
    hostname: String,
    servername: String,
    realname: String,
}

impl ClientInfoBuilder {
    pub fn new_with(
        nickname: String,
        username: String,
        hostname: String,
        servername: String,
        realname: String,
    ) -> Self {
        Self {
            stream: None,
            password: None,
            nickname,
            username,
            hostname,
            servername,
            realname,
        }
    }

    pub fn with_stream(&mut self, stream: Arc<Mutex<TcpStream>>) {
        self.stream = Some(stream);
    }

    pub fn with_password(&mut self, password: String) {
        self.password = Some(password);
    }

    pub fn build(self) -> ClientInfo {
        ClientInfo {
            stream: self.stream,
            password: self.password,
            nickname: self.nickname,
            username: self.username,
            hostname: self.hostname,
            servername: self.servername,
            realname: self.realname,
            operator: false,
            channels: vec![],
        }
    }
}
