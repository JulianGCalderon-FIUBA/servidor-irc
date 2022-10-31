use crate::server::{
    client_handler::{
        registration::RegistrationState, responses::errors::ErrorReply, ClientHandler,
    },
    client_trait::ClientTrait,
};

impl<T: ClientTrait> ClientHandler<T> {
    pub fn assert_who_is_valid(&self, _parameters: &[String]) -> Option<ErrorReply> {
        if self.registration.state() != &RegistrationState::Registered {
            return Some(ErrorReply::UnregisteredClient);
        }

        None
    }

    pub fn assert_whois_is_valid(&self, parameters: &Vec<String>) -> Option<ErrorReply> {
        if parameters.is_empty() {
            return Some(ErrorReply::NoNicknameGiven431);
        }
        if self.registration.state() != &RegistrationState::Registered {
            return Some(ErrorReply::UnregisteredClient);
        }
        None
    }
}
