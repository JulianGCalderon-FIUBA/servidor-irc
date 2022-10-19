use std::{
    net::TcpStream,
    sync::{Arc, Mutex},
};

mod registration_state;

use crate::server::database::{ClientInfo, ClientInfoBuilder};
pub use registration_state::RegistrationState;

/// Holds a Clients' relevant information.
pub struct ConnectionInfo {
    pub password: Option<String>,
    pub nickname: Option<String>,
    pub username: Option<String>,
    pub hostname: Option<String>,
    pub servername: Option<String>,
    pub realname: Option<String>,
    pub registration_state: RegistrationState,
}

impl ConnectionInfo {
    pub fn new() -> Self {
        Self {
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

    pub fn build_client_info(&mut self, stream: Arc<Mutex<TcpStream>>) -> Option<ClientInfo> {
        let mut client_builder = ClientInfoBuilder::new_with(
            self.nickname.clone()?,
            self.username.clone()?,
            self.hostname.clone()?,
            self.servername.clone()?,
            self.realname.clone()?,
        );

        client_builder.with_stream(stream);

        if let Some(password) = self.password.clone() {
            client_builder.with_password(password);
        }

        Some(client_builder.build())
    }
}
