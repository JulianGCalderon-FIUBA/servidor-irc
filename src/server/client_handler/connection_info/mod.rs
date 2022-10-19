use std::net::TcpStream;

mod registration_state;

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
}
