use std::io;

use crate::server::connection::Connection;
use crate::server::connection_handler::connection_handler_trait::{
    CommandArgs, ConnectionHandlerLogic,
};

use crate::server::consts::channel::DISTRIBUTED_CHANNEL;
use crate::server::consts::modes::{ADD_MODE, REMOVE_MODE};
use crate::server::data_structures::*;

use super::ServerHandler;

mod mode_requests;

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

    fn squit_logic(&mut self, arguments: CommandArgs) -> io::Result<bool> {
        let (prefix, params, trail) = arguments;

        let sender = prefix.unwrap();
        let servername = &params[0];
        let comment = trail;

        self.send_squit_notification(&sender, servername, comment);

        if self.database.is_immediate_server(servername) {
            self.database.remove_server(servername);

            let all_clients = self.database.get_all_clients();
            let server_clients: Vec<ClientInfo> = all_clients
                .into_iter()
                .filter(|client| client.servername == *servername)
                .collect();

            for client in server_clients {
                self.database.disconnect_client(&client.nickname());
                self.send_quit_notification(client.nickname(), "Net split".to_string());
            }

            // preguntar si hay que desconectarlos o eliminarlos
        }

        Ok(true)
    }
}

impl<C: Connection> ServerHandler<C> {
    pub fn is_channel(&self, target: &str) -> bool {
        target.starts_with(DISTRIBUTED_CHANNEL)
    }

    fn add_server(&mut self, servername: String, serverinfo: String, hopcount: usize) {
        let server = ServerInfo::new(servername, serverinfo, hopcount);
        self.database.add_distant_server(server);
    }
}
