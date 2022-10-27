use std::io::{Read, Write};

mod registration_state;

use crate::server::database::{Client, ClientBuilder};
pub use registration_state::RegistrationState;

/// Holds a Clients' relevant information.
pub struct Connection<T: Read + Write> {
    nickname: Option<String>,
    state: RegistrationState,
    builder: Option<ClientBuilder<T>>,
}

impl<T: Read + Write> Connection<T> {
    pub fn with_stream(stream: T) -> Self {
        Self {
            nickname: None,
            state: RegistrationState::NotInitialized,
            builder: Some(ClientBuilder::new().stream(stream)),
        }
    }

    pub fn build(&mut self) -> Option<Client<T>> {
        self.builder.take()?.build()
    }

    pub fn set_nickname(&mut self, nickname: String) {
        if self.builder.is_some() {
            self.nickname = Some(nickname.clone());
            self.builder = Some(self.builder.take().unwrap().nickname(nickname));
        }
    }

    pub fn set_password(&mut self, password: String) {
        if self.builder.is_some() {
            self.builder = Some(self.builder.take().unwrap().password(password));
        }
    }

    pub fn set_info(
        &mut self,
        username: String,
        hostname: String,
        servername: String,
        realname: String,
    ) {
        if self.builder.is_some() {
            self.builder = Some(
                self.builder
                    .take()
                    .unwrap()
                    .username(username)
                    .hostname(hostname)
                    .servername(servername)
                    .realname(realname),
            );
        }
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
}
