mod client_builder;
mod client_info;

pub use client_builder::ClientBuilder;
pub use client_info::ClientInfo;
use std::io;

use crate::server::client_trait::ClientTrait;

pub struct Client<T: ClientTrait> {
    stream: T,
    _password: Option<String>,
    nicknames: Vec<String>,
    username: String,
    hostname: String,
    servername: String,
    realname: String,
    operator: bool,
}

impl<T: ClientTrait> Client<T> {
    pub fn set_server_operator(&mut self) {
        self.operator = true;
    }

    pub fn is_server_operator(&mut self) -> bool {
        self.operator
    }

    pub fn get_stream(&self) -> io::Result<T> {
        self.stream.try_clone()
    }

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

    pub fn update_nickname(&mut self, nickname: String) {
        self.nicknames.push(nickname);
    }

    pub fn nickname(&self) -> String {
        self.nicknames.last().unwrap().to_string()
    }

    pub fn had_nickname(&self, nickname: &str) -> bool {
        self.nicknames.contains(&nickname.to_string())
    }
}
