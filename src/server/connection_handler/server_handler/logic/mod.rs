use std::io;

use crate::macros::ok_or_return;
use crate::server::connection::Connection;
use crate::server::connection_handler::connection_handler_trait::{
    CommandArgs, ConnectionHandlerLogic,
};

use crate::server::connection_handler::mode_requests::{
    parse_channel_mode_string, parse_user_mode_string,
};
use crate::server::consts::channel::DISTRIBUTED_CHANNEL;
use crate::server::data_structures::*;

use super::ServerHandler;

mod mode_logic;

impl<C: Connection> ConnectionHandlerLogic<C> for ServerHandler<C> {
    fn nick_logic(&mut self, arguments: CommandArgs) -> std::io::Result<bool> {
        let (prefix, mut params, _) = arguments;

        let nickname = params.remove(0);

        if let Some(old_nickname) = prefix {
            self.database.update_nickname(&old_nickname, &nickname);
            self.send_nick_update_notification(&old_nickname, &nickname);
            return Ok(true);
        }

        let hopcount = params[0]
            .parse::<usize>()
            .expect("Hopcount should be a number");
        self.send_nick_notification(&nickname, hopcount);
        self.hopcounts.insert(nickname, hopcount);

        Ok(true)
    }

    fn user_logic(&mut self, arguments: CommandArgs) -> std::io::Result<bool> {
        let (prefix, params, trail) = arguments;

        let nickname = &prefix.expect("Prefix should be Some");
        let client = ClientBuilder::<C>::new()
            .nickname(nickname)
            .hopcount(
                self.hopcounts
                    .remove(nickname)
                    .expect("Hopcount value should be saved in hopcounts"),
            )
            .username(params.get(0).expect("Parameters should have correct size"))
            .hostname(params.get(1).expect("Parameters should have correct size"))
            .servername(params.get(2).expect("Parameters should have correct size"))
            .realname(&trail.expect("Trail should be Some"))
            .immediate(&self.servername)
            .build_external_client()
            .expect("Client's information should be stored and available");

        self.send_user_notification(&client.get_info());
        self.database.add_external_client(client);

        Ok(true)
    }

    fn privmsg_logic(&mut self, arguments: CommandArgs) -> std::io::Result<bool> {
        let (prefix, mut params, trail) = arguments;

        let sender = prefix.expect("Prefix should be Some");
        let target = params.remove(0);
        let content = trail.expect("Trail should be Some");

        self.send_privmsg_notification(&sender, &target, &content);

        Ok(true)
    }

    fn oper_logic(&mut self, _arguments: CommandArgs) -> io::Result<bool> {
        todo!()
    }

    fn notice_logic(&mut self, arguments: CommandArgs) -> io::Result<bool> {
        let (prefix, mut params, trail) = arguments;

        let sender = prefix.expect("Prefix should be Some");
        let target = params.remove(0);
        let content = trail.expect("Trail should be Some");

        self.send_notice_notification(&sender, &target, &content);

        Ok(true)
    }

    fn join_logic(&mut self, arguments: CommandArgs) -> io::Result<bool> {
        let (prefix, mut params, _) = arguments;

        let nickname = prefix.expect("Prefix should be Some");
        let channel = params.remove(0);
        self.database.add_client_to_channel(&channel, &nickname);
        self.send_join_notification(&nickname, &channel);
        Ok(true)
    }

    fn part_logic(&mut self, arguments: CommandArgs) -> io::Result<bool> {
        let (prefix, mut params, _) = arguments;

        let nickname = prefix.expect("Prefix should be Some");
        let channel = params.remove(0);
        self.database
            .remove_client_from_channel(&channel, &nickname);
        self.send_part_notification(&nickname, &channel);
        Ok(true)
    }

    fn invite_logic(&mut self, arguments: CommandArgs) -> io::Result<bool> {
        let (prefix, params, _) = arguments;

        let inviting = &prefix.expect("Prefix should be Some");
        let invited = &params[0];
        let channel = &params[1];

        self.send_invite_notification(inviting, invited, channel);
        Ok(true)
    }

    fn away_logic(&mut self, arguments: CommandArgs) -> io::Result<bool> {
        let (prefix, _, trail) = arguments;
        self.database.set_away_message(
            prefix.as_ref().expect("Prefix should be Some"),
            trail.clone(),
        );

        let nickname = prefix.expect("Prefix should be Some");
        let message = trail;

        self.send_away_notification(&nickname, &message);
        Ok(true)
    }

    fn topic_logic(&mut self, arguments: CommandArgs) -> io::Result<bool> {
        let (prefix, params, _) = arguments;

        let nickname = &prefix.expect("Prefix should be Some");
        let channel = &params[0];
        let topic = &params[1];
        self.database.set_channel_topic(channel, topic);

        self.send_topic_notification(nickname, channel, topic);
        Ok(true)
    }

    fn kick_logic(&mut self, arguments: CommandArgs) -> io::Result<bool> {
        let (prefix, mut params, trail) = arguments;

        let kicker = prefix.expect("Prefix should be Some");
        let kicked = params.remove(1);
        let channel = params.remove(0);
        let message = trail;

        self.send_kick_notification(&kicker, &channel, &kicked, &message);
        self.database.remove_client_from_channel(&channel, &kicked);

        Ok(true)
    }

    fn mode_logic(&mut self, arguments: CommandArgs) -> io::Result<bool> {
        let (prefix, mut params, _) = arguments;

        let sender = prefix.expect("Prefix should be Some");
        let target = params.remove(0);

        let mode = params
            .get(0)
            .expect("Parameters should have mode")
            .to_string();
        let argument = params.get(1).map(|s| s.to_string()).unwrap_or_default();
        let request = format!("{mode} {argument}");

        self.send_mode_notification(&sender, &target, &request);

        if self.is_channel(&target) {
            self.mode_command_for_channel(target, params);
        } else {
            self.mode_command_for_user(target, params);
        }

        Ok(true)
    }

    fn quit_logic(&mut self, arguments: CommandArgs) -> io::Result<bool> {
        let (prefix, _, trail) = arguments;

        let nickname = prefix.expect("Prefix should be Some");
        let message = trail.expect("Trail should be Some");

        self.database.disconnect_client(&nickname);
        self.send_quit_notification(nickname, message);

        Ok(true)
    }

    fn server_logic(&mut self, arguments: CommandArgs) -> io::Result<bool> {
        let (_, mut params, trail) = arguments;

        let hopcount = params
            .remove(1)
            .parse::<usize>()
            .expect("Hopcount should be a number");
        let servername = params.remove(0);
        let serverinfo = trail.expect("Trail should be Some");

        self.send_server_notification(&servername, hopcount + 1, &serverinfo);

        self.add_server(servername, serverinfo, hopcount);

        Ok(true)
    }

    fn squit_logic(&mut self, arguments: CommandArgs) -> io::Result<bool> {
        let (prefix, params, trail) = arguments;

        let sender = prefix.expect("Prefix should be Some");
        let servername = &params[0];
        let comment = trail;

        self.send_squit_notification(&sender, servername, comment);

        if self.database.is_immediate_server(servername) {
            self.database.remove_server(servername);

            let all_clients = self.database.get_all_clients();
            let server_clients: Vec<ClientInfo> = all_clients
                .into_iter()
                .filter(|client| {
                    let server = ok_or_return!(
                        self.database.get_immediate_server(&client.nickname()),
                        false
                    );
                    server == *servername
                })
                .collect();

            for client in server_clients {
                println!("desconectando al cliente: {}", client.nickname());
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

    fn mode_command_for_channel(&mut self, channel: String, mut args: Vec<String>) {
        let mode_string = args.remove(0);
        let mut mode_arguments = args;
        mode_arguments.reverse();

        let mut mode_requests = parse_channel_mode_string(mode_string, mode_arguments);

        let request = mode_requests.remove(0);

        self.handle_channel_mode_request(&channel, request);
    }

    fn mode_command_for_user(&mut self, user: String, mut args: Vec<String>) {
        let mode_string = args.remove(0);

        let mut mode_requests = parse_user_mode_string(mode_string);
        let request = mode_requests.remove(0);

        self.handle_user_mode_request(&user, request);
    }
}
