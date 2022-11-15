use crate::server::connection::Connection;
use std::io;

pub trait ConnectionHandlerLogic<C: Connection> {
    fn pass_logic(&mut self, _params: Vec<String>) -> io::Result<()> {
        Ok(())
    }
    fn nick_logic(&mut self, _params: Vec<String>) -> io::Result<()> {
        Ok(())
    }
    fn user_logic(&mut self, _params: Vec<String>, _trail: Option<String>) -> io::Result<()> {
        Ok(())
    }
    fn oper_logic(&mut self, _params: Vec<String>) -> io::Result<()> {
        Ok(())
    }
    fn privmsg_logic(&mut self, _params: Vec<String>, _trail: Option<String>) -> io::Result<()> {
        Ok(())
    }
    fn notice_logic(&mut self, _params: Vec<String>, _trail: Option<String>) -> io::Result<()> {
        Ok(())
    }
    fn join_logic(&mut self, _params: Vec<String>) -> io::Result<()> {
        Ok(())
    }
    fn part_logic(&mut self, _params: Vec<String>) -> io::Result<()> {
        Ok(())
    }
    fn invite_logic(&mut self, _params: Vec<String>) -> io::Result<()> {
        Ok(())
    }
    fn names_logic(&mut self, _params: Vec<String>) -> io::Result<()> {
        Ok(())
    }
    fn list_logic(&mut self, _params: Vec<String>) -> io::Result<()> {
        Ok(())
    }
    fn who_logic(&mut self, _params: Vec<String>) -> io::Result<()> {
        Ok(())
    }
    fn whois_logic(&mut self, _params: Vec<String>) -> io::Result<()> {
        Ok(())
    }
    fn away_logic(&mut self, _trail: Option<String>) -> io::Result<()> {
        Ok(())
    }
    fn topic_logic(&mut self, _params: Vec<String>) -> io::Result<()> {
        Ok(())
    }
    fn kick_logic(&mut self, _params: Vec<String>, _trail: Option<String>) -> io::Result<()> {
        Ok(())
    }
    fn mode_logic(&mut self, _params: Vec<String>) -> io::Result<()> {
        Ok(())
    }
    fn quit_logic(&mut self, _trail: Option<String>) -> io::Result<()> {
        Ok(())
    }
}
