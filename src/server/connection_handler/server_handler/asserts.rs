use crate::server::connection::Connection;
use crate::server::connection_handler::connection_handler_trait::ConnectionHandlerAsserts;
use crate::server::connection_handler::responses::ErrorReply;

use super::ServerHandler;

impl<C: Connection> ConnectionHandlerAsserts<C> for ServerHandler<C> {
    fn assert_pass_command_is_valid(&self, _params: &[String]) -> Result<(), ErrorReply> {
        todo!()
    }

    fn assert_nick_command_is_valid(&self, _params: &[String]) -> Result<(), ErrorReply> {
        todo!()
    }

    fn assert_user_command_is_valid(
        &self,
        _params: &[String],
        _trail: &Option<String>,
    ) -> Result<(), ErrorReply> {
        todo!()
    }

    fn assert_oper_command_is_valid(&self, _params: &[String]) -> Result<(), ErrorReply> {
        todo!()
    }

    fn assert_privmsg_command_is_valid(
        &self,
        _params: &[String],
        _trail: &Option<String>,
    ) -> Result<(), ErrorReply> {
        todo!()
    }

    fn assert_notice_command_is_valid(
        &self,
        _params: &[String],
        _trail: &Option<String>,
    ) -> Result<(), ErrorReply> {
        todo!()
    }

    fn assert_join_command_is_valid(&self, _params: &[String]) -> Result<(), ErrorReply> {
        todo!()
    }

    fn assert_part_command_is_valid(&self, _params: &[String]) -> Result<(), ErrorReply> {
        todo!()
    }

    fn assert_invite_command_is_valid(&self, _params: &[String]) -> Result<(), ErrorReply> {
        todo!()
    }

    fn assert_names_command_is_valid(&self, _params: &[String]) -> Result<(), ErrorReply> {
        todo!()
    }

    fn assert_list_command_is_valid(&self, _params: &[String]) -> Result<(), ErrorReply> {
        todo!()
    }

    fn assert_who_command_is_valid(&self, _params: &[String]) -> Result<(), ErrorReply> {
        todo!()
    }

    fn assert_whois_command_is_valid(&self, _params: &[String]) -> Result<(), ErrorReply> {
        todo!()
    }

    fn assert_away_command_is_valid(&self, _trail: &Option<String>) -> Result<(), ErrorReply> {
        todo!()
    }

    fn assert_topic_command_is_valid(&self, _params: &[String]) -> Result<(), ErrorReply> {
        todo!()
    }

    fn assert_kick_command_is_valid(
        &self,
        _params: &[String],
        _trail: &Option<String>,
    ) -> Result<(), ErrorReply> {
        todo!()
    }

    fn assert_mode_command_is_valid(&self, _params: &[String]) -> Result<(), ErrorReply> {
        todo!()
    }

    fn assert_quit_command_is_valid(&self, _trail: &Option<String>) -> Result<(), ErrorReply> {
        todo!()
    }
}
