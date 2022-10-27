mod client_builder;

pub use client_builder::ClientBuilder;
use std::sync::{Arc, Mutex};

use crate::server::client_trait::ClientTrait;

pub struct Client<T: ClientTrait> {
    pub stream: Option<Arc<Mutex<T>>>,
    pub password: Option<String>,
    pub nickname: String,
    pub username: String,
    pub hostname: String,
    pub servername: String,
    pub realname: String,
    pub operator: bool,
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
}
