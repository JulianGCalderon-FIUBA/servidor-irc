mod client_builder;
mod client_info;

pub use client_builder::ClientBuilder;
pub use client_info::ClientInfo;
use std::io;

use crate::server::client_trait::ClientTrait;

pub struct Client<T: ClientTrait> {
    stream: T,
    _password: Option<String>,
    nickname: String,
    username: String,
    hostname: String,
    servername: String,
    realname: String,
    operator: bool,
    online: bool,
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
            nickname: self.nickname.clone(),
            username: self.username.clone(),
            hostname: self.hostname.clone(),
            servername: self.servername.clone(),
            realname: self.realname.clone(),
            operator: self.operator,
        }
    }

    pub fn disconnect(&mut self) {
        self.online = false;
    }

    pub fn is_online(&self) -> bool {
        self.online
    }

    pub fn _password(&mut self) -> Option<String> {
        self._password.clone()
    }
}
