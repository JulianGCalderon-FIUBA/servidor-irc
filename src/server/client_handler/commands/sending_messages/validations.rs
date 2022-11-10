use crate::server::{client_handler::registration::RegistrationState, client_trait::Connection};

use crate::server::client_handler::responses::errors::ErrorReply;

use super::ClientHandler;

impl<C: Connection> ClientHandler<C> {
    /// Asserts target for message exists.
    /// Possible errors:
    ///     - Target does not exist.
    pub fn assert_target_is_valid(&self, target: &str) -> Option<ErrorReply> {
        let target = target.to_string();

        let is_client = self.database.contains_client(&target);
        let is_channel = self.database.contains_channel(&target);

        if !(is_client || is_channel) {
            return Some(ErrorReply::NoSuchNickname401 { nickname: target });
        }

        None
    }
    /// Asserts message can be sent.
    /// Possible errors:
    ///     - Not enough parameters.
    ///     - Client is not registered.
    pub fn assert_message_command_is_valid(
        &self,
        command: &str,
        parameters: &Vec<String>,
        trailing: &Option<String>,
    ) -> Option<ErrorReply> {
        if parameters.is_empty() {
            let command = command.to_string();
            return Some(ErrorReply::NoRecipient411 { command });
        }

        if trailing.is_none() {
            return Some(ErrorReply::NoTextToSend412 {});
        }

        if self.registration.state() != &RegistrationState::Registered {
            return Some(ErrorReply::UnregisteredClient {});
        }

        None
    }
}
