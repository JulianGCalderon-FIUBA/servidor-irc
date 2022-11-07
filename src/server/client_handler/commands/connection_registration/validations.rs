use super::super::{OPER_COMMAND, PASS_COMMAND, USER_COMMAND};
use crate::server::client_handler::registration::RegistrationState;
use crate::server::client_handler::responses::errors::ErrorReply;
use crate::server::client_trait::Connection;

use super::ClientHandler;

impl<C: Connection> ClientHandler<C> {
    /// Asserts pass command can be executed.
    /// Possible errors:
    ///     - Not enough parameters.
    ///     - Client is already registered.
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
    /// Asserts nick command can be executed.
    /// Possible errors:
    ///     - Not enough parameters.
    ///     - Nickname is already in use.
    ///     - Nick collision.
    pub fn assert_nick_command_is_valid(&self, parameters: &Vec<String>) -> Option<ErrorReply> {
        if parameters.is_empty() {
            return Some(ErrorReply::NoNicknameGiven431);
        }

        let nickname = parameters[0].to_string();

        if self.database.contains_client(&nickname) {
            if self.registration.state() == &RegistrationState::Registered {
                return Some(ErrorReply::NicknameInUse433 { nickname });
            } else {
                return Some(ErrorReply::NickCollision436 { nickname });
            }
        }

        None
    }
    /// Asserts user command can be executed.
    /// Possible errors:
    ///     - Not enough parameters.
    ///     - Client has not registered nickname yet.
    ///     - Client is already registered.
    pub fn assert_user_command_is_valid(
        &self,
        parameters: &Vec<String>,
        trailing: &Option<String>,
    ) -> Option<ErrorReply> {
        if parameters.is_empty() || trailing.is_none() {
            let command = USER_COMMAND.to_string();
            return Some(ErrorReply::NeedMoreParameters461 { command });
        }

        if self.registration.state() == &RegistrationState::NotInitialized {
            return Some(ErrorReply::NoNickname);
        }

        if self.registration.state() == &RegistrationState::Registered {
            return Some(ErrorReply::AlreadyRegistered462);
        }

        None
    }
    /// Asserts oper command can be executed.
    /// Possible errors:
    ///     - Not enough parameters.
    ///     - Client is not registered.
    pub fn assert_oper_command_is_valid(&self, parameters: &Vec<String>) -> Option<ErrorReply> {
        if parameters.len() != 2 {
            let command = OPER_COMMAND.to_string();
            return Some(ErrorReply::NeedMoreParameters461 { command });
        }

        if self.registration.state() != &RegistrationState::Registered {
            return Some(ErrorReply::UnregisteredClient);
        }

        let username = &parameters[0];
        let password = &parameters[1];
        if !self.database.are_credentials_valid(username, password) {
            return Some(ErrorReply::PasswordMismatch464);
        }

        None
    }
}
