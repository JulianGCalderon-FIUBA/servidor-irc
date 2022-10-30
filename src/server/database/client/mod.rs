mod client_builder;

pub use client_builder::ClientBuilder;
use std::sync::{Arc, Mutex};

use crate::server::client_trait::ClientTrait;

pub struct Client<T: ClientTrait> {
    stream: Option<Arc<Mutex<T>>>,
    password: Option<String>,
    nickname: String,
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

    pub fn _is_server_operator(&mut self) -> bool {
        self.operator
    }

    pub fn get_stream(&self) -> Option<Arc<Mutex<T>>> {
        let stream = self.stream.as_ref()?;

        Some(Arc::clone(stream))
    }

    pub fn disconnect(&mut self) {
        self.stream = None;
    }

    pub fn password(&self) -> Option<String> {
        self.password.clone()
    }

    pub fn nickname(&self) -> String {
        self.nickname.clone()
    }

    pub fn realname(&self) -> String {
        self.realname.clone()
    }

    pub fn username(&self) -> String {
        self.username.clone()
    }

    pub fn hostname(&self) -> String {
        self.hostname.clone()
    }

    pub fn servername(&self) -> String {
        self.servername.clone()
    }
}
