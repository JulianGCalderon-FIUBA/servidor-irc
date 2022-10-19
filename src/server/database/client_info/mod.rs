mod client_info_builder;

pub use client_info_builder::ClientInfoBuilder;
use std::{
    net::TcpStream,
    sync::{Arc, Mutex},
};

pub struct ClientInfo {
    pub stream: Arc<Mutex<Option<TcpStream>>>,
    pub password: Option<String>,
    pub nickname: String,
    pub username: String,
    pub hostname: String,
    pub servername: String,
    pub realname: String,
    pub operator: bool,
}

impl ClientInfo {
    pub fn set_server_operator(&mut self) {
        todo!()
    }

    pub fn get_stream(&self, nickname: &str) -> Option<Arc<Mutex<TcpStream>>> {
        todo!()
    }

    pub fn _disconnect(&mut self) {
        todo!()
    }

    pub fn _get_stream(&self) {
        todo!()
    }
}
