use crate::server::connection::Connection;
use crate::server::connection_handler::commands::*;
use crate::server::connection_handler::connection_handler_trait::ConnectionHandlerAsserts;
use crate::server::connection_handler::responses::ErrorReply;

use super::ClientHandler;

impl<C: Connection> ConnectionHandlerAsserts<C> for ClientHandler<C> {
    fn assert_pass_command_is_valid(
        &self,
        _params: &[String],
    ) -> Result<(), crate::server::connection_handler::responses::ErrorReply> {
        Err(ErrorReply::AlreadyRegistered462)
    }

    fn assert_nick_command_is_valid(&self, _params: &[String]) -> Result<(), ErrorReply> {
        Err(ErrorReply::AlreadyRegistered462)
    }

    fn assert_user_command_is_valid(
        &self,
        _params: &[String],
        _trail: &Option<String>,
    ) -> Result<(), ErrorReply> {
        Err(ErrorReply::AlreadyRegistered462)
    }

    fn assert_oper_command_is_valid(&self, params: &[String]) -> Result<(), ErrorReply> {
        if params.len() != 2 {
            let command = OPER_COMMAND.to_string();
            return Err(ErrorReply::NeedMoreParameters461 { command });
        }

        let username = &params[0];
        let password = &params[1];
        if !self.database.are_credentials_valid(username, password) {
            return Err(ErrorReply::PasswordMismatch464);
        }

        Ok(())
    }

    fn assert_privmsg_command_is_valid(
        &self,
        params: &[String],
        trail: &Option<String>,
    ) -> Result<(), ErrorReply> {
        if params.is_empty() {
            let command = PRIVMSG_COMMAND.to_string();
            return Err(ErrorReply::NoRecipient411 { command });
        }

        if trail.is_none() {
            return Err(ErrorReply::NoTextToSend412 {});
        }

        Ok(())
    }

    fn assert_notice_command_is_valid(
        &self,
        params: &[String],
        trail: &Option<String>,
    ) -> Result<(), ErrorReply> {
        if params.is_empty() {
            let command = NOTICE_COMMAND.to_string();
            return Err(ErrorReply::NoRecipient411 { command });
        }

        if trail.is_none() {
            return Err(ErrorReply::NoTextToSend412 {});
        }

        Ok(())
    }

    fn assert_join_command_is_valid(&self, params: &[String]) -> Result<(), ErrorReply> {
        if params.is_empty() {
            let command = JOIN_COMMAND.to_string();
            return Err(ErrorReply::NeedMoreParameters461 { command });
        }

        Ok(())
    }

    fn assert_part_command_is_valid(&self, params: &[String]) -> Result<(), ErrorReply> {
        if params.is_empty() {
            let command = PART_COMMAND.to_string();
            return Err(ErrorReply::NeedMoreParameters461 { command });
        }

        Ok(())
    }

    fn assert_invite_command_is_valid(&self, params: &[String]) -> Result<(), ErrorReply> {
        if params.len() != 2 {
            let command = INVITE_COMMAND.to_string();
            return Err(ErrorReply::NeedMoreParameters461 { command });
        }

        let invited_client = params[0].to_string();
        let inviting_client = self.nickname.clone();
        let channel = params[1].to_string();

        if !self.database.contains_client(&invited_client) {
            return Err(ErrorReply::NoSuchNickname401 {
                nickname: invited_client,
            });
        }

        if self.database.contains_channel(&channel) {
            if !self
                .database
                .is_client_in_channel(&inviting_client, &channel)
            {
                return Err(ErrorReply::NotOnChannel442 { channel });
            }
            if self
                .database
                .is_client_in_channel(&invited_client, &channel)
            {
                return Err(ErrorReply::UserOnChannel443 {
                    nickname: invited_client,
                    channel,
                });
            }
        }

        if self.database.channel_has_mode(&channel, 'i')
            && !self
                .database
                .is_channel_operator(&channel, &inviting_client)
        {
            return Err(ErrorReply::ChanOPrivIsNeeded482 { channel });
        }

        Ok(())
    }

    fn assert_names_command_is_valid(&self, _params: &[String]) -> Result<(), ErrorReply> {
        Ok(())
    }

    fn assert_list_command_is_valid(&self, _params: &[String]) -> Result<(), ErrorReply> {
        Ok(())
    }

    fn assert_who_command_is_valid(&self, _params: &[String]) -> Result<(), ErrorReply> {
        Ok(())
    }

    fn assert_whois_command_is_valid(&self, params: &[String]) -> Result<(), ErrorReply> {
        if params.is_empty() {
            return Err(ErrorReply::NoNicknameGiven431);
        }

        Ok(())
    }

    fn assert_away_command_is_valid(&self, _trail: &Option<String>) -> Result<(), ErrorReply> {
        Ok(())
    }

    fn assert_topic_command_is_valid(&self, params: &[String]) -> Result<(), ErrorReply> {
        if params.is_empty() {
            let command = TOPIC_COMMAND.to_string();
            return Err(ErrorReply::NeedMoreParameters461 { command });
        }

        let nickname = self.nickname.clone();
        let channel = params[0].to_string();

        if !self.database.is_client_in_channel(&nickname, &channel) {
            return Err(ErrorReply::NotOnChannel442 { channel });
        }

        if self.database.channel_has_mode(&channel, 't')
            && !self.database.is_channel_operator(&channel, &nickname)
        {
            return Err(ErrorReply::ChanOPrivIsNeeded482 { channel });
        }

        Ok(())
    }

    fn assert_kick_command_is_valid(
        &self,
        params: &[String],
        _trail: &Option<String>,
    ) -> Result<(), ErrorReply> {
        if params.len() < 2 {
            let command = KICK_COMMAND.to_string();
            return Err(ErrorReply::NeedMoreParameters461 { command });
        }

        Ok(())
    }

    fn assert_mode_command_is_valid(&self, params: &[String]) -> Result<(), ErrorReply> {
        if params.is_empty() {
            let command = MODE_COMMAND.to_string();
            return Err(ErrorReply::NeedMoreParameters461 { command });
        }

        let channel = params[0].clone();
        let nickname = self.nickname.clone();

        if !self.database.contains_channel(&channel) {
            return Err(ErrorReply::NoSuchChannel403 { channel });
        }

        if !self.database.is_client_in_channel(&nickname, &channel) {
            return Err(ErrorReply::NotOnChannel442 { channel });
        }

        if !self.database.is_channel_operator(&channel, &nickname) && params.len() > 1 {
            return Err(ErrorReply::ChanOPrivIsNeeded482 { channel });
        }

        Ok(())
    }

    fn assert_quit_command_is_valid(&self, _trail: &Option<String>) -> Result<(), ErrorReply> {
        Ok(())
    }
}
