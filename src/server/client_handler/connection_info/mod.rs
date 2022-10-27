use std::io::{Read, Write};

mod registration_state;

use crate::server::database::{ClientInfo, ClientInfoBuilder};
pub use registration_state::RegistrationState;

/// Holds a Clients' relevant information.
pub struct ConnectionInfo<T> {
    pub stream: Option<T>,
    pub password: Option<String>,
    pub nickname: Option<String>,
    pub username: Option<String>,
    pub hostname: Option<String>,
    pub servername: Option<String>,
    pub realname: Option<String>,
    pub registration_state: RegistrationState,
}

impl<T: Read + Write> ConnectionInfo<T> {
    pub fn new_with_stream(stream: T) -> Self {
        Self {
            stream: Some(stream),
            password: None,
            nickname: None,
            username: None,
            hostname: None,
            servername: None,
            realname: None,
            registration_state: RegistrationState::NotInitialized,
        }
    }
    pub fn advance_state(&mut self) {
        self.registration_state = self.registration_state.next();
    }

    pub fn build_client_info(&mut self) -> Option<ClientInfo<T>> {
        let mut client_builder = ClientInfoBuilder::new()
            .nickname(self.nickname.clone()?)
            .username(self.username.clone()?)
            .hostname(self.hostname.clone()?)
            .servername(self.servername.clone()?)
            .realname(self.realname.clone()?)
            .stream(self.stream.take().unwrap());

        if let Some(password) = self.password.clone() {
            client_builder = client_builder.password(password);
        }

        client_builder.build()
    }

    pub fn nickname(&self) -> String {
        self.nickname.clone().unwrap()
    }
}
