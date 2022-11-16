use crate::server::connection::Connection;
use crate::server::connection_handler::connection_handler_trait::{
    ConnectionHandlerLogic, ConnectionHandlerUtils,
};
use crate::server::connection_handler::responses::{CommandResponse, ErrorReply, Notification};

use super::ClientHandler;

impl<C: Connection> ConnectionHandlerLogic<C> for ClientHandler<C> {
    fn oper_logic(&mut self, _params: Vec<String>) -> std::io::Result<()> {
        self.database.set_server_operator(&self.nickname);

        self.send_response(&CommandResponse::YouAreOper381)
    }

    fn privmsg_logic(
        &mut self,
        mut params: Vec<String>,
        trail: Option<String>,
    ) -> std::io::Result<()> {
        let content = trail.unwrap();
        let targets = params.pop().unwrap();

        for target in targets.split(',') {
            if let Err(error) = self.assert_target_is_valid(target) {
                self.send_response(&error)?;
                continue;
            }

            self.send_privmsg_to_target(target, &content)?;
        }

        Ok(())
    }

    fn notice_logic(
        &mut self,
        mut params: Vec<String>,
        trail: Option<String>,
    ) -> std::io::Result<()> {
        let content = trail.unwrap();
        let targets = params.pop().unwrap();

        for target in targets.split(',') {
            if let Err(error) = self.assert_target_is_valid(target) {
                self.send_response(&error)?;
                continue;
            }

            self.send_notice_to_target(target, &content)?;
        }

        Ok(())
    }

    fn join_logic(&mut self, _params: Vec<String>) -> std::io::Result<()> {
        Ok(())
    }

    fn part_logic(&mut self, _params: Vec<String>) -> std::io::Result<()> {
        Ok(())
    }

    fn invite_logic(&mut self, _params: Vec<String>) -> std::io::Result<()> {
        Ok(())
    }

    fn names_logic(&mut self, _params: Vec<String>) -> std::io::Result<()> {
        Ok(())
    }

    fn list_logic(&mut self, _params: Vec<String>) -> std::io::Result<()> {
        Ok(())
    }

    fn who_logic(&mut self, _params: Vec<String>) -> std::io::Result<()> {
        Ok(())
    }

    fn whois_logic(&mut self, _params: Vec<String>) -> std::io::Result<()> {
        Ok(())
    }

    fn away_logic(&mut self, trail: Option<String>) -> std::io::Result<()> {
        self.database.set_away_message(&trail, &self.nickname);

        let reply = match trail {
            Some(_) => CommandResponse::NowAway,
            None => CommandResponse::UnAway,
        };

        self.send_response(&reply)
    }

    fn topic_logic(&mut self, _params: Vec<String>) -> std::io::Result<()> {
        Ok(())
    }

    fn kick_logic(&mut self, _params: Vec<String>, _trail: Option<String>) -> std::io::Result<()> {
        Ok(())
    }

    fn mode_logic(&mut self, _params: Vec<String>) -> std::io::Result<()> {
        Ok(())
    }

    fn quit_logic(&mut self, trail: Option<String>) -> std::io::Result<()> {
        let message = trail.unwrap_or_else(|| self.nickname.clone());

        let notification = Notification::Quit { message };

        self.database.disconnect_client(&self.nickname);
        let channels = self.database.get_channels_for_client(&self.nickname);
        for channel in channels {
            self.send_message_to_channel(&channel, &notification.to_string());
        }

        self.send_response(&notification.to_string())
    }
}

impl<C: Connection> ClientHandler<C> {
    fn send_privmsg_to_target(
        &mut self,
        target: &str,
        content: &str,
    ) -> Result<(), std::io::Error> {
        let nickname = self.nickname.clone();
        let notification = Notification::Privmsg {
            prefix: nickname,
            target: target.to_string(),
            message: content.to_owned(),
        };

        self.send_message_to_target(&notification.to_string(), target)?;

        if self.database.contains_client(target) {
            if let Some(message) = self.database.get_away_message(target) {
                let nickname = target.to_string();
                let reply = CommandResponse::Away { nickname, message };
                self.send_response(&reply)?;
            }
        }

        Ok(())
    }

    fn send_notice_to_target(&mut self, target: &str, content: &str) -> Result<(), std::io::Error> {
        let nickname = self.nickname.clone();
        let notification = Notification::Notice {
            prefix: nickname,
            target: target.to_string(),
            message: content.to_owned(),
        };

        self.send_message_to_target(&notification.to_string(), target)?;

        Ok(())
    }

    fn assert_target_is_valid(&self, target: &str) -> Result<(), ErrorReply> {
        let target = target.to_string();
        let is_client = self.database.contains_client(&target);
        let is_channel = self.database.contains_channel(&target);

        if !(is_client || is_channel) {
            return Err(ErrorReply::NoSuchNickname401 { nickname: target });
        }

        if self.database.channel_has_mode(&target, 'n')
            && !self.database.is_client_in_channel(&self.nickname, &target)
        {
            return Err(ErrorReply::CannotSendToChannel404 { channel: target });
        }

        if self.database.channel_has_mode(&target, 'm')
            && !self.database.is_channel_speaker(&target, &self.nickname)
        {
            return Err(ErrorReply::CannotSendToChannel404 { channel: target });
        }

        Ok(())
    }
}
