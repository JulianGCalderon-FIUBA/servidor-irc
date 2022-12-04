use crate::server::connection::Connection;
use crate::server::connection_handler::connection_handler_trait::{
    CommandArgs, ConnectionHandlerAsserts,
};
use crate::server::consts::commands::*;
use crate::server::responses::ErrorReply;

use super::RegistrationHandler;

impl<C: Connection> ConnectionHandlerAsserts<C> for RegistrationHandler<C> {
    fn assert_pass_command_is_valid(&self, arguments: &CommandArgs) -> Result<(), ErrorReply> {
        let (_, params, _) = arguments;

        if params.is_empty() {
            let command = PASS_COMMAND.to_string();
            return Err(ErrorReply::NeedMoreParameters461 { command });
        }

        if self.attributes.get("nickname").is_some() {
            return Err(ErrorReply::AlreadyRegistered462);
        }

        Ok(())
    }

    fn assert_nick_command_is_valid(&self, arguments: &CommandArgs) -> Result<(), ErrorReply> {
        let (_, params, _) = arguments;

        if params.is_empty() {
            return Err(ErrorReply::NoNicknameGiven431);
        }

        let nickname = &params[0];

        self.assert_nickname_collision(nickname)
    }

    fn assert_user_command_is_valid(&self, arguments: &CommandArgs) -> Result<(), ErrorReply> {
        let (_, params, trail) = arguments;

        if params.is_empty() || trail.is_none() {
            let command = USER_COMMAND.to_string();
            return Err(ErrorReply::NeedMoreParameters461 { command });
        }

        if self.attributes.get("nickname").is_none() {
            return Err(ErrorReply::NoNickname);
        }

        Ok(())
    }

    fn assert_oper_command_is_valid(&self, _arguments: &CommandArgs) -> Result<(), ErrorReply> {
        Err(ErrorReply::NotRegistered451)
    }

    fn assert_privmsg_command_is_valid(&self, _arguments: &CommandArgs) -> Result<(), ErrorReply> {
        Err(ErrorReply::NotRegistered451)
    }

    fn assert_notice_command_is_valid(&self, _arguments: &CommandArgs) -> Result<(), ErrorReply> {
        Err(ErrorReply::NotRegistered451)
    }

    fn assert_join_command_is_valid(&self, _arguments: &CommandArgs) -> Result<(), ErrorReply> {
        Err(ErrorReply::NotRegistered451)
    }

    fn assert_part_command_is_valid(&self, _arguments: &CommandArgs) -> Result<(), ErrorReply> {
        Err(ErrorReply::NotRegistered451)
    }

    fn assert_invite_command_is_valid(&self, _arguments: &CommandArgs) -> Result<(), ErrorReply> {
        Err(ErrorReply::NotRegistered451)
    }

    fn assert_names_command_is_valid(&self, _arguments: &CommandArgs) -> Result<(), ErrorReply> {
        Err(ErrorReply::NotRegistered451)
    }

    fn assert_list_command_is_valid(&self, _arguments: &CommandArgs) -> Result<(), ErrorReply> {
        Err(ErrorReply::NotRegistered451)
    }

    fn assert_who_command_is_valid(&self, _arguments: &CommandArgs) -> Result<(), ErrorReply> {
        Err(ErrorReply::NotRegistered451)
    }

    fn assert_whois_command_is_valid(&self, _arguments: &CommandArgs) -> Result<(), ErrorReply> {
        Err(ErrorReply::NotRegistered451)
    }

    fn assert_away_command_is_valid(&self, _arguments: &CommandArgs) -> Result<(), ErrorReply> {
        Err(ErrorReply::NotRegistered451)
    }

    fn assert_topic_command_is_valid(&self, _arguments: &CommandArgs) -> Result<(), ErrorReply> {
        Err(ErrorReply::NotRegistered451)
    }

    fn assert_kick_command_is_valid(&self, _arguments: &CommandArgs) -> Result<(), ErrorReply> {
        Err(ErrorReply::NotRegistered451)
    }

    fn assert_mode_command_is_valid(&self, _arguments: &CommandArgs) -> Result<(), ErrorReply> {
        Err(ErrorReply::NotRegistered451)
    }

    fn assert_quit_command_is_valid(&self, _arguments: &CommandArgs) -> Result<(), ErrorReply> {
        Ok(())
    }

    fn assert_server_command_is_valid(&self, arguments: &CommandArgs) -> Result<(), ErrorReply> {
        let (_, params, trail) = arguments;

        if params.len() < 2 || trail.is_none() {
            let command = SERVER_COMMAND.to_string();
            return Err(ErrorReply::NeedMoreParameters461 { command });
        }

        if params[1].parse::<usize>().is_err() {
            let command = SERVER_COMMAND.to_string();
            let message = "Hopcount is not numeric".to_string();
            return Err(ErrorReply::UnknownError400 { command, message });
        }

        Ok(())
    }

    fn assert_squit_command_is_valid(&self, _arguments: &CommandArgs) -> Result<(), ErrorReply> {
        Err(ErrorReply::NotRegistered451)
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
