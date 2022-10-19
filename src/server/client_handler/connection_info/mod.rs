use std::{io, net::TcpStream};

mod registration_state;

use crate::server::database::{ClientInfo, ClientInfoBuilder};
pub use registration_state::RegistrationState;

/// Holds a Clients' relevant information.
pub struct ConnectionInfo {
    pub stream: TcpStream,
    pub password: Option<String>,
    pub nickname: Option<String>,
    pub username: Option<String>,
    pub hostname: Option<String>,
    pub servername: Option<String>,
    pub realname: Option<String>,
    pub registration_state: RegistrationState,
}

impl ConnectionInfo {
    pub fn with_stream(stream: TcpStream) -> Self {
        Self {
            stream,
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

    pub fn get_client_info(&self) -> io::Result<Option<ClientInfo>> {
        if self.registration_state != RegistrationState::Registered {
            return Ok(None);
        }

        let mut client_builder = ClientInfoBuilder::new_with(
            self.nickname.clone().unwrap(),
            self.username.clone().unwrap(),
            self.hostname.clone().unwrap(),
            self.servername.clone().unwrap(),
            self.realname.clone().unwrap(),
        );
        client_builder.with_stream(self.stream.try_clone()?);

        if let Some(password) = self.password.clone() {
            client_builder.with_password(password);
        }

        Ok(Some(client_builder.build()))
    }
}
