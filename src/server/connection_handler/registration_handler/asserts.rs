use crate::server::connection::Connection;
use crate::server::connection_handler::commands::{PASS_COMMAND, USER_COMMAND};
use crate::server::connection_handler::connection_handler_trait::ConnectionHandlerAsserts;
use crate::server::connection_handler::responses::ErrorReply;

use super::RegistrationHandler;

impl<C: Connection> ConnectionHandlerAsserts<C> for RegistrationHandler<C> {
    fn assert_pass_command_is_valid(&self, params: &[String]) -> Result<(), ErrorReply> {
        if params.is_empty() {
            let command = PASS_COMMAND.to_string();
            return Err(ErrorReply::NeedMoreParameters461 { command });
        }

        if self.attributes.get("nickname").is_some() {
            return Err(ErrorReply::AlreadyRegistered462);
        }

        Ok(())
    }

    fn assert_nick_command_is_valid(&self, params: &[String]) -> Result<(), ErrorReply> {
        if params.is_empty() {
            return Err(ErrorReply::NoNicknameGiven431);
        }

        let nickname = &params[0];

        self.assert_nickname_collision(nickname)
    }

    fn assert_user_command_is_valid(
        &self,
        params: &[String],
        trail: &Option<String>,
    ) -> Result<(), ErrorReply> {
        if params.is_empty() || trail.is_none() {
            let command = USER_COMMAND.to_string();
            return Err(ErrorReply::NeedMoreParameters461 { command });
        }

        if self.attributes.get("nickname").is_none() {
            return Err(ErrorReply::NoNickname);
        }

        Ok(())
    }

    fn assert_oper_command_is_valid(&self, _params: &[String]) -> Result<(), ErrorReply> {
        Err(ErrorReply::NotRegistered451)
    }

    fn assert_privmsg_command_is_valid(
        &self,
        _params: &[String],
        _trail: &Option<String>,
    ) -> Result<(), ErrorReply> {
        Err(ErrorReply::NotRegistered451)
    }

    fn assert_notice_command_is_valid(
        &self,
        _params: &[String],
        _trail: &Option<String>,
    ) -> Result<(), ErrorReply> {
        Err(ErrorReply::NotRegistered451)
    }

    fn assert_join_command_is_valid(&self, _params: &[String]) -> Result<(), ErrorReply> {
        Err(ErrorReply::NotRegistered451)
    }

    fn assert_part_command_is_valid(&self, _params: &[String]) -> Result<(), ErrorReply> {
        Err(ErrorReply::NotRegistered451)
    }

    fn assert_invite_command_is_valid(&self, _params: &[String]) -> Result<(), ErrorReply> {
        Err(ErrorReply::NotRegistered451)
    }

    fn assert_names_command_is_valid(&self, _params: &[String]) -> Result<(), ErrorReply> {
        Err(ErrorReply::NotRegistered451)
    }

    fn assert_list_command_is_valid(&self, _params: &[String]) -> Result<(), ErrorReply> {
        Err(ErrorReply::NotRegistered451)
    }

    fn assert_who_command_is_valid(&self, _params: &[String]) -> Result<(), ErrorReply> {
        Err(ErrorReply::NotRegistered451)
    }

    fn assert_whois_command_is_valid(&self, _params: &[String]) -> Result<(), ErrorReply> {
        Err(ErrorReply::NotRegistered451)
    }

    fn assert_away_command_is_valid(&self, _trail: &Option<String>) -> Result<(), ErrorReply> {
        Err(ErrorReply::NotRegistered451)
    }

    fn assert_topic_command_is_valid(&self, _params: &[String]) -> Result<(), ErrorReply> {
        Err(ErrorReply::NotRegistered451)
    }

    fn assert_kick_command_is_valid(
        &self,
        _params: &[String],
        _trail: &Option<String>,
    ) -> Result<(), ErrorReply> {
        Err(ErrorReply::NotRegistered451)
    }

    fn assert_mode_command_is_valid(&self, _params: &[String]) -> Result<(), ErrorReply> {
        Err(ErrorReply::NotRegistered451)
    }

    fn assert_quit_command_is_valid(&self, _trail: &Option<String>) -> Result<(), ErrorReply> {
        Ok(())
    }
}

impl<C: Connection> RegistrationHandler<C> {
    pub fn assert_nickname_collision(&self, nickname: &str) -> Result<(), ErrorReply> {
        let nickname = nickname.to_string();

        if self.database.contains_client(&nickname) {
            return Err(ErrorReply::NickCollision436 { nickname });
        }

        Ok(())
    }
}
