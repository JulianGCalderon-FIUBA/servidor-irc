/// This module contains validations for sending messages operations.
mod validations;

use crate::server::ClientHandler;
use crate::server::{
    client_handler::responses::notifications::Notification, client_trait::Connection,
};
use std::io;

use super::{NOTICE_COMMAND, PRIVMSG_COMMAND};

impl<C: Connection> ClientHandler<C> {
    /// Sends private message to client.
    pub fn privmsg_command(
        &mut self,
        mut parameters: Vec<String>,
        trailing: Option<String>,
    ) -> io::Result<()> {
        if let Some(error) =
            self.assert_message_command_is_valid(PRIVMSG_COMMAND, &parameters, &trailing)
        {
            return self.send_response_for_error(error);
        }
        let content = trailing.unwrap();
        let targets = parameters.pop().unwrap();

        for target in targets.split(',') {
            if let Some(error) = self.assert_target_is_valid(target) {
                self.send_response_for_error(error)?;
                continue;
            }

            let nickname = self.registration.nickname().unwrap();
            let notification = Notification::Privmsg {
                prefix: nickname,
                target: target.to_string(),
                message: content.clone(),
            };

            self.send_message_to_target(&notification.to_string(), target)?;

            // if self.database.contains_client(target) {
            //     self.away_response_for_client(target);
            // }
        }

        Ok(())
    }
    /// Sends notice to targets.
    pub fn notice_command(
        &mut self,
        mut parameters: Vec<String>,
        trailing: Option<String>,
    ) -> io::Result<()> {
        if let Some(error) =
            self.assert_message_command_is_valid(NOTICE_COMMAND, &parameters, &trailing)
        {
            return self.send_response_for_error(error);
        }
        let content = trailing.unwrap();
        let targets = parameters.pop().unwrap();

        for target in targets.split(',') {
            if let Some(error) = self.assert_target_is_valid(target) {
                self.send_response_for_error(error)?;
                continue;
            }

            let nickname = self.registration.nickname().unwrap();
            let notification = Notification::Notice {
                prefix: nickname,
                target: target.to_string(),
                message: content.clone(),
            };

            self.send_message_to_target(&notification.to_string(), target)?;
        }
        Ok(())
    }
}
