use crate::server::connection::Connection;
use std::io;

use super::structure::CommandArgs;

pub trait ConnectionHandlerLogic<C: Connection> {
    fn pass_logic(&mut self, _arguments: CommandArgs) -> io::Result<bool> {
        Ok(true)
    }
    fn nick_logic(&mut self, _arguments: CommandArgs) -> io::Result<bool> {
        Ok(true)
    }
    fn user_logic(&mut self, _arguments: CommandArgs) -> io::Result<bool> {
        Ok(true)
    }
    fn oper_logic(&mut self, _arguments: CommandArgs) -> io::Result<bool> {
        Ok(true)
    }
    fn privmsg_logic(&mut self, _arguments: CommandArgs) -> io::Result<bool> {
        Ok(true)
    }
    fn notice_logic(&mut self, _arguments: CommandArgs) -> io::Result<bool> {
        Ok(true)
    }
    fn join_logic(&mut self, _arguments: CommandArgs) -> io::Result<bool> {
        Ok(true)
    }
    fn part_logic(&mut self, _arguments: CommandArgs) -> io::Result<bool> {
        Ok(true)
    }
    fn invite_logic(&mut self, _arguments: CommandArgs) -> io::Result<bool> {
        Ok(true)
    }
    fn names_logic(&mut self, _arguments: CommandArgs) -> io::Result<bool> {
        Ok(true)
    }
    fn list_logic(&mut self, _arguments: CommandArgs) -> io::Result<bool> {
        Ok(true)
    }
    fn who_logic(&mut self, _arguments: CommandArgs) -> io::Result<bool> {
        Ok(true)
    }
    fn whois_logic(&mut self, _arguments: CommandArgs) -> io::Result<bool> {
        Ok(true)
    }
    fn away_logic(&mut self, _arguments: CommandArgs) -> io::Result<bool> {
        Ok(true)
    }
    fn topic_logic(&mut self, _arguments: CommandArgs) -> io::Result<bool> {
        Ok(true)
    }
    fn kick_logic(&mut self, _arguments: CommandArgs) -> io::Result<bool> {
        Ok(true)
    }
    fn mode_logic(&mut self, _arguments: CommandArgs) -> io::Result<bool> {
        Ok(true)
    }
    fn quit_logic(&mut self, _arguments: CommandArgs) -> io::Result<bool> {
        Ok(true)
    }
    fn server_logic(&mut self, _arguments: CommandArgs) -> io::Result<bool> {
        Ok(true)
    }
    fn squit_logic(&mut self, _arguments: CommandArgs) -> io::Result<bool> {
        Ok(true)
    }
    fn ctcp_logic(&mut self, _arguments: CommandArgs) -> io::Result<bool> {
        Ok(true)
    }
}
