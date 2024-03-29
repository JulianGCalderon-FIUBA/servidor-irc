use std::io;

use crate::macros::ok_or_return;
use crate::server::connection::Connection;
use crate::server::connection_handler::{
    CommandArgs, ConnectionHandlerLogic, ConnectionHandlerUtils,
};

use crate::server::connection_handler::mode_requests::{
    parse_channel_mode_string, parse_user_mode_string,
};
use crate::server::consts::channel::{DISTRIBUTED_CHANNEL, LOCAL_CHANNEL};
use crate::server::data_structures::*;
use crate::server::responses::{CommandResponse, Notification};

use super::utils::collect_list;
use super::ClientHandler;

/// Contains the extended logic of the MODE command.
mod mode_logic;

pub const SQUIT_MESSAGE: &str = "Net split";

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

        self.send_oper_notification();

        Ok(true)
    }

    fn privmsg_logic(&mut self, arguments: CommandArgs) -> std::io::Result<bool> {
        let (_, mut params, trail) = arguments;
        let content = trail.expect("Verified in assert");
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
        let content = trail.expect("Verified in assert");
        let targets = params.pop().expect("Verified in assert");

        for target in targets.split(',') {
            if let Err(error) = self.assert_target_is_valid(target) {
                self.stream.send(&error)?;
                continue;
            }

            self.send_notice_to_target(target, &content);
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

            let creating_channel = !self.database.contains_channel(channel);

            self.database.add_client_to_channel(channel, &self.nickname);

            if creating_channel {
                self.database.add_channel_operator(channel, &self.nickname);
                self.send_channel_operator_notification(channel, &self.nickname);
            }

            self.send_join_response(channel)?;
        }

        Ok(true)
    }

    fn part_logic(&mut self, arguments: CommandArgs) -> std::io::Result<bool> {
        let (_, mut params, _) = arguments;
        let channels = params.pop().expect("Verified in assert");

        for channel in channels.split(',') {
            if let Err(error) = self.assert_can_part_channel(channel) {
                self.stream.send(&error)?;
                continue;
            }

            self.send_part_notification(channel);

            self.database
                .remove_client_from_channel(channel, &self.nickname);
        }

        Ok(true)
    }

    fn invite_logic(&mut self, arguments: CommandArgs) -> std::io::Result<bool> {
        let (_, mut params, _) = arguments;
        let channel = params.pop().expect("Verified in assert");
        let invited_client = params.pop().expect("Verified in assert");
        let inviting_client = self.nickname.clone();

        self.send_invite_notification(&invited_client, &channel)
            .ok();

        self.stream
            .send(&CommandResponse::inviting(&inviting_client, &channel))?;

        self.database.add_channel_invite(&channel, &invited_client);

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
            self.send_name_response_for_remaining_clients()?;
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
            Some(mask) => self.get_clients_for_mask(mask),
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
        let nickmasks = params.pop().expect("Verified in assert");
        let server = params.get(0);

        for nickmask in nickmasks.split(',') {
            let clients: Vec<ClientInfo> = self.get_clients_for_nickmask(nickmask);

            if let Err(error) = self.assert_can_send_whois_response(&clients, nickmask) {
                self.stream.send(&error)?;
                continue;
            }
            for client in clients {
                if let Some(server) = server {
                    if &client.servername != server {
                        continue;
                    }
                }
                self.send_whois_response(client)?;
            }
        }

        Ok(true)
    }

    fn away_logic(&mut self, arguments: CommandArgs) -> std::io::Result<bool> {
        let (_, _, trail) = arguments;
        self.database
            .set_away_message(&self.nickname, trail.clone());

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

        let target = params.remove(0);

        if target.starts_with([DISTRIBUTED_CHANNEL, LOCAL_CHANNEL]) {
            self.mode_command_for_channel(target, params)?;
        } else {
            self.mode_command_for_user(target, params)?;
        }

        Ok(true)
    }

    fn quit_logic(&mut self, arguments: CommandArgs) -> std::io::Result<bool> {
        let (_, _, trail) = arguments;
        let message = trail.unwrap_or_else(|| self.nickname.clone());

        let nickname = self.nickname.clone();
        self.database.disconnect_client(&nickname);

        self.send_quit_notification(&nickname, &message);

        self.stream.send(&CommandResponse::quit(&message))?;

        Ok(false)
    }

    fn squit_logic(&mut self, arguments: CommandArgs) -> std::io::Result<bool> {
        let (_, params, trail) = arguments;

        let servername = &params[0];
        let comment = trail;

        self.send_squit_notification(servername, comment);

        if self.database.is_immediate_server(servername) {
            self.database.remove_server(servername);

            let all_clients = self.database.get_all_clients();
            let server_clients: Vec<ClientInfo> = all_clients
                .into_iter()
                .filter(|client| {
                    let server =
                        ok_or_return!(self.database.get_immediate_server(&client.nickname), false);
                    server == *servername
                })
                .collect();

            for client in server_clients {
                self.database.disconnect_client(&client.nickname);
                self.send_quit_notification(&client.nickname, SQUIT_MESSAGE);
            }
        }

        Ok(true)
    }

    fn ctcp_logic(&mut self, arguments: CommandArgs) -> io::Result<bool> {
        let (_, mut params, trail) = arguments;

        let target = params.remove(0);
        let mut content = trail.expect("Verified in assert");

        content.insert(0, 1 as char);
        content.push(1 as char);

        self.send_privmsg_to_target(&target, &content)?;

        Ok(true)
    }
}

impl<C: Connection> ClientHandler<C> {
    fn send_privmsg_to_target(&mut self, target: &str, content: &str) -> io::Result<()> {
        self.send_privmsg_notification(target, content);

        if let Ok(Some(message)) = self.database.get_away_message(target) {
            self.stream.send(&CommandResponse::away(target, &message))?;
        }

        Ok(())
    }

    fn send_notice_to_target(&mut self, target: &str, content: &str) {
        self.send_notice_notification(target, content);
    }

    fn kick_client_from_channel(
        &mut self,
        nickname: &str,
        channel: &str,
        comment: &Option<String>,
    ) {
        self.send_kick_notification(channel, nickname, comment);
        self.database.remove_client_from_channel(channel, nickname);
    }

    fn mode_command_for_channel(
        &mut self,
        channel: String,
        mut args: Vec<String>,
    ) -> Result<(), io::Error> {
        if args.is_empty() {
            self.send_channel_mode_is_response(&channel)?;
            return Ok(());
        }

        let mode_string = args.remove(0);
        let mut mode_arguments = args;
        mode_arguments.reverse();

        let mode_requests = parse_channel_mode_string(mode_string, mode_arguments);

        for request in mode_requests {
            self.handle_channel_mode_request(&channel, request)?;
        }

        Ok(())
    }

    fn mode_command_for_user(&mut self, user: String, mut args: Vec<String>) -> io::Result<()> {
        if args.is_empty() {
            self.send_user_mode_is_response(&user)?;
            return Ok(());
        }

        let mode_string = args.remove(0);

        let mode_requests = parse_user_mode_string(mode_string);
        for request in mode_requests {
            self.handle_user_mode_request(&user, request)?;
        }

        Ok(())
    }
}
