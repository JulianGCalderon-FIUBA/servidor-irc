use crate::server::connection::Connection;
use crate::server::connection_handler::commands::{PASS_COMMAND, USER_COMMAND};
use crate::server::connection_handler::connection_handler_trait::ConnectionHandlerAsserts;
use crate::server::connection_handler::responses::ErrorReply;

use super::RegistrationHandler;

impl<C: Connection> ConnectionHandlerAsserts<C> for RegistrationHandler<C> {
    fn assert_pass_command_is_valid(
        &self,
        parameters: &[String],
    ) -> Result<(), crate::server::connection_handler::responses::ErrorReply> {
        if parameters.is_empty() {
            let command = PASS_COMMAND.to_string();
            return Err(ErrorReply::NeedMoreParameters461 { command });
        }

        if self.attributes.get("nickname").is_some() {
            return Err(ErrorReply::AlreadyRegistered462);
        }

        Ok(())
    }

    fn assert_nick_command_is_valid(&self, parameters: &[String]) -> Result<(), ErrorReply> {
        if parameters.is_empty() {
            return Err(ErrorReply::NoNicknameGiven431);
        }

        Ok(())
    }

    fn assert_user_command_is_valid(
        &self,
        parameters: &[String],
        trailing: &Option<String>,
    ) -> Result<(), ErrorReply> {
        if parameters.is_empty() || trailing.is_none() {
            let command = USER_COMMAND.to_string();
            return Err(ErrorReply::NeedMoreParameters461 { command });
        }

        if self.attributes.get("nickname").is_none() {
            return Err(ErrorReply::NoNickname);
        }

        Ok(())
    }
}
