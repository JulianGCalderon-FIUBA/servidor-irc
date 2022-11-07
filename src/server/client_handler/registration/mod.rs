use std::collections::HashMap;

mod registration_state;

use crate::server::client_trait::Connection;
use crate::server::database::{Client, ClientBuilder};
pub use registration_state::RegistrationState;

/// Holds a Clients' relevant information when registering.
pub struct Registration<C: Connection> {
    stream: Option<C>,
    nickname: Option<String>,
    state: RegistrationState,
    attributes: HashMap<&'static str, String>,
}

impl<C: Connection> Registration<C> {
    /// Creates new [`Registration`].
    pub fn with_stream(stream: C) -> Self {
        Self {
            stream: Some(stream),
            nickname: None,
            state: RegistrationState::NotInitialized,
            attributes: HashMap::new(),
        }
    }
    /// Sets a Client's attribute.
    pub fn set_attribute(&mut self, key: &'static str, value: String) {
        self.attributes.insert(key, value);
    }
    /// Gets a Client's attribute.
    pub fn get_attribute(&mut self, key: &'static str) -> Option<String> {
        self.attributes.get(key).map(|attr| attr.to_owned())
    }

    /// Sets Client nickname when registering.
    pub fn set_nickname(&mut self, nickname: String) {
        self.nickname = Some(nickname);

        if self.state != RegistrationState::Registered {
            self.state = RegistrationState::NicknameSent;
        }
    }

    /// Returns nickname used during registration process.
    pub fn nickname(&self) -> Option<String> {
        self.nickname.clone()
    }

    /// Returns current Registration State.
    pub fn state(&self) -> &RegistrationState {
        &self.state
    }

    /// Builds new [`Client`]
    pub fn build(&mut self) -> Option<Client<C>> {
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
