use std::io;

use crate::server::{
    connection::Connection, connection_handler::client_handler::ClientHandler,
    responses::CommandResponse,
};

impl<C: Connection> ClientHandler<C> {
    pub(super) fn send_privmsg_to_target(&mut self, target: &str, content: &str) -> io::Result<()> {
        self.send_privmsg_notification(target, content)?;

        if let Ok(Some(message)) = self.database.get_away_message(target) {
            self.stream.send(&CommandResponse::away(target, &message))?;
        }

        Ok(())
    }

    pub(super) fn send_notice_to_target(&mut self, target: &str, content: &str) -> io::Result<()> {
        self.send_notice_notification(target, content)?;

        Ok(())
    }

    pub(super) fn kick_client_from_channel(
        &mut self,
        nickname: &str,
        channel: &str,
        comment: &Option<String>,
    ) {
        self.send_kick_notification(channel, nickname, comment);
        self.database.remove_client_from_channel(nickname, channel);
    }

    pub(super) fn add_modes(
        &mut self,
        add: Vec<char>,
        arguments: &mut Vec<String>,
        channel: &str,
    ) -> io::Result<()> {
        for mode in add {
            self.handle_add_mode(mode, channel, arguments)?;
        }
        Ok(())
    }

    pub(super) fn remove_modes(
        &mut self,
        remove: Vec<char>,
        arguments: &mut Vec<String>,
        channel: &str,
    ) -> io::Result<()> {
        for mode in remove {
            self.handle_remove_mode(mode, channel, arguments)?;
        }
        Ok(())
    }
}
