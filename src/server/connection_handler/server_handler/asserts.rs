use crate::server::connection::Connection;
use crate::server::connection_handler::connection_handler_trait::{
    CommandArgs, ConnectionHandlerAsserts,
};

use crate::server::consts::modes::{ADD_MODE, REMOVE_MODE, VALID_CHANNEL_MODES, VALID_USER_MODES};
use crate::server::responses::ErrorReply;

use super::ServerHandler;

impl<C: Connection> ConnectionHandlerAsserts<C> for ServerHandler<C> {
    fn assert_pass_command_is_valid(&self, _arguments: &CommandArgs) -> Result<(), ErrorReply> {
        Err(ErrorReply::AlreadyRegistered462)
    }

    fn assert_nick_command_is_valid(&self, arguments: &CommandArgs) -> Result<(), ErrorReply> {
        let (prefix, params, _) = arguments;
        if params.len() < 2 && prefix.is_none() {
            return Err(ErrorReply::NoReply);
        }
        if params.is_empty() && prefix.is_some() {
            return Err(ErrorReply::NoReply);
        }

        if let Some(hopcount) = params.get(1) {
            if hopcount.parse::<usize>().is_err() {
                return Err(ErrorReply::NoReply);
            }
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

        let target = &params[0];
        if !self.database.contains_channel(target) && !self.database.contains_client(target) {
            return Err(ErrorReply::NoReply);
        }

        Ok(())
    }

    fn assert_notice_command_is_valid(&self, arguments: &CommandArgs) -> Result<(), ErrorReply> {
        let (prefix, params, trail) = arguments;
        if params.is_empty() || prefix.is_none() || trail.is_none() {
            return Err(ErrorReply::NoReply);
        }

        let target = &params[0];
        if !self.database.contains_channel(target) || !self.database.contains_client(target) {
            return Err(ErrorReply::NoReply);
        }

        Ok(())
    }

    fn assert_join_command_is_valid(&self, arguments: &CommandArgs) -> Result<(), ErrorReply> {
        let (prefix, params, _) = arguments;
        if params.is_empty() || prefix.is_none() {
            return Err(ErrorReply::NoReply);
        }

        let nickname = prefix.as_ref().unwrap();
        if !self.database.contains_client(nickname) {
            return Err(ErrorReply::NoReply);
        }

        Ok(())
    }

    fn assert_part_command_is_valid(&self, arguments: &CommandArgs) -> Result<(), ErrorReply> {
        let (prefix, params, _) = arguments;
        if params.is_empty() || prefix.is_none() {
            return Err(ErrorReply::NoReply);
        }

        let nickname = prefix.as_ref().unwrap();
        let channel = &params[0];
        if !self.database.contains_client(nickname) || !self.database.contains_channel(channel) {
            return Err(ErrorReply::NoReply);
        }
        Ok(())
    }

    fn assert_invite_command_is_valid(&self, arguments: &CommandArgs) -> Result<(), ErrorReply> {
        let (prefix, params, _) = arguments;
        if params.len() < 2 || prefix.is_none() {
            return Err(ErrorReply::NoReply);
        }

        let inviting = prefix.as_ref().unwrap();
        if !self.database.contains_client(inviting) {
            return Err(ErrorReply::NoReply);
        }

        Ok(())
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

    fn assert_away_command_is_valid(&self, arguments: &CommandArgs) -> Result<(), ErrorReply> {
        let (prefix, _, _) = arguments;
        if prefix.is_none() {
            return Err(ErrorReply::NoReply);
        }

        let nickname = prefix.as_ref().unwrap();
        if !self.database.contains_client(nickname) {
            return Err(ErrorReply::NoReply);
        }

        Ok(())
    }

    fn assert_topic_command_is_valid(&self, arguments: &CommandArgs) -> Result<(), ErrorReply> {
        let (prefix, params, _) = arguments;

        if prefix.is_none() || params.len() < 2 {
            return Err(ErrorReply::NoReply);
        }

        let nickname = prefix.as_ref().unwrap();
        let channel = &params[0];
        if !self.database.contains_client(nickname) || !self.database.contains_channel(channel) {
            return Err(ErrorReply::NoReply);
        }

        Ok(())
    }

    fn assert_kick_command_is_valid(&self, arguments: &CommandArgs) -> Result<(), ErrorReply> {
        let (prefix, params, _) = arguments;

        if prefix.is_none() || params.len() < 2 {
            return Err(ErrorReply::NoReply);
        }

        let kicked = &params[1];
        let channel = &params[0];

        if !self.database.contains_client(kicked) || !self.database.contains_channel(channel) {
            return Err(ErrorReply::NoReply);
        }

        Ok(())
    }

    fn assert_mode_command_is_valid(&self, arguments: &CommandArgs) -> Result<(), ErrorReply> {
        let (prefix, params, _) = arguments;
        if params.len() < 2 {
            return Err(ErrorReply::NoReply);
        }

        let mode = &params[1];
        if mode.len() != 2 {
            return Err(ErrorReply::NoReply);
        }

        if !mode.starts_with([ADD_MODE, REMOVE_MODE])
            || !(mode.ends_with(VALID_CHANNEL_MODES) || mode.ends_with(VALID_USER_MODES))
        {
            return Err(ErrorReply::NoReply);
        }

        if prefix.is_none() {
            return Err(ErrorReply::NoReply);
        }

        Ok(())
    }

    fn assert_quit_command_is_valid(&self, arguments: &CommandArgs) -> Result<(), ErrorReply> {
        let (prefix, _, trail) = arguments;
        if prefix.is_none() || trail.is_none() {
            return Err(ErrorReply::NoReply);
        }
        let nickname = prefix.as_ref().unwrap();
        if !self.database.contains_client(nickname) {
            return Err(ErrorReply::NoReply);
        }
        Ok(())
    }

    fn assert_server_command_is_valid(&self, arguments: &CommandArgs) -> Result<(), ErrorReply> {
        let (_, params, trail) = arguments;
        if params.len() < 2 || trail.is_none() {
            return Err(ErrorReply::NoReply);
        }

        let hopcount = &params[1];
        if hopcount.parse::<usize>().is_err() {
            return Err(ErrorReply::NoReply);
        }

        Ok(())
    }

    fn assert_squit_command_is_valid(&self, arguments: &CommandArgs) -> Result<(), ErrorReply> {
        let (prefix, params, _) = arguments;
        if params.is_empty() || prefix.is_none() {
            return Err(ErrorReply::NoReply);
        }

        let servername = &params[0];

        if !self.database.contains_server(servername) {
            return Err(ErrorReply::NoReply);
        }
        Ok(())
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
