mod client_info_builder;

pub use client_info_builder::ClientInfoBuilder;
use std::{
    net::TcpStream,
    sync::{Arc, Mutex},
};

pub struct ClientInfo {
    pub stream: Option<Arc<Mutex<TcpStream>>>,
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
        self.operator = true;
    }

    pub fn get_stream(&self) -> Option<Arc<Mutex<TcpStream>>> {
        let stream = self.stream.as_ref()?;

        Some(Arc::clone(stream))
    }

    pub fn disconnect(&mut self) {
        self.stream = None;
    }
}