use std::io;

use crate::server::connection::Connection;
use crate::server::connection_handler::connection_handler_trait::{
    ConnectionHandlerLogic, ConnectionHandlerUtils,
};

use crate::server::connection_handler::consts::modes::*;
use crate::server::connection_handler::responses::{CommandResponse, ErrorReply, Notification};
use crate::server::database::ClientInfo;

use self::utils::{collect_list, parse_modes};

use super::ClientHandler;

mod auxiliars;
mod booleans;
mod mode_requests;
mod responses;
mod utils;

impl<C: Connection> ConnectionHandlerLogic<C> for ClientHandler<C> {
    fn nick_logic(&mut self, params: Vec<String>) -> io::Result<bool> {
        let new_nickname = params[0].clone();
        self.database.update_nickname(&self.nickname, &new_nickname);
        self.nickname = new_nickname;

        Ok(true)
    }

    fn oper_logic(&mut self, _params: Vec<String>) -> std::io::Result<bool> {
        self.database.set_server_operator(&self.nickname);

        self.send_response(&CommandResponse::YouAreOper381)?;

        Ok(true)
    }

    fn privmsg_logic(
        &mut self,
        mut params: Vec<String>,
        trail: Option<String>,
    ) -> std::io::Result<bool> {
        let content = trail.unwrap();
        let targets = params.remove(0);

        for target in targets.split(',') {
            if let Err(error) = self.assert_target_is_valid(target) {
                self.send_response(&error)?;
                continue;
            }

            self.send_privmsg_to_target(target, &content)?;
        }

        Ok(true)
    }

    fn notice_logic(
        &mut self,
        mut params: Vec<String>,
        trail: Option<String>,
    ) -> std::io::Result<bool> {
        let content = trail.unwrap();
        let targets = params.pop().unwrap();

        for target in targets.split(',') {
            if let Err(error) = self.assert_target_is_valid(target) {
                self.send_response(&error)?;
                continue;
            }

            self.send_notice_to_target(target, &content)?;
        }

        Ok(true)
    }

    fn join_logic(&mut self, params: Vec<String>) -> std::io::Result<bool> {
        let channels = params[0].split(',');

        let mut keys = collect_list(params.get(1)).into_iter();

        for channel in channels {
            let key = keys.next();

            if let Err(error) = self.assert_can_join_channel(channel, &key) {
                self.send_response(&error)?;
                continue;
            }

            let notification = Notification::Join {
                nickname: self.nickname.clone(),
                channel: channel.to_string(),
            };
            self.send_message_to_channel(&notification, channel);

            self.database.add_client_to_channel(&self.nickname, channel);

            self.send_join_response(channel)?;
        }

        Ok(true)
    }

    fn part_logic(&mut self, mut _params: Vec<String>) -> std::io::Result<bool> {
        let channels = _params.pop().unwrap();

        for channel in channels.split(',') {
            if let Err(error) = self.assert_can_part_channel(channel) {
                self.send_response(&error)?;
                continue;
            }

            let notification = Notification::Part {
                nickname: self.nickname.clone(),
                channel: channel.to_string(),
            };
            self.send_message_to_channel(&notification, channel);

            self.database
                .remove_client_from_channel(&self.nickname, channel);
        }

        Ok(true)
    }

    fn invite_logic(&mut self, mut params: Vec<String>) -> std::io::Result<bool> {
        let channel = params.pop().unwrap();
        let invited_client = params.pop().unwrap();
        let inviting_client = self.nickname.clone();

        let invitation = Notification::Invite {
            inviting_client: inviting_client.clone(),
            invited_client: invited_client.clone(),
            channel: channel.clone(),
        };

        if self
            .send_message_to_client(&invitation, &invited_client)
            .is_ok()
        {
            let invite_response = CommandResponse::Inviting341 {
                nickname: inviting_client,
                channel,
            };
            self.send_response(&invite_response)?;
        } else {
            let no_such_nick_reply = ErrorReply::NoSuchNickname401 {
                nickname: invited_client.clone(),
            };
            self.send_response(&no_such_nick_reply)?;
        }

        Ok(true)
    }

    fn names_logic(&mut self, params: Vec<String>) -> std::io::Result<bool> {
        let channels = self.channels_to_list(params.get(0));

        for channel in channels {
            if !self.can_name_channel(&channel) {
                continue;
            }

            let clients = self.database.get_clients_for_channel(&channel);
            let name_reply = CommandResponse::NameReply353 {
                channel: channel.clone(),
                clients,
            };
            self.send_response(&name_reply)?;

            if !params.is_empty() {
                let end_of_names = CommandResponse::EndOfNames366 { channel };
                self.send_response(&end_of_names)?
            }
        }

        if params.is_empty() {
            let end_of_names = CommandResponse::EndOfNames366 {
                channel: "".to_string(),
            };
            self.send_response(&end_of_names)?;
        }

        Ok(true)
    }

    fn list_logic(&mut self, _params: Vec<String>) -> std::io::Result<bool> {
        let channels = self.channels_to_list(_params.get(0));

        self.send_response(&CommandResponse::ListStart321)?;

        for channel in channels {
            if self.can_list_channel(&channel) {
                let topic = match self.database.get_topic_for_channel(&channel) {
                    Some(topic) => topic,
                    None => "No topic set".to_string(),
                };

                let prv = self.database.channel_has_mode(&channel, PRIVATE)
                    && !self.database.is_client_in_channel(&self.nickname, &channel);

                self.send_response(&CommandResponse::List322 {
                    channel,
                    prv,
                    topic,
                })?;
            }
        }
        self.send_response(&CommandResponse::ListEnd323)?;

        Ok(true)
    }

    fn who_logic(&mut self, mut params: Vec<String>) -> std::io::Result<bool> {
        let mask = params.pop();

        let mut clients = match &mask {
            Some(mask) => self.database.get_clients_for_mask(mask),
            None => self.clients_for_default_who(),
        };

        clients.sort();

        for client_info in clients {
            self.send_whoreply_response(client_info)?;
        }

        self.send_response(&CommandResponse::EndOfWho315 { name: mask })?;

        Ok(true)
    }

    fn whois_logic(&mut self, mut params: Vec<String>) -> std::io::Result<bool> {
        let nickmasks = params.pop().unwrap();
        let _server = params.get(0);

        for nickmask in nickmasks.split(',') {
            let mut clients: Vec<ClientInfo> = self.database.get_clients_for_nickmask(nickmask);

            clients.sort();

            if clients.is_empty() {
                let nickname = nickmask.to_string();
                self.send_response(&ErrorReply::NoSuchNickname401 { nickname })?;
            } else {
                for client in clients {
                    self.send_whois_response(client)?;
                }
            }
        }

        Ok(true)
    }

    fn away_logic(&mut self, trail: Option<String>) -> std::io::Result<bool> {
        self.database.set_away_message(&trail, &self.nickname);

        let reply = match trail {
            Some(_) => CommandResponse::NowAway,
            None => CommandResponse::UnAway,
        };

        self.send_response(&reply)?;

        Ok(true)
    }

    fn topic_logic(&mut self, mut params: Vec<String>) -> std::io::Result<bool> {
        let channel = params.remove(0);

        if let Some(topic) = params.pop() {
            self.database.set_channel_topic(&channel, &topic);
        } else {
            self.send_topic_response(channel)?;
        }

        Ok(true)
    }

    fn kick_logic(&mut self, params: Vec<String>, trail: Option<String>) -> std::io::Result<bool> {
        let channel = params[0].split(',');
        let nickname = params[1].split(',');

        for (channel, nickname) in channel.zip(nickname) {
            if let Err(error) = self.assert_can_kick_from_channel(channel) {
                self.send_response(&error)?;
            } else {
                self.kick_client_from_channel(nickname, channel, &trail);
            }
        }

        Ok(true)
    }

    fn mode_logic(&mut self, params: Vec<String>) -> std::io::Result<bool> {
        let modes: Vec<char> = params[1].chars().collect();

        let (add, remove) = parse_modes(modes);

        self.add_modes(add, params.clone())?;
        self.remove_modes(remove, params)?;

        Ok(true)
    }

    fn quit_logic(&mut self, trail: Option<String>) -> std::io::Result<bool> {
        let message = trail.unwrap_or_else(|| self.nickname.clone());

        let notification = Notification::Quit { message };

        self.database.disconnect_client(&self.nickname);
        let channels = self.database.get_channels_for_client(&self.nickname);
        for channel in channels {
            self.send_message_to_channel(&channel, &notification.to_string());
        }

        self.send_response(&notification.to_string())?;

        Ok(false)
    }
}
