use std::net::TcpStream;

mod registration_state;

use crate::server::database::{ClientInfo, ClientInfoBuilder};
pub use registration_state::RegistrationState;

/// Holds a Clients' relevant information.
pub struct ConnectionInfo {
    pub stream: Option<TcpStream>,
    pub password: Option<String>,
    pub nickname: Option<String>,
    pub username: Option<String>,
    pub hostname: Option<String>,
    pub servername: Option<String>,
    pub realname: Option<String>,
    pub registration_state: RegistrationState,
}

impl ConnectionInfo {
    pub fn new_with_stream(stream: TcpStream) -> Self {
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

    pub fn build_client_info(&mut self) -> Option<ClientInfo> {
        let mut client_builder = ClientInfoBuilder::new_with(
            self.nickname.clone()?,
            self.username.clone()?,
            self.hostname.clone()?,
            self.servername.clone()?,
            self.realname.clone()?,
        );

        client_builder.with_stream(self.stream.take()?);

        if let Some(password) = self.password.clone() {
            client_builder.with_password(password);
        }

        Some(client_builder.build())
    }
}
