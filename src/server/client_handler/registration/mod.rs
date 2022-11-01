use std::collections::HashMap;

mod registration_state;

use crate::server::{
    client_trait::ClientTrait,
    database::{Client, ClientBuilder},
};
pub use registration_state::RegistrationState;

/// Holds a Clients' relevant information.
pub struct Registration<T: ClientTrait> {
    stream: Option<T>,
    nickname: Option<String>,
    state: RegistrationState,
    attributes: HashMap<&'static str, String>,
}

impl<T: ClientTrait> Registration<T> {
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
        self.attributes.get(key).map(|attr| attr.to_owned())
    }

    pub fn set_nickname(&mut self, nickname: String) {
        self.nickname = Some(nickname);

        if self.state != RegistrationState::Registered {
            self.state = RegistrationState::NicknameSent;
        }
    }

    pub fn nickname(&self) -> Option<String> {
        self.nickname.clone()
    }

    pub fn state(&self) -> &RegistrationState {
        &self.state
    }

    pub fn build(&mut self) -> Option<Client<T>> {
        let client = ClientBuilder::new()
            .nickname(self.nickname()?)
            .password(self.get_attribute("password"))
            .username(self.get_attribute("username")?)
            .hostname(self.get_attribute("hostname")?)
            .servername(self.get_attribute("servername")?)
            .realname(self.get_attribute("realname")?)
            .stream(self.stream.take()?)
            .build();

        self.state = RegistrationState::Registered;

        client
    }
}
