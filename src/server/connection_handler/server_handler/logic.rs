use std::io;

use crate::server::connection::Connection;
use crate::server::connection_handler::connection_handler_trait::{
    CommandArgs, ConnectionHandlerLogic, ConnectionHandlerUtils,
};

use crate::server::data_structures_2::*;
use crate::server::responses::Notification;

use super::ServerHandler;

impl<C: Connection> ConnectionHandlerLogic<C> for ServerHandler<C> {
    fn nick_logic(&mut self, arguments: CommandArgs) -> std::io::Result<bool> {
        let (prefix, mut params, _) = arguments;

        if let Some(old_nickname) = prefix {
            let new_nickname = &params.remove(0);
            self.database.update_nickname(&old_nickname, new_nickname);
        } else {
            let hopcount = params.pop().unwrap().parse::<usize>().unwrap();
            let nickname = params.pop().unwrap();
            self.hopcounts.insert(nickname, hopcount);
        }

        Ok(true)
    }

    fn user_logic(&mut self, arguments: CommandArgs) -> std::io::Result<bool> {
        let (prefix, mut params, trail) = arguments;

        let nickname = prefix.unwrap();
        let hopcount = *self.hopcounts.get(&nickname).unwrap();
        let servername = params.pop().unwrap();
        let hostname = params.pop().unwrap();
        let username = params.pop().unwrap();
        let realname = trail.unwrap();

        // let client = ExternalClient::new(
        //     nickname,
        //     username,
        //     hostname,
        //     servername,
        //     realname,
        //     hopcount,
        //     self.servername.clone(),
        // );

        let info = ClientInfo::new(
            &nickname,
            &username,
            &hostname,
            &servername,
            &realname,
            hopcount,
        );
        let client = ExternalClient::new(&self.servername, info);

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

    fn join_logic(&mut self, _arguments: CommandArgs) -> io::Result<bool> {
        Ok(true)
    }

    fn part_logic(&mut self, _arguments: CommandArgs) -> io::Result<bool> {
        Ok(true)
    }

    fn invite_logic(&mut self, arguments: CommandArgs) -> io::Result<bool> {
        let (prefix, params, _) = arguments;

        let inviting = &prefix.unwrap();
        let invited = &params[0];
        let channel = &params[1];

        let invite_notification = Notification::invite(inviting, invited, channel);
        self.send_message_to_client(&invite_notification, invited)
            .ok();

        Ok(true)
    }

    fn away_logic(&mut self, arguments: CommandArgs) -> io::Result<bool> {
        let (prefix, _, trail) = arguments;
        self.database
            .set_away_message(&trail, prefix.as_ref().unwrap());

        let away_notification = Notification::away(&prefix.unwrap(), &trail);
        self.send_message_to_all_other_servers(&away_notification);

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
}
