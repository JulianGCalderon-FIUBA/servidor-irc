use std::io::{Read, Write};

mod registration_state;

use crate::server::database::{ClientInfo, ClientInfoBuilder};
pub use registration_state::RegistrationState;

/// Holds a Clients' relevant information.
pub struct ConnectionInfo<T: Read + Write> {
    pub nickname: Option<String>,
    pub state: RegistrationState,
    pub builder: Option<ClientInfoBuilder<T>>,
}

impl<T: Read + Write> ConnectionInfo<T> {
    pub fn with_stream(stream: T) -> Self {
        Self {
            nickname: None,
            state: RegistrationState::NotInitialized,
            builder: Some(ClientInfoBuilder::new().stream(stream)),
        }
    }

    pub fn build(&mut self) -> ClientInfo<T> {
        self.builder.take().unwrap().build().unwrap()
    }

    pub fn set_nickname(&mut self, nickname: String) {
        self.nickname = Some(nickname.clone());
        self.builder = Some(self.builder.take().unwrap().nickname(nickname));
    }

    pub fn set_password(&mut self, password: String) {
        self.builder = Some(self.builder.take().unwrap().password(password));
    }

    pub fn set_info(
        &mut self,
        username: String,
        hostname: String,
        servername: String,
        realname: String,
    ) {
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

    pub fn advance_state(&mut self) {
        self.state = self.state.next();
    }

    pub fn nickname(&self) -> String {
        self.nickname.clone().unwrap()
    }
}
