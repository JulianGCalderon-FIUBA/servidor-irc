use std::io;

use crate::server::connection::Connection;
use crate::server::connection_handler::connection_handler_trait::{
    CommandArgs, ConnectionHandlerLogic, ConnectionHandlerUtils,
};
use crate::server::database::ExternalClient;
use crate::server::responses::Notification;

use super::ServerHandler;

impl<C: Connection> ConnectionHandlerLogic<C> for ServerHandler<C> {
    fn nick_logic(&mut self, arguments: CommandArgs) -> std::io::Result<bool> {
        let (_, mut params, _) = arguments;
        let hopcount = params.pop().unwrap().parse::<usize>().unwrap();
        let nickname = params.pop().unwrap();

        self.hopcounts.insert(nickname, hopcount);

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

        let client =
            ExternalClient::new(nickname, username, hostname, servername, realname, hopcount);

        self.database.add_external_client(&self.servername, client);

        Ok(true)
    }

    fn privmsg_logic(&mut self, arguments: CommandArgs) -> std::io::Result<bool> {
        let (prefix, mut params, trail) = arguments;

        let sender = prefix.unwrap();
        let target = params.remove(0);
        let content = trail.unwrap();

        self.send_privmsg_notification(&sender, &target, &content)?;

        Ok(true)
    }
}

impl<C: Connection> ServerHandler<C> {
    pub(super) fn send_privmsg_notification(
        &mut self,
        sender: &str,
        target: &str,
        content: &str,
    ) -> Result<(), io::Error> {
        let notification = Notification::Privmsg {
            sender: sender.to_string(),
            target: target.to_string(),
            message: content.to_owned(),
        };
        self.send_message_to_target(&notification, target)
    }
}
