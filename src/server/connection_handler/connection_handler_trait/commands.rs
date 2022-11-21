use crate::server::{connection::Connection, responses::ErrorReply};
use std::io;

use super::{
    structure::CommandArgs, ConnectionHandlerAsserts, ConnectionHandlerLogic,
    ConnectionHandlerUtils,
};

pub trait ConnectionHandlerCommands<C: Connection>:
    ConnectionHandlerAsserts<C> + ConnectionHandlerLogic<C> + ConnectionHandlerUtils<C>
{
    fn pass_command(&mut self, arguments: CommandArgs) -> io::Result<bool> {
        if let Err(error) = self.assert_pass_command_is_valid(&arguments) {
            self.send_response(&error)?;
            return Ok(true);
        }

        self.pass_logic(arguments)
    }
    fn nick_command(&mut self, arguments: CommandArgs) -> io::Result<bool> {
        if let Err(error) = self.assert_nick_command_is_valid(&arguments) {
            self.send_response(&error)?;
            return Ok(true);
        }

        self.nick_logic(arguments)
    }
    fn user_command(&mut self, arguments: CommandArgs) -> io::Result<bool> {
        if let Err(error) = self.assert_user_command_is_valid(&arguments) {
            self.send_response(&error)?;
            return Ok(true);
        }

        self.user_logic(arguments)
    }
    fn oper_command(&mut self, arguments: CommandArgs) -> io::Result<bool> {
        if let Err(error) = self.assert_oper_command_is_valid(&arguments) {
            self.send_response(&error)?;
            return Ok(true);
        }

        self.oper_logic(arguments)
    }

    fn privmsg_command(&mut self, arguments: CommandArgs) -> io::Result<bool> {
        if let Err(error) = self.assert_privmsg_command_is_valid(&arguments) {
            self.send_response(&error)?;
            return Ok(true);
        }

        self.privmsg_logic(arguments)
    }
    fn notice_command(&mut self, arguments: CommandArgs) -> io::Result<bool> {
        if let Err(error) = self.assert_notice_command_is_valid(&arguments) {
            self.send_response(&error)?;
            return Ok(true);
        }

        self.notice_logic(arguments)
    }

    fn join_command(&mut self, arguments: CommandArgs) -> io::Result<bool> {
        if let Err(error) = self.assert_join_command_is_valid(&arguments) {
            self.send_response(&error)?;
            return Ok(true);
        }

        self.join_logic(arguments)
    }
    fn part_command(&mut self, arguments: CommandArgs) -> io::Result<bool> {
        if let Err(error) = self.assert_part_command_is_valid(&arguments) {
            self.send_response(&error)?;
            return Ok(true);
        }

        self.part_logic(arguments)
    }
    fn invite_command(&mut self, arguments: CommandArgs) -> io::Result<bool> {
        if let Err(error) = self.assert_invite_command_is_valid(&arguments) {
            self.send_response(&error)?;
            return Ok(true);
        }

        self.invite_logic(arguments)
    }
    fn names_command(&mut self, arguments: CommandArgs) -> io::Result<bool> {
        if let Err(error) = self.assert_names_command_is_valid(&arguments) {
            self.send_response(&error)?;
            return Ok(true);
        }

        self.names_logic(arguments)
    }
    fn list_command(&mut self, arguments: CommandArgs) -> io::Result<bool> {
        if let Err(error) = self.assert_list_command_is_valid(&arguments) {
            self.send_response(&error)?;
            return Ok(true);
        }

        self.list_logic(arguments)
    }
    fn who_command(&mut self, arguments: CommandArgs) -> io::Result<bool> {
        if let Err(error) = self.assert_who_command_is_valid(&arguments) {
            self.send_response(&error)?;
            return Ok(true);
        }

        self.who_logic(arguments)
    }
    fn whois_command(&mut self, arguments: CommandArgs) -> io::Result<bool> {
        if let Err(error) = self.assert_whois_command_is_valid(&arguments) {
            self.send_response(&error)?;
            return Ok(true);
        }

        self.whois_logic(arguments)
    }
    fn away_command(&mut self, arguments: CommandArgs) -> io::Result<bool> {
        if let Err(error) = self.assert_away_command_is_valid(&arguments) {
            self.send_response(&error)?;
            return Ok(true);
        }

        self.away_logic(arguments)
    }
    fn topic_command(&mut self, arguments: CommandArgs) -> io::Result<bool> {
        if let Err(error) = self.assert_topic_command_is_valid(&arguments) {
            self.send_response(&error)?;
            return Ok(true);
        }

        self.topic_logic(arguments)
    }
    fn kick_command(&mut self, arguments: CommandArgs) -> io::Result<bool> {
        if let Err(error) = self.assert_kick_command_is_valid(&arguments) {
            self.send_response(&error)?;
            return Ok(true);
        }

        self.kick_logic(arguments)
    }

    fn mode_command(&mut self, arguments: CommandArgs) -> io::Result<bool> {
        if let Err(error) = self.assert_mode_command_is_valid(&arguments) {
            self.send_response(&error)?;
            return Ok(true);
        }

        self.mode_logic(arguments)
    }

    fn quit_command(&mut self, arguments: CommandArgs) -> io::Result<bool> {
        if let Err(error) = self.assert_quit_command_is_valid(&arguments) {
            self.send_response(&error)?;
            return Ok(true);
        }

        self.quit_logic(arguments)
    }

    fn server_command(&mut self, arguments: CommandArgs) -> io::Result<bool> {
        if let Err(error) = self.assert_server_command_is_valid(&arguments) {
            self.send_response(&error)?;
            return Ok(true);
        }

        self.server_logic(arguments)
    }

    fn squit_command(&mut self, arguments: CommandArgs) -> io::Result<bool> {
        if let Err(error) = self.assert_squit_command_is_valid(&arguments) {
            self.send_response(&error)?;
            return Ok(true);
        }

        self.squit_logic(arguments)
    }

    fn on_unknown_command(&mut self, command: String) -> io::Result<bool> {
        self.send_response(&ErrorReply::UnknownCommand421 { command })?;
        Ok(true)
    }
}
