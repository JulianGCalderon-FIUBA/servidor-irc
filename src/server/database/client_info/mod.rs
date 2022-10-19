mod client_info_builder;

pub use client_info_builder::ClientInfoBuilder;
use std::net::TcpStream;

pub struct ClientInfo {
    pub stream: Option<TcpStream>,
    pub password: Option<String>,
    pub nickname: String,
    pub username: String,
    pub hostname: String,
    pub servername: String,
    pub realname: String,
    pub operator: bool,
}

impl ClientInfo {
    pub fn _set_operator(&mut self) {
        todo!()
    }

    pub fn _disconnect(&mut self) {
        todo!()
    }

    pub fn _get_stream(&self) {
        todo!()
    }
}
