use crate::server::{connection::Connection, connection_handler::responses::ErrorReply};

pub trait ConnectionHandlerAsserts<C: Connection> {
    fn assert_pass_command_is_valid(&self, _parameters: &[String]) -> Result<(), ErrorReply> {
        Ok(())
    }
    fn assert_nick_command_is_valid(&self, _parameters: &[String]) -> Result<(), ErrorReply> {
        Ok(())
    }
    fn assert_user_command_is_valid(
        &self,
        _parameters: &[String],
        _trailing: &Option<String>,
    ) -> Result<(), ErrorReply> {
        Ok(())
    }
}
