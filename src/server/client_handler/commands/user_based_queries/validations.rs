use crate::server::client_handler::registration::RegistrationState;
use crate::server::client_handler::responses::errors::ErrorReply;
use crate::server::client_handler::ClientHandler;
use crate::server::client_trait::ClientTrait;

impl<T: ClientTrait> ClientHandler<T> {
    /// Asserts whois command can be executed.
    /// Possible errors:
    ///     - Client is not registered.
    pub fn assert_who_is_valid(&self, _parameters: &[String]) -> Option<ErrorReply> {
        if self.registration.state() != &RegistrationState::Registered {
            return Some(ErrorReply::UnregisteredClient);
        }

        None
    }
    /// Asserts who command can be executed.
    /// Possible errors:
    ///     - Client is not registered.
    ///     - Not enough parameters.
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
