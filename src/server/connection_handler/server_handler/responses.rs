use crate::{
    macros::ok_or_return,
    server::{
        connection::Connection, connection_handler::ConnectionHandlerUtils,
        data_structures::ClientInfo, responses::Notification,
    },
};

use super::ServerHandler;

impl<C: Connection> ServerHandler<C> {
    pub(super) fn send_privmsg_notification(&mut self, sender: &str, target: &str, content: &str) {
        let notification = Notification::privmsg(sender, target, content);
        self.send_message_to_target(&notification, target)
    }

    pub(super) fn send_notice_notification(&mut self, sender: &str, target: &str, content: &str) {
        let notification = Notification::notice(sender, target, content);
        self.send_message_to_target(&notification, target)
    }

    pub(super) fn send_quit_notification(&mut self, nickname: String, message: String) {
        let quit_notification = Notification::quit(&nickname, &message);
        let channels = ok_or_return!(self.database.get_channels_for_client(&nickname));
        for channel in channels {
            self.send_message_to_local_clients_on_channel(&quit_notification, &channel);
        }
        self.send_message_to_all_other_servers(&quit_notification);
    }

    pub(super) fn send_nick_update_notification(&mut self, old_nickname: &str, new_nickname: &str) {
        let notification = Notification::nick_update(old_nickname, new_nickname);
        self.send_message_to_all_other_servers(&notification);
    }

    pub(super) fn send_nick_notification(&mut self, nickname: &str, hopcount: usize) {
        let notification = Notification::nick(nickname, hopcount + 1);
        self.send_message_to_all_other_servers(&notification);
    }

    pub(super) fn send_user_notification(&mut self, client: &ClientInfo) {
        let notification = Notification::user(client);
        self.send_message_to_all_other_servers(&notification);
    }

    pub(super) fn send_join_notification(&mut self, nickname: &str, channel: &str) {
        let notification = Notification::join(nickname, channel);

        self.send_message_to_local_clients_on_channel(&notification, channel);
        self.send_message_to_all_other_servers(&notification);
    }

    pub(super) fn send_part_notification(&mut self, nickname: &str, channel: &str) {
        let notification = Notification::part(nickname, channel);

        self.send_message_to_local_clients_on_channel(&notification, channel);
        self.send_message_to_all_other_servers(&notification);
    }

    pub(super) fn send_invite_notification(
        &mut self,
        inviting: &str,
        invited: &str,
        channel: &str,
    ) {
        let invite_notification = Notification::invite(inviting, invited, channel);
        if self.database.is_local_client(invited) {
            self.send_message_to_client(&invite_notification, invited)
                .ok();
        }
        self.send_message_to_all_other_servers(&invite_notification);
    }

    pub(super) fn send_away_notification(&mut self, nickname: &str, message: &Option<String>) {
        let notification = Notification::away(nickname, message);
        self.send_message_to_all_other_servers(&notification);
    }

    pub(super) fn send_topic_notification(&mut self, nickname: &str, channel: &str, topic: &str) {
        let notification = Notification::topic(nickname, channel, topic);
        self.send_message_to_local_clients_on_channel(&notification, channel);
        self.send_message_to_all_other_servers(&notification);
    }

    pub(super) fn send_kick_notification(
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

    pub(super) fn send_server_notification(
        &mut self,
        servername: &str,
        hopcount: usize,
        serverinfo: &str,
    ) {
        let server_notification = Notification::server(servername, hopcount, serverinfo);
        self.send_message_to_all_other_servers(&server_notification);
    }

    pub(super) fn send_mode_notification(&mut self, sender: &str, target: &str, request: &str) {
        let notification = Notification::mode(sender, target, request);

        if self.is_channel(target) {
            self.send_message_to_local_clients_on_channel(&notification, target);
        }
        self.send_message_to_all_other_servers(&notification);
    }

    pub(super) fn send_squit_notification(
        &mut self,
        sender: &str,
        servername: &str,
        comment: Option<String>,
    ) {
        let notification = Notification::squit(sender, servername, comment);
        self.send_message_to_all_other_servers(&notification);
    }
}
