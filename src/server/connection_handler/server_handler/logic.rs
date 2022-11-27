use std::io;

use crate::macros::{ok_or_return, some_or_return};
use crate::server::connection::Connection;
use crate::server::connection_handler::connection_handler_trait::{
    CommandArgs, ConnectionHandlerLogic, ConnectionHandlerUtils,
};

use crate::server::consts::channel::DISTRIBUTED_CHANNEL;
use crate::server::consts::modes::{
    ChannelFlag, ADD_MODE, REMOVE_MODE, SET_BANMASK, SET_KEY, SET_OPERATOR, SET_SPEAKER,
    SET_USER_LIMIT,
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

    fn topic_logic(&mut self, arguments: CommandArgs) -> io::Result<bool> {
        let (prefix, params, _) = arguments;

        let nickname = &prefix.unwrap();
        let channel = &params[0];
        let topic = &params[1];
        self.database.set_channel_topic(channel, topic);

        self.send_topic_notification(nickname, channel, topic);
        Ok(true)
    }

    fn kick_logic(&mut self, arguments: CommandArgs) -> io::Result<bool> {
        let (prefix, mut params, trail) = arguments;

        let kicker = prefix.unwrap();
        let kicked = params.remove(1);
        let channel = params.remove(0);
        let message = trail;

        self.send_kick_notification(&kicker, &channel, &kicked, &message);
        self.database.remove_client_from_channel(&kicked, &channel);

        Ok(true)
    }

    fn mode_logic(&mut self, arguments: CommandArgs) -> io::Result<bool> {
        let (_, mut params, _) = arguments;

        let channel = params[0].clone();
        let mode: Vec<char> = params[1].chars().collect();
        let mut argument = None;
        if params.len() > 2 {
            argument = params.pop();
        }
        if mode[0] == ADD_MODE {
            self.handle_add_mode(mode[1], &channel, argument.clone());
        }
        if mode[0] == REMOVE_MODE {
            self.handle_remove_mode(mode[1], &channel, argument.clone());
        }

        self.send_mode_notification(&channel, &params[1], argument);

        Ok(true)
    }

    fn quit_logic(&mut self, arguments: CommandArgs) -> io::Result<bool> {
        let (prefix, _, trail) = arguments;

        let nickname = prefix.unwrap();
        let message = trail.unwrap();

        self.database.disconnect_client(&nickname);
        self.send_quit_notification(nickname, message);

        Ok(true)
    }

    fn server_logic(&mut self, arguments: CommandArgs) -> io::Result<bool> {
        let (_, mut params, trail) = arguments;

        let hopcount = params.remove(1).parse::<usize>().unwrap();
        let servername = params.remove(0);
        let serverinfo = trail.unwrap();

        self.send_server_notification(&servername, hopcount + 1, &serverinfo);

        self.add_server(servername, serverinfo, hopcount);

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
        let channels = self.database.get_channels_for_client(&nickname).unwrap();
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

    fn send_topic_notification(&mut self, nickname: &str, channel: &str, topic: &str) {
        let notification = Notification::topic(nickname, channel, topic);
        self.send_message_to_local_clients_on_channel(&notification, channel);
        self.send_message_to_all_other_servers(&notification);
    }

    fn send_kick_notification(
        &mut self,
        kicker: &str,
        channel: &str,
        kicked: &str,
        message: &Option<String>,
    ) {
        let notification = Notification::kick(kicker, channel, kicked, message);
        self.send_message_to_local_clients_on_channel(&notification, channel);
        self.send_message_to_all_other_servers(&notification);
    }

    fn send_server_notification(&mut self, servername: &str, hopcount: usize, serverinfo: &str) {
        let server_notification = Notification::server(servername, hopcount, serverinfo);
        self.send_message_to_all_other_servers(&server_notification);
    }

    fn send_mode_notification(&mut self, target: &str, mode: &str, argument: Option<String>) {
        let notification = Notification::mode(target, mode, argument);
        if self.is_channel(target) {
            self.send_message_to_local_clients_on_channel(&notification, target);
        }
        self.send_message_to_all_other_servers(&notification);
    }

    fn is_channel(&self, target: &str) -> bool {
        target.starts_with(DISTRIBUTED_CHANNEL)
    }

    fn add_server(&mut self, servername: String, serverinfo: String, hopcount: usize) {
        let server = ServerInfo::new(servername, serverinfo, hopcount);
        self.database.add_distant_server(server);
    }

    fn set_limit(&self, channel: &str, argument: Option<String>) {
        let limit = some_or_return!(argument);
        let limit = ok_or_return!(limit.parse::<usize>());
        self.database.set_channel_limit(channel, Some(limit))
    }

    fn add_banmasks(&self, channel: &str, argument: Option<String>) {
        let banmask = some_or_return!(argument);
        self.database.add_channel_banmask(channel, &banmask);
    }

    fn remove_banmasks(&self, channel: &str, argument: Option<String>) {
        let banmask = some_or_return!(argument);
        self.database.remove_channel_banmask(channel, &banmask);
    }

    fn add_speakers(&self, channel: &str, argument: Option<String>) {
        let speaker = some_or_return!(argument);
        self.database.add_speaker(channel, &speaker);
    }

    fn set_key(&self, channel: &str, argument: Option<String>) {
        let key = some_or_return!(argument);
        self.database.set_channel_key(channel, Some(key));
    }

    fn add_channops(&self, channel: &str, argument: Option<String>) {
        let channop = some_or_return!(argument);
        self.database.add_channop(channel, &channop);
    }

    fn unset_limit(&self, channel: &str) {
        self.database.set_channel_limit(channel, None)
    }

    fn remove_speakers(&self, channel: &str, argument: Option<String>) {
        let speaker = some_or_return!(argument);
        self.database.remove_speaker(channel, &speaker);
    }

    fn unset_key(&self, channel: &str) {
        self.database.set_channel_key(channel, None)
    }

    fn remove_channops(&self, channel: &str, argument: Option<String>) {
        let channop = some_or_return!(argument);
        self.database.remove_channop(channel, &channop)
    }

    fn handle_add_mode(&mut self, mode: char, channel: &str, argument: Option<String>) {
        match mode {
            SET_USER_LIMIT => self.set_limit(channel, argument),
            SET_BANMASK => {
                self.add_banmasks(channel, argument);
            }
            SET_SPEAKER => {
                self.add_speakers(channel, argument);
            }
            SET_KEY => {
                self.set_key(channel, argument);
            }
            SET_OPERATOR => self.add_channops(channel, argument),
            char => {
                let flag = ChannelFlag::from_char(char);
                self.database.set_channel_mode(channel, flag)
            }
        }
    }

    fn handle_remove_mode(&mut self, mode: char, channel: &str, argument: Option<String>) {
        match mode {
            SET_USER_LIMIT => self.unset_limit(channel),
            SET_BANMASK => {
                self.remove_banmasks(channel, argument);
            }
            SET_SPEAKER => {
                self.remove_speakers(channel, argument);
            }
            SET_KEY => {
                self.unset_key(channel);
            }
            SET_OPERATOR => self.remove_channops(channel, argument),
            char => {
                let flag = ChannelFlag::from_char(char);
                self.database.unset_channel_mode(channel, flag)
            }
        }
    }
}
