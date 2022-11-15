use crate::server::connection::Connection;
use std::io;

pub trait ConnectionHandlerCommands<C: Connection> {
    fn pass_command(&mut self, _parameters: Vec<String>) -> io::Result<()> {
        Ok(())
    }
    fn nick_command(&mut self, _parameters: Vec<String>) -> io::Result<()> {
        Ok(())
    }
    fn user_command(
        &mut self,
        _parameters: Vec<String>,
        _trailing: Option<String>,
    ) -> io::Result<()> {
        Ok(())
    }
    fn oper_command(&mut self, _parameters: Vec<String>) -> io::Result<()> {
        Ok(())
    }
    fn privmsg_command(
        &mut self,
        _parameters: Vec<String>,
        _trailing: Option<String>,
    ) -> io::Result<()> {
        Ok(())
    }
    fn notice_command(
        &mut self,
        _parameters: Vec<String>,
        _trailing: Option<String>,
    ) -> io::Result<()> {
        Ok(())
    }

    fn join_command(&mut self, _parameters: Vec<String>) -> io::Result<()> {
        Ok(())
    }
    fn part_command(&mut self, _parameters: Vec<String>) -> io::Result<()> {
        Ok(())
    }
    fn invite_command(&mut self, _parameters: Vec<String>) -> io::Result<()> {
        Ok(())
    }
    fn names_command(&mut self, _parameters: Vec<String>) -> io::Result<()> {
        Ok(())
    }
    fn list_command(&mut self, _parameters: Vec<String>) -> io::Result<()> {
        Ok(())
    }
    fn who_command(&mut self, _parameters: Vec<String>) -> io::Result<()> {
        Ok(())
    }
    fn whois_command(&mut self, _parameters: Vec<String>) -> io::Result<()> {
        Ok(())
    }
    fn away_command(&mut self, _trailing: Option<String>) -> io::Result<()> {
        Ok(())
    }
    fn topic_command(&mut self, _parameters: Vec<String>) -> io::Result<()> {
        Ok(())
    }
    fn kick_command(
        &mut self,
        _parameters: Vec<String>,
        _trailing: Option<String>,
    ) -> io::Result<()> {
        Ok(())
    }
    fn mode_command(&mut self, _parameters: Vec<String>) -> io::Result<()> {
        Ok(())
    }
    fn quit_command(&mut self, _trailing: Option<String>) -> io::Result<()> {
        Ok(())
    }
    fn on_unknown_command(&mut self, _command: String) -> io::Result<()> {
        Ok(())
    }
}
