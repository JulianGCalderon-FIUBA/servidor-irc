use crate::server::{client_handler::registration::RegistrationState, client_trait::ClientTrait};

use crate::server::client_handler::responses::errors::ErrorReply;

use super::ClientHandler;

use super::PRIVMSG_COMMAND;

impl<T: ClientTrait> ClientHandler<T> {
    pub fn assert_target_is_valid(&self, target: &str) -> Option<ErrorReply> {
        let target = target.to_string();

        let is_client = self.database.contains_client(&target);
        let is_channel = self.database.contains_channel(&target);

        if !(is_client || is_channel) {
            return Some(ErrorReply::NoSuchNickname401 { nickname: target });
        }

        let nickname = self.registration.nickname().unwrap();
        if is_channel && !self.database.is_client_in_channel(&nickname, &target) {
            return Some(ErrorReply::CanNotSendToChannel404 { channel: target });
        }

        None
    }

    pub fn assert_privmsg_command_is_valid(
        &self,
        parameters: &Vec<String>,
        trailing: &Option<String>,
    ) -> Option<ErrorReply> {
        if parameters.is_empty() {
            return Some(ErrorReply::NoRecipient411 {
                command: PRIVMSG_COMMAND.to_string(),
            });
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
