use crate::server::{connection::Connection, connection_handler::responses::ErrorReply};

pub trait ConnectionHandlerAsserts<C: Connection> {
    fn assert_pass_command_is_valid(&self, params: &[String]) -> Result<(), ErrorReply>;
    fn assert_nick_command_is_valid(&self, params: &[String]) -> Result<(), ErrorReply>;
    fn assert_user_command_is_valid(
        &self,
        params: &[String],
        trail: &Option<String>,
    ) -> Result<(), ErrorReply>;
    fn assert_oper_command_is_valid(&self, params: &[String]) -> Result<(), ErrorReply>;
    fn assert_privmsg_command_is_valid(
        &self,
        params: &[String],
        trail: &Option<String>,
    ) -> Result<(), ErrorReply>;
    fn assert_notice_command_is_valid(
        &self,
        params: &[String],
        trail: &Option<String>,
    ) -> Result<(), ErrorReply>;
    fn assert_join_command_is_valid(&self, params: &[String]) -> Result<(), ErrorReply>;
    fn assert_part_command_is_valid(&self, params: &[String]) -> Result<(), ErrorReply>;
    fn assert_invite_command_is_valid(&self, params: &[String]) -> Result<(), ErrorReply>;
    fn assert_names_command_is_valid(&self, params: &[String]) -> Result<(), ErrorReply>;
    fn assert_list_command_is_valid(&self, params: &[String]) -> Result<(), ErrorReply>;
    fn assert_who_command_is_valid(&self, params: &[String]) -> Result<(), ErrorReply>;
    fn assert_whois_command_is_valid(&self, params: &[String]) -> Result<(), ErrorReply>;
    fn assert_away_command_is_valid(&self, trail: &Option<String>) -> Result<(), ErrorReply>;
    fn assert_topic_command_is_valid(&self, params: &[String]) -> Result<(), ErrorReply>;
    fn assert_kick_command_is_valid(
        &self,
        params: &[String],
        trail: &Option<String>,
    ) -> Result<(), ErrorReply>;
    fn assert_mode_command_is_valid(&self, params: &[String]) -> Result<(), ErrorReply>;
    fn assert_quit_command_is_valid(&self, trail: &Option<String>) -> Result<(), ErrorReply>;
}
