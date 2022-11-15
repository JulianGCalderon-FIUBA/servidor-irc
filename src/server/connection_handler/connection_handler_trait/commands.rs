use crate::server::connection::Connection;
use std::io;

pub trait ConnectionHandlerCommands<C: Connection> {
    fn pass_command(&mut self, parameters: Vec<String>) -> io::Result<()>;
    fn nick_command(&mut self, parameters: Vec<String>) -> io::Result<()>;
    fn user_command(&mut self, parameters: Vec<String>, trailing: Option<String>)
        -> io::Result<()>;
    fn oper_command(&mut self, parameters: Vec<String>) -> io::Result<()>;
    fn privmsg_command(
        &mut self,
        parameters: Vec<String>,
        trailing: Option<String>,
    ) -> io::Result<()>;
    fn notice_command(
        &mut self,
        parameters: Vec<String>,
        trailing: Option<String>,
    ) -> io::Result<()>;

    fn join_command(&mut self, parameters: Vec<String>) -> io::Result<()>;
    fn part_command(&mut self, parameters: Vec<String>) -> io::Result<()>;
    fn invite_command(&mut self, parameters: Vec<String>) -> io::Result<()>;
    fn names_command(&mut self, parameters: Vec<String>) -> io::Result<()>;
    fn list_command(&mut self, parameters: Vec<String>) -> io::Result<()>;
    fn who_command(&mut self, parameters: Vec<String>) -> io::Result<()>;
    fn whois_command(&mut self, parameters: Vec<String>) -> io::Result<()>;
    fn away_command(&mut self, trailing: Option<String>) -> io::Result<()>;
    fn topic_command(&mut self, parameters: Vec<String>) -> io::Result<()>;
    fn kick_command(&mut self, parameters: Vec<String>, trailing: Option<String>)
        -> io::Result<()>;
    fn mode_command(&mut self, parameters: Vec<String>) -> io::Result<()>;
    fn quit_command(&mut self, trailing: Option<String>) -> io::Result<()>;
    fn on_unknown_command(&mut self, command: String) -> io::Result<()>;
}
