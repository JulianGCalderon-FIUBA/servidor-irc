use crate::server::client_handler::registration::RegistrationState;
use crate::server::client_handler::responses::errors::ErrorReply;
use crate::server::client_trait::ClientTrait;

use super::ClientHandler;

use super::OPER_COMMAND;
use super::PASS_COMMAND;
use super::USER_COMMAND;

impl<T: ClientTrait> ClientHandler<T> {
    pub fn assert_pass_command_is_valid(&self, parameters: &Vec<String>) -> Option<ErrorReply> {
        if parameters.len() != 1 {
            let command = PASS_COMMAND.to_string();
            return Some(ErrorReply::NeedMoreParameters461 { command });
        }

        if self.registration.state() != &RegistrationState::NotInitialized {
            return Some(ErrorReply::AlreadyRegistered462);
        }

        None
    }

    pub fn assert_nick_command_is_valid(&self, parameters: &Vec<String>) -> Option<ErrorReply> {
        if parameters.is_empty() {
            return Some(ErrorReply::NoNicknameGiven431);
        }

        let nickname = parameters[0].to_string();

        if self.contains_client(&nickname) {
            if self.registration.state() == &RegistrationState::Registered {
                return Some(ErrorReply::NicknameInUse433 { nickname });
            } else {
                return Some(ErrorReply::NickCollision436 { nickname });
            }
        }

        None
    }

    pub fn assert_user_command_is_valid(
        &self,
        parameters: &Vec<String>,
        trailing: &Option<String>,
    ) -> Option<ErrorReply> {
        if parameters.len() != 3 || trailing.is_none() {
            let command = USER_COMMAND.to_string();
            return Some(ErrorReply::NeedMoreParameters461 { command });
        }

        if self.registration.state() != &RegistrationState::NicknameSent {
            return Some(ErrorReply::NoNickname);
        }

        None
    }

    pub fn assert_oper_command_is_valid(&self, parameters: &Vec<String>) -> Option<ErrorReply> {
        if parameters.len() != 2 {
            let command = OPER_COMMAND.to_string();
            return Some(ErrorReply::NeedMoreParameters461 { command });
        }

        if self.registration.state() != &RegistrationState::Registered {
            return Some(ErrorReply::UnregisteredClient);
        }

        None
    }
}
