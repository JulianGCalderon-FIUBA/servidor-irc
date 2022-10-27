use std::{
    collections::HashMap,
    io::{Read, Write},
};

mod registration_state;

use crate::server::database::{Client, ClientBuilder};
pub use registration_state::RegistrationState;

/// Holds a Clients' relevant information.
pub struct Connection<T: Read + Write> {
    stream: Option<T>,
    nickname: Option<String>,
    state: RegistrationState,
    attributes: HashMap<&'static str, String>,
}

impl<T: Read + Write> Connection<T> {
    pub fn with_stream(stream: T) -> Self {
        Self {
            stream: Some(stream),
            nickname: None,
            state: RegistrationState::NotInitialized,
            attributes: HashMap::new(),
        }
    }

    pub fn set_attribute(&mut self, key: &'static str, value: String) {
        self.attributes.insert(key, value);
    }

    pub fn get_attribute(&mut self, key: &'static str) -> Option<String> {
        self.attributes.get(key).map(|s| s.to_owned())
    }

    pub fn set_nickname(&mut self, nickname: String) {
        self.nickname = Some(nickname);
    }

    pub fn advance_state(&mut self) {
        self.state = self.state.next();
    }

    pub fn nickname(&self) -> Option<String> {
        self.nickname.clone()
    }

    pub fn state(&self) -> &RegistrationState {
        &self.state
    }

    pub fn build(&mut self) -> Option<Client<T>> {
        ClientBuilder::new()
            .stream(self.stream.take()?)
            .nickname(self.nickname()?)
            .password(self.get_attribute("nickname"))
            .username(self.get_attribute("username")?)
            .hostname(self.get_attribute("hostname")?)
            .servername(self.get_attribute("servername")?)
            .realname(self.get_attribute("realname")?)
            .build()
    }
}
