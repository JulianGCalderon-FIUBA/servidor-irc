use crate::server::client_handler::registration::RegistrationState;
use crate::server::client_handler::responses::errors::ErrorReply;
use crate::server::client_trait::ClientTrait;

use super::ClientHandler;
use std::io;

use super::OPER_COMMAND;
use super::PASS_COMMAND;
use super::USER_COMMAND;

impl<T: ClientTrait> ClientHandler<T> {
    pub fn assert_pass_command_is_valid(&mut self, parameters: &Vec<String>) -> Option<ErrorReply> {
        if parameters.len() != 1 {
            return Some(ErrorReply::NeedMoreParameters461 {
                command: PASS_COMMAND.to_string(),
            });
        }

        if self.registration.state() != &RegistrationState::NotInitialized {
            return Some(ErrorReply::AlreadyRegistered462);
        }

        None
    }

    pub fn assert_nick_command_is_valid(&mut self, parameters: &Vec<String>) -> Option<ErrorReply> {
        if parameters.is_empty() {
            return Some(ErrorReply::NoNicknameGiven431);
        }

        let nickname = &parameters[0].to_string();
        if self.database.contains_client(nickname) {
            if self.registration.state() == &RegistrationState::Registered {
                return Some(ErrorReply::NicknameInUse433 {
                    nickname: nickname.to_string(),
                });
            } else {
                return Some(ErrorReply::NickCollision436 {
                    nickname: nickname.to_string(),
                });
            }
        }

        None
    }

    pub fn assert_user_command_is_valid(
        &mut self,
        parameters: &Vec<String>,
        trailing: &Option<String>,
    ) -> Option<ErrorReply> {
        if parameters.len() != 3 || trailing.is_none() {
            return Some(ErrorReply::NeedMoreParameters461 {
                command: USER_COMMAND.to_string(),
            });
        }

        if self.registration.state() != &RegistrationState::NicknameSent {
            return Some(ErrorReply::NoNickname);
        }

        None
    }

    pub fn validate_oper_command(&mut self, parameters: &Vec<String>) -> io::Result<bool> {
        if parameters.len() != 2 {
            self.need_more_params_error(OPER_COMMAND)?;
            return Ok(false);
        }

        if self.registration.state() != &RegistrationState::Registered {
            self.unregistered_error()?;
            return Ok(false);
        }

        Ok(true)
    }
}
