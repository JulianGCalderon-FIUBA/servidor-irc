use std::io;

use crate::server::connection::Connection;
use crate::server::connection_handler::connection_handler_trait::{
    CommandArgs, ConnectionHandlerLogic, ConnectionHandlerUtils,
};

use crate::server::data_structures::*;
use crate::server::responses::{CommandResponse, Notification};

use self::utils::{collect_list, parse_modes};

use super::ClientHandler;

mod auxiliars;
mod booleans;
mod mode_requests;
mod responses;
mod utils;

impl<C: Connection> ConnectionHandlerLogic<C> for ClientHandler<C> {
    fn nick_logic(&mut self, arguments: CommandArgs) -> io::Result<bool> {
        let (_, params, _) = arguments;

        let new_nickname = params[0].clone();
        self.database.update_nickname(&self.nickname, &new_nickname);

        let nick_notification = Notification::nick_update(&self.nickname, &new_nickname);
        self.send_message_to_all_servers(&nick_notification);

        self.nickname = new_nickname;

        Ok(true)
    }

    fn oper_logic(&mut self, _arguments: CommandArgs) -> std::io::Result<bool> {
        self.database.set_server_operator(&self.nickname);
        self.stream.send(&CommandResponse::you_are_oper())?;

        // TODO: RELAY MODE TO OTHER SERVERS

        Ok(true)
    }

    fn privmsg_logic(&mut self, arguments: CommandArgs) -> std::io::Result<bool> {
        let (_, mut params, trail) = arguments;
        let content = trail.unwrap();
        let targets = params.remove(0);

        for target in targets.split(',') {
            if let Err(error) = self.assert_target_is_valid(target) {
                self.stream.send(&error)?;
                continue;
            }

            self.send_privmsg_to_target(target, &content)?;
        }

        Ok(true)
    }

    fn notice_logic(&mut self, arguments: CommandArgs) -> std::io::Result<bool> {
        let (_, mut params, trail) = arguments;
        let content = trail.unwrap();
        let targets = params.pop().unwrap();

        for target in targets.split(',') {
            if let Err(error) = self.assert_target_is_valid(target) {
                self.stream.send(&error)?;
                continue;
            }

            self.send_notice_to_target(target, &content)?;
        }

        Ok(true)
    }

    fn join_logic(&mut self, arguments: CommandArgs) -> std::io::Result<bool> {
        let (_, params, _) = arguments;
        let channels = params[0].split(',');

        let mut keys = collect_list(params.get(1)).into_iter();

        for channel in channels {
            let key = keys.next();

            if let Err(error) = self.assert_can_join_channel(channel, &key) {
                self.stream.send(&error)?;
                continue;
            }

            self.send_join_notification(channel);

            self.database.add_client_to_channel(&self.nickname, channel);

            self.send_join_response(channel)?;
        }

        Ok(true)
    }

    fn part_logic(&mut self, arguments: CommandArgs) -> std::io::Result<bool> {
        let (_, mut params, _) = arguments;
        let channels = params.pop().unwrap();

        for channel in channels.split(',') {
            if let Err(error) = self.assert_can_part_channel(channel) {
                self.stream.send(&error)?;
                continue;
            }

            self.send_part_notification(channel);

            self.database
                .remove_client_from_channel(&self.nickname, channel);
        }

        Ok(true)
    }

    fn invite_logic(&mut self, arguments: CommandArgs) -> std::io::Result<bool> {
        let (_, mut params, _) = arguments;
        let channel = params.pop().unwrap();
        let invited_client = params.pop().unwrap();
        let inviting_client = self.nickname.clone();

        if self
            .send_invite_notification(invited_client, &channel)
            .is_ok()
        {
            self.stream
                .send(&CommandResponse::inviting(inviting_client, channel))?;
        }

        Ok(true)
    }

    fn names_logic(&mut self, arguments: CommandArgs) -> std::io::Result<bool> {
        let (_, params, _) = arguments;
        let channels = self.channels_to_list(params.get(0));

        for channel in channels {
            if !self.can_name_channel(&channel) {
                continue;
            }

            self.send_names_response(&channel)?;

            if !params.is_empty() {
                self.stream.send(&CommandResponse::end_of_names(&channel))?;
            }
        }

        if params.is_empty() {
            self.stream.send(&CommandResponse::end_of_names(""))?;
        }

        Ok(true)
    }

    fn list_logic(&mut self, arguments: CommandArgs) -> std::io::Result<bool> {
        let (_, params, _) = arguments;
        let channels = self.channels_to_list(params.get(0));

        self.stream.send(&CommandResponse::list_start())?;

        for channel in channels {
            if !self.can_list_channel(&channel) {
                continue;
            }
            self.send_list_response(channel)?;
        }
        self.stream.send(&CommandResponse::list_end())?;

        Ok(true)
    }

    fn who_logic(&mut self, arguments: CommandArgs) -> std::io::Result<bool> {
        let (_, mut params, _) = arguments;
        let mask = params.pop();

        let clients = match &mask {
            Some(mask) => self.database.get_clients_for_mask(mask),
            None => self.clients_for_default_who(),
        };

        for client_info in clients {
            self.send_whoreply_response(client_info)?;
        }

        self.stream.send(&CommandResponse::end_of_who(mask))?;

        Ok(true)
    }

    fn whois_logic(&mut self, arguments: CommandArgs) -> std::io::Result<bool> {
        let (_, mut params, _) = arguments;
        let nickmasks = params.pop().unwrap();
        let _server = params.get(0);

        for nickmask in nickmasks.split(',') {
            let clients: Vec<ClientInfo> = self.database.get_clients_for_nickmask(nickmask);

            if let Err(error) = self.assert_can_send_whois_response(&clients, nickmask) {
                self.stream.send(&error)?;
                continue;
            }
            for client in clients {
                self.send_whois_response(client)?;
            }
        }

        Ok(true)
    }

    fn away_logic(&mut self, arguments: CommandArgs) -> std::io::Result<bool> {
        let (_, _, trail) = arguments;
        self.database.set_away_message(&trail, &self.nickname);

        let away_notification = Notification::away(&self.nickname, &trail);
        let reply = match trail {
            Some(_) => CommandResponse::now_away(),
            None => CommandResponse::unaway(),
        };

        self.send_message_to_all_servers(&away_notification);
        self.stream.send(&reply)?;

        Ok(true)
    }

    fn topic_logic(&mut self, arguments: CommandArgs) -> std::io::Result<bool> {
        let (_, mut params, _) = arguments;
        let channel = params.remove(0);

        if let Some(topic) = params.pop() {
            self.database.set_channel_topic(&channel, &topic);
            self.send_topic_notification(&channel, &topic);
        } else {
            self.send_topic_response(&channel)?;
        }

        Ok(true)
    }

    fn kick_logic(&mut self, arguments: CommandArgs) -> std::io::Result<bool> {
        let (_, params, trail) = arguments;
        let channel = params[0].split(',');
        let nickname = params[1].split(',');

        for (channel, nickname) in channel.zip(nickname) {
            if let Err(error) = self.assert_can_kick_from_channel(channel) {
                self.stream.send(&error)?;
            } else {
                self.kick_client_from_channel(nickname, channel, &trail);
            }
        }

        Ok(true)
    }

    fn mode_logic(&mut self, arguments: CommandArgs) -> std::io::Result<bool> {
        let (_, mut params, _) = arguments;

        let channel = params[0].clone();

        if params.len() == 1 {
            self.send_mode_response(&channel)?;
            return Ok(true);
        }

        let modes: Vec<char> = params[1].chars().collect();

        let (add, remove) = parse_modes(modes);

        let mut arguments: Vec<String> = params.drain(2..).collect();
        arguments.reverse();

        self.add_modes(add, &mut arguments, &channel)?;
        self.remove_modes(remove, &mut arguments, &channel)?;

        Ok(true)
    }

    fn quit_logic(&mut self, arguments: CommandArgs) -> std::io::Result<bool> {
        let (_, _, trail) = arguments;
        let message = trail.unwrap_or_else(|| self.nickname.clone());

        self.database.disconnect_client(&self.nickname);

        self.send_quit_notification(&message);

        self.stream.send(&CommandResponse::quit(&message))?;

        Ok(false)
    }
}
