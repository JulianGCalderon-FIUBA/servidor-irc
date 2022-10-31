mod utils;
mod validations;

use crate::server::client_trait::ClientTrait;
use crate::server::ClientHandler;
use std::io;

pub const NOTICE_COMMAND: &str = "NOTICE";
pub const PRIVMSG_COMMAND: &str = "PRIVMSG";

impl<T: ClientTrait> ClientHandler<T> {
    pub fn privmsg_command(
        &mut self,
        parameters: Vec<String>,
        trailing: Option<String>,
    ) -> io::Result<()> {
        self.message_command(PRIVMSG_COMMAND, parameters, trailing)
    }

    pub fn notice_command(
        &mut self,
        parameters: Vec<String>,
        trailing: Option<String>,
    ) -> io::Result<()> {
        self.message_command(NOTICE_COMMAND, parameters, trailing)
    }

    fn message_command(
        &mut self,
        command: &str,
        mut parameters: Vec<String>,
        trailing: Option<String>,
    ) -> io::Result<()> {
        if let Some(error) = self.assert_message_command_is_valid(command, &parameters, &trailing) {
            return self.send_response_for_error(error);
        }
        let content = trailing.unwrap();
        let targets = parameters.pop().unwrap();
        self.message_command_to_targets(command, targets, content)
    }
}
