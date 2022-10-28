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
        parameters: Vec<String>,
        trailing: Option<String>,
    ) -> io::Result<()> {
        if let Some(error) = self.assert_privmsg_command_is_valid(&parameters, &trailing) {
            return self.send_response_for_error(error);
        }

        let content = trailing.unwrap();

        let targets = &parameters[0];
        for target in targets.split(',') {
            if let Some(error) = self.assert_target_is_valid(target) {
                self.send_response_for_error(error)?;
            }

            let message = self.build_text_message(PRIVMSG_COMMAND, target, &content);
            self.send_message_to(target, &message)?;
        }

        self.send_response_for_reply(CommandResponse::Ok200)
    }

    pub fn notice_command(
        &mut self,
        parameters: Vec<String>,
        trailing: Option<String>,
    ) -> io::Result<()> {
        if let Some(error) = self.assert_privmsg_command_is_valid(&parameters, &trailing) {
            return self.send_response_for_error(error);
        }

        let content = trailing.unwrap();

        let targets = &parameters[0];
        for target in targets.split(',') {
            if let Some(error) = self.assert_target_is_valid(target) {
                self.send_response_for_error(error)?;
            }

            let message = self.build_text_message(PRIVMSG_COMMAND, target, &content);
            self.send_message_to(target, &message)?;
        }

        self.send_response_for_reply(CommandResponse::Ok200)
    }
}
