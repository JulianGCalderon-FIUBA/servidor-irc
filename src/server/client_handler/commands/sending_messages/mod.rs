mod utils;
mod validations;

use crate::server::ClientHandler;
use std::io::{self, Read, Write};

pub const NOTICE_COMMAND: &str = "NOTICE";
pub const PRIVMSG_COMMAND: &str = "PRIVMSG";

impl<T: Read + Write> ClientHandler<T> {
    pub fn privmsg_command(
        &mut self,
        parameters: Vec<String>,
        trailing: Option<String>,
    ) -> io::Result<()> {
        if !self.validate_privmsg_command(&parameters, &trailing)? {
            return Ok(());
        }

        let content = trailing.unwrap();

        let targets = &parameters[0];
        for target in targets.split(',') {
            let message = self.build_text_message(PRIVMSG_COMMAND, target, &content);
            self.send_message_to(target, &message)?;
            if self.database.contains_client(target) {
                // let away = self.database.away_message_from_client(target);
                // if let Some(away) = away {
                //     self.away_reply(target, away)?;
                // }
            }
        }

        self.ok_reply()
    }

    pub fn notice_command(
        &mut self,
        parameters: Vec<String>,
        trailing: Option<String>,
    ) -> io::Result<()> {
        if !self.validate_privmsg_command(&parameters, &trailing)? {
            return Ok(());
        }

        let content = trailing.unwrap();

        let targets = &parameters[0];
        for target in targets.split(',') {
            let message = self.build_text_message(NOTICE_COMMAND, target, &content);
            self.send_message_to(target, &message)?;
        }
        Ok(())
    }
}
