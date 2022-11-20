use crate::server::{connection::Connection, connection_handler::responses::ErrorReply};
use std::io;

use super::{ConnectionHandlerAsserts, ConnectionHandlerLogic, ConnectionHandlerUtils};

pub trait ConnectionHandlerCommands<C: Connection>:
    ConnectionHandlerAsserts<C> + ConnectionHandlerLogic<C> + ConnectionHandlerUtils<C>
{
    fn pass_command(&mut self, params: Vec<String>) -> io::Result<bool> {
        if let Err(error) = self.assert_pass_command_is_valid(&params) {
            self.send_response(&error)?;
            return Ok(true);
        }

        self.pass_logic(params)
    }
    fn nick_command(&mut self, params: Vec<String>) -> io::Result<bool> {
        if let Err(error) = self.assert_nick_command_is_valid(&params) {
            self.send_response(&error)?;
            return Ok(true);
        }

        self.nick_logic(params)
    }
    fn user_command(&mut self, params: Vec<String>, trail: Option<String>) -> io::Result<bool> {
        if let Err(error) = self.assert_user_command_is_valid(&params, &trail) {
            self.send_response(&error)?;
            return Ok(true);
        }

        self.user_logic(params, trail)
    }
    fn oper_command(&mut self, params: Vec<String>) -> io::Result<bool> {
        if let Err(error) = self.assert_oper_command_is_valid(&params) {
            self.send_response(&error)?;
            return Ok(true);
        }

        self.oper_logic(params)
    }

    fn privmsg_command(&mut self, params: Vec<String>, trail: Option<String>) -> io::Result<bool> {
        if let Err(error) = self.assert_privmsg_command_is_valid(&params, &trail) {
            self.send_response(&error)?;
            return Ok(true);
        }

        self.privmsg_logic(params, trail)
    }
    fn notice_command(&mut self, params: Vec<String>, trail: Option<String>) -> io::Result<bool> {
        if let Err(error) = self.assert_notice_command_is_valid(&params, &trail) {
            self.send_response(&error)?;
            return Ok(true);
        }

        self.notice_logic(params, trail)
    }

    fn join_command(&mut self, params: Vec<String>) -> io::Result<bool> {
        if let Err(error) = self.assert_join_command_is_valid(&params) {
            self.send_response(&error)?;
            return Ok(true);
        }

        self.join_logic(params)
    }
    fn part_command(&mut self, params: Vec<String>) -> io::Result<bool> {
        if let Err(error) = self.assert_part_command_is_valid(&params) {
            self.send_response(&error)?;
            return Ok(true);
        }

        self.part_logic(params)
    }
    fn invite_command(&mut self, params: Vec<String>) -> io::Result<bool> {
        if let Err(error) = self.assert_invite_command_is_valid(&params) {
            self.send_response(&error)?;
            return Ok(true);
        }

        self.invite_logic(params)
    }
    fn names_command(&mut self, params: Vec<String>) -> io::Result<bool> {
        if let Err(error) = self.assert_names_command_is_valid(&params) {
            self.send_response(&error)?;
            return Ok(true);
        }

        self.names_logic(params)
    }
    fn list_command(&mut self, params: Vec<String>) -> io::Result<bool> {
        if let Err(error) = self.assert_list_command_is_valid(&params) {
            self.send_response(&error)?;
            return Ok(true);
        }

        self.list_logic(params)
    }
    fn who_command(&mut self, params: Vec<String>) -> io::Result<bool> {
        if let Err(error) = self.assert_who_command_is_valid(&params) {
            self.send_response(&error)?;
            return Ok(true);
        }

        self.who_logic(params)
    }
    fn whois_command(&mut self, params: Vec<String>) -> io::Result<bool> {
        if let Err(error) = self.assert_whois_command_is_valid(&params) {
            self.send_response(&error)?;
            return Ok(true);
        }

        self.whois_logic(params)
    }
    fn away_command(&mut self, trail: Option<String>) -> io::Result<bool> {
        if let Err(error) = self.assert_away_command_is_valid(&trail) {
            self.send_response(&error)?;
            return Ok(true);
        }

        self.away_logic(trail)
    }
    fn topic_command(&mut self, params: Vec<String>) -> io::Result<bool> {
        if let Err(error) = self.assert_topic_command_is_valid(&params) {
            self.send_response(&error)?;
            return Ok(true);
        }

        self.topic_logic(params)
    }
    fn kick_command(&mut self, params: Vec<String>, trail: Option<String>) -> io::Result<bool> {
        if let Err(error) = self.assert_kick_command_is_valid(&params, &trail) {
            self.send_response(&error)?;
            return Ok(true);
        }

        self.kick_logic(params, trail)
    }

    fn mode_command(&mut self, params: Vec<String>) -> io::Result<bool> {
        if let Err(error) = self.assert_mode_command_is_valid(&params) {
            self.send_response(&error)?;
            return Ok(true);
        }

        self.mode_logic(params)
    }

    fn quit_command(&mut self, trail: Option<String>) -> io::Result<bool> {
        if let Err(error) = self.assert_quit_command_is_valid(&trail) {
            self.send_response(&error)?;
            return Ok(true);
        }

        self.quit_logic(trail)
    }

    fn server_command(&mut self, params: Vec<String>, trail: Option<String>) -> io::Result<bool> {
        if let Err(error) = self.assert_server_command_is_valid(&params, &trail) {
            self.send_response(&error)?;
            return Ok(true);
        }

        self.server_logic(params, trail)
    }

    fn squit_command(&mut self, params: Vec<String>, trail: Option<String>) -> io::Result<bool> {
        if let Err(error) = self.assert_squit_command_is_valid(&params) {
            self.send_response(&error)?;
            return Ok(true);
        }

        self.squit_logic(params, trail)
    }

    fn on_unknown_command(&mut self, command: String) -> io::Result<bool> {
        self.send_response(&ErrorReply::UnknownCommand421 { command })?;
        Ok(true)
    }
}
