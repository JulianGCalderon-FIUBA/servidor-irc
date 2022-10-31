mod utils;
mod validations;

use crate::server::{
    client_handler::responses::replies::CommandResponse, client_trait::ClientTrait, ClientHandler,
};
use std::io;

pub const NOTICE_COMMAND: &str = "NOTICE";
pub const PRIVMSG_COMMAND: &str = "PRIVMSG";

impl<T: ClientTrait> ClientHandler<T> {
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

        self.send_message_for_command(PRIVMSG_COMMAND, targets, content)
    }

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

        self.send_message_for_command(NOTICE_COMMAND, targets, content)
    }

    fn send_message_for_command(
        &mut self,
        command: &str,
        targets: String,
        content: String,
    ) -> io::Result<()> {
        for target in targets.split(',') {
            if let Some(error) = self.assert_target_is_valid(target) {
                self.send_response_for_error(error)?;
                continue;
            }

            let message = self.message_for_command(command, target, &content);
            self.send_message_to_target(&message, target)?;

            // if command == PRIVMSG_COMMAND && self.database.contains_client(target) {
            //     self.away_response_for_client(target);
            // }
        }

        self.send_response_for_reply(CommandResponse::Ok200)
    }
}
