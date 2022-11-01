use crate::server::{client_handler::registration::RegistrationState, client_trait::ClientTrait};

use crate::server::client_handler::responses::errors::ErrorReply;

use super::ClientHandler;

impl<T: ClientTrait> ClientHandler<T> {
    pub fn assert_target_is_valid(&self, target: &str) -> Option<ErrorReply> {
        let target = target.to_string();

        let is_client = self.contains_client(&target);
        let is_channel = self.contains_channel(&target);

        if !(is_client || is_channel) {
            return Some(ErrorReply::NoSuchNickname401 { nickname: target });
        }

        let nickname = self.registration.nickname().unwrap();
        if is_channel && !self.is_client_in_channel(&nickname, &target) {
            return Some(ErrorReply::CanNotSendToChannel404 { channel: target });
        }

        None
    }

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
