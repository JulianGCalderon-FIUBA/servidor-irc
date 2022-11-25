use std::io;

use crate::server::connection::Connection;
use crate::server::connection_handler::connection_handler_trait::{
    CommandArgs, ConnectionHandlerLogic, ConnectionHandlerUtils,
};

use crate::server::data_structures::*;
use crate::server::responses::Notification;

use super::ServerHandler;

impl<C: Connection> ConnectionHandlerLogic<C> for ServerHandler<C> {
    fn nick_logic(&mut self, arguments: CommandArgs) -> std::io::Result<bool> {
        let (prefix, mut params, _) = arguments;

        let nickname = params.remove(0);

        if let Some(old_nickname) = prefix {
            self.database.update_nickname(&old_nickname, &nickname);
            self.send_nick_update_notification(&old_nickname, &nickname);
            return Ok(true);
        }

        let hopcount = params[0].parse::<usize>().unwrap();
        self.send_nick_notification(&nickname, hopcount);
        self.hopcounts.insert(nickname, hopcount);

        Ok(true)
    }

    fn user_logic(&mut self, arguments: CommandArgs) -> std::io::Result<bool> {
        let (prefix, params, trail) = arguments;

        let nickname = &prefix.unwrap();
        let client = ClientBuilder::<C>::new()
            .nickname(nickname)
            .hopcount(self.hopcounts.remove(nickname).unwrap())
            .username(params.get(0).unwrap())
            .hostname(params.get(1).unwrap())
            .servername(params.get(2).unwrap())
            .realname(&trail.unwrap())
            .immediate(&self.servername)
            .build_external_client()
            .unwrap();

        self.send_user_notification(&client.get_info());
        self.database.add_external_client(client);

        Ok(true)
    }

    fn privmsg_logic(&mut self, arguments: CommandArgs) -> std::io::Result<bool> {
        let (prefix, mut params, trail) = arguments;

        let sender = prefix.unwrap();
        let target = params.remove(0);
        let content = trail.unwrap();

        self.send_privmsg_notification(&sender, &target, &content);

        Ok(true)
    }

    fn oper_logic(&mut self, _arguments: CommandArgs) -> io::Result<bool> {
        todo!()
    }

    fn notice_logic(&mut self, arguments: CommandArgs) -> io::Result<bool> {
        let (prefix, mut params, trail) = arguments;

        let sender = prefix.unwrap();
        let target = params.remove(0);
        let content = trail.unwrap();

        self.send_notice_notification(&sender, &target, &content);

        Ok(true)
    }

    fn join_logic(&mut self, arguments: CommandArgs) -> io::Result<bool> {
        let (prefix, mut params, _) = arguments;

        let nickname = prefix.unwrap();
        let channel = params.remove(0);
        self.database.add_client_to_channel(&nickname, &channel);
        self.send_join_notification(&nickname, &channel);
        Ok(true)
    }

    fn part_logic(&mut self, arguments: CommandArgs) -> io::Result<bool> {
        let (prefix, mut params, _) = arguments;

        let nickname = prefix.unwrap();
        let channel = params.remove(0);
        self.database
            .remove_client_from_channel(&nickname, &channel);
        self.send_part_notification(&nickname, &channel);
        Ok(true)
    }

    fn invite_logic(&mut self, arguments: CommandArgs) -> io::Result<bool> {
        let (prefix, params, _) = arguments;

        let inviting = &prefix.unwrap();
        let invited = &params[0];
        let channel = &params[1];

        println!("holaa");

        self.send_invite_notification(inviting, invited, channel);
        Ok(true)
    }

    fn away_logic(&mut self, arguments: CommandArgs) -> io::Result<bool> {
        let (prefix, _, trail) = arguments;
        self.database
            .set_away_message(&trail, prefix.as_ref().unwrap());

        let nickname = prefix.unwrap();
        let message = trail;

        self.send_away_notification(&nickname, &message);
        Ok(true)
    }

    fn topic_logic(&mut self, _arguments: CommandArgs) -> io::Result<bool> {
        Ok(true)
    }

    fn kick_logic(&mut self, _arguments: CommandArgs) -> io::Result<bool> {
        Ok(true)
    }

    fn mode_logic(&mut self, _arguments: CommandArgs) -> io::Result<bool> {
        todo!()
    }

    fn quit_logic(&mut self, arguments: CommandArgs) -> io::Result<bool> {
        let (prefix, _, trail) = arguments;

        let nickname = prefix.unwrap();
        self.send_quit_notification(nickname, trail.unwrap());

        Ok(true)
    }

    fn server_logic(&mut self, arguments: CommandArgs) -> io::Result<bool> {
        let (_, mut params, trail) = arguments;

        let hopcount = params.remove(1).parse::<usize>().unwrap();
        let servername = params.remove(0);
        let serverinfo = trail.unwrap();

        let server_notification = Notification::server(&servername, hopcount + 1, &serverinfo);
        self.send_message_to_all_other_servers(&server_notification);

        let server = ServerInfo::new(&servername, &serverinfo, hopcount);
        self.database.add_distant_server(server);

        Ok(true)
    }

    fn squit_logic(&mut self, _arguments: CommandArgs) -> io::Result<bool> {
        todo!()
    }
}

impl<C: Connection> ServerHandler<C> {
    pub(super) fn send_privmsg_notification(&mut self, sender: &str, target: &str, content: &str) {
        let notification = Notification::privmsg(sender, target, content);
        self.send_message_to_target(&notification, target).ok();
    }

    pub(super) fn send_notice_notification(&mut self, sender: &str, target: &str, content: &str) {
        let notification = Notification::notice(sender, target, content);
        self.send_message_to_target(&notification, target).ok();
    }

    fn send_quit_notification(&mut self, nickname: String, message: String) {
        let quit_notification = Notification::quit(&nickname, &message);
        let channels = self.database.get_channels_for_client(&nickname);
        for channel in channels {
            self.send_message_to_local_clients_on_channel(&quit_notification, &channel);
        }
        self.send_message_to_all_other_servers(&quit_notification);
    }

    fn send_nick_update_notification(&mut self, old_nickname: &str, new_nickname: &str) {
        let notification = Notification::nick_update(old_nickname, new_nickname);
        self.send_message_to_all_other_servers(&notification);
    }

    fn send_nick_notification(&mut self, nickname: &str, hopcount: usize) {
        let notification = Notification::nick(nickname, hopcount + 1);
        self.send_message_to_all_other_servers(&notification);
    }

    fn send_user_notification(&mut self, client: &ClientInfo) {
        let notification = Notification::user(client);
        self.send_message_to_all_other_servers(&notification);
    }

    fn send_join_notification(&mut self, nickname: &str, channel: &str) {
        let notification = Notification::join(nickname, channel);

        self.send_message_to_local_clients_on_channel(&notification, channel);
        self.send_message_to_all_other_servers(&notification);
    }

    fn send_part_notification(&mut self, nickname: &str, channel: &str) {
        let notification = Notification::part(nickname, channel);

        self.send_message_to_local_clients_on_channel(&notification, channel);
        self.send_message_to_all_other_servers(&notification);
    }

    fn send_invite_notification(&mut self, inviting: &str, invited: &str, channel: &str) {
        let invite_notification = Notification::invite(inviting, invited, channel);
        if self.database.is_local_client(invited) {
            self.send_message_to_client(&invite_notification, invited)
                .ok();
        }
        self.send_message_to_all_other_servers(&invite_notification);
    }

    fn send_away_notification(&mut self, nickname: &str, message: &Option<String>) {
        let notification = Notification::away(nickname, message);
        self.send_message_to_all_other_servers(&notification);
    }
}
