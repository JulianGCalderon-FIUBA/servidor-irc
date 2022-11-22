use crate::server::connection::Connection;
use crate::server::connection_handler::connection_handler_trait::{
    CommandArgs, ConnectionHandlerAsserts,
};
use crate::server::responses::ErrorReply;

use super::ServerHandler;

impl<C: Connection> ConnectionHandlerAsserts<C> for ServerHandler<C> {
    fn assert_pass_command_is_valid(&self, _arguments: &CommandArgs) -> Result<(), ErrorReply> {
        Err(ErrorReply::AlreadyRegistered462)
    }

    fn assert_nick_command_is_valid(&self, arguments: &CommandArgs) -> Result<(), ErrorReply> {
        let (_, params, _) = arguments;
        if params.is_empty() {
            return Err(ErrorReply::NoReply);
        }

        let nickname = &params[0];
        self.assert_nickname_not_in_use(nickname)
    }

    fn assert_user_command_is_valid(&self, arguments: &CommandArgs) -> Result<(), ErrorReply> {
        let (prefix, params, trail) = arguments;

        if params.len() < 3 || trail.is_none() || prefix.is_none() {
            return Err(ErrorReply::NoReply);
        }

        let nickname = prefix.as_ref().unwrap();

        if self.hopcounts.get(nickname).is_none() {
            return Err(ErrorReply::NoReply);
        }

        Ok(())
    }

    fn assert_oper_command_is_valid(&self, _arguments: &CommandArgs) -> Result<(), ErrorReply> {
        todo!()
    }

    fn assert_privmsg_command_is_valid(&self, arguments: &CommandArgs) -> Result<(), ErrorReply> {
        let (prefix, params, trail) = arguments;
        if params.is_empty() || prefix.is_none() || trail.is_none() {
            return Err(ErrorReply::NoReply);
        }

        Ok(())
    }

    fn assert_notice_command_is_valid(&self, _arguments: &CommandArgs) -> Result<(), ErrorReply> {
        todo!()
    }

    fn assert_join_command_is_valid(&self, _arguments: &CommandArgs) -> Result<(), ErrorReply> {
        todo!()
    }

    fn assert_part_command_is_valid(&self, _arguments: &CommandArgs) -> Result<(), ErrorReply> {
        todo!()
    }

    fn assert_invite_command_is_valid(&self, _arguments: &CommandArgs) -> Result<(), ErrorReply> {
        todo!()
    }

    fn assert_names_command_is_valid(&self, _arguments: &CommandArgs) -> Result<(), ErrorReply> {
        Err(ErrorReply::NoReply)
    }

    fn assert_list_command_is_valid(&self, _arguments: &CommandArgs) -> Result<(), ErrorReply> {
        Err(ErrorReply::NoReply)
    }

    fn assert_who_command_is_valid(&self, _arguments: &CommandArgs) -> Result<(), ErrorReply> {
        Err(ErrorReply::NoReply)
    }

    fn assert_whois_command_is_valid(&self, _arguments: &CommandArgs) -> Result<(), ErrorReply> {
        Err(ErrorReply::NoReply)
    }

    fn assert_away_command_is_valid(&self, _arguments: &CommandArgs) -> Result<(), ErrorReply> {
        Err(ErrorReply::NoReply)
    }

    fn assert_topic_command_is_valid(&self, _arguments: &CommandArgs) -> Result<(), ErrorReply> {
        todo!()
    }

    fn assert_kick_command_is_valid(&self, _arguments: &CommandArgs) -> Result<(), ErrorReply> {
        todo!()
    }

    fn assert_mode_command_is_valid(&self, _arguments: &CommandArgs) -> Result<(), ErrorReply> {
        todo!()
    }

    fn assert_quit_command_is_valid(&self, _arguments: &CommandArgs) -> Result<(), ErrorReply> {
        todo!()
    }

    fn assert_server_command_is_valid(&self, _arguments: &CommandArgs) -> Result<(), ErrorReply> {
        todo!()
    }

    fn assert_squit_command_is_valid(&self, _arguments: &CommandArgs) -> Result<(), ErrorReply> {
        todo!()
    }
}

impl<C: Connection> ServerHandler<C> {
    pub fn assert_nickname_not_in_use(&self, nickname: &str) -> Result<(), ErrorReply> {
        let nickname = nickname.to_string();

        if self.database.contains_client(&nickname) {
            return Err(ErrorReply::NickCollision436 { nickname });
        }
        // KILL
        Ok(())
    }
}
