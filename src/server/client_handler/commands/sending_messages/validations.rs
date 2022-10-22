use super::ClientHandler;
use std::io;
// use std::sync::mpsc::channel;

use super::PRIVMSG_COMMAND;

impl ClientHandler {
    // GENERAL

    pub fn validate_targets(&mut self, parameters: &[String]) -> io::Result<bool> {
        let mut valid = true;
        let targets = &parameters[0];
        for target in targets.split(',') {
            let is_client = self.database.contains_client(target);
            let is_channel = self.database.get_channels().contains(&target.to_string());

            if !(is_client || is_channel) {
                self.no_such_nickname_error(target)?;
                valid = false;
            }

            if is_channel {
                let clients = self.database.get_clients(target);
                if !clients.contains(self.connection.nickname.as_ref().unwrap()) {
                    self.cannot_send_to_chan_error(target)?;
                    valid = false;
                }
            }
        }

        Ok(valid)
    }

    // COMMANDS

    pub fn validate_privmsg_command(
        &mut self,
        parameters: &Vec<String>,
        trailing: &Option<String>,
    ) -> io::Result<bool> {
        if parameters.is_empty() {
            self.no_recipient_error(PRIVMSG_COMMAND)?;
            return Ok(false);
        }

        if trailing.is_none() {
            self.no_text_to_send_error()?;
            return Ok(false);
        }

        if !self.validate_targets(parameters)? {
            return Ok(false);
        };

        Ok(true)
    }
}
