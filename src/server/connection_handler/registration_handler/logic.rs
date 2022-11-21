use std::io;

use crate::server::connection::Connection;
use crate::server::connection_handler::connection_handler_trait::{
    CommandArgs, ConnectionHandlerLogic, ConnectionHandlerUtils,
};
use crate::server::connection_handler::responses::Notification;
use crate::server::database::ClientInfo;

use super::connection_type::ConnectionType;
use super::RegistrationHandler;

impl<C: Connection> ConnectionHandlerLogic<C> for RegistrationHandler<C> {
    fn pass_logic(&mut self, arguments: CommandArgs) -> io::Result<bool> {
        let (_, mut params, _) = arguments;
        let password = params.pop().unwrap();
        self.attributes.insert("password", password);

        Ok(true)
    }

    fn nick_logic(&mut self, arguments: CommandArgs) -> io::Result<bool> {
        let (_, mut params, _) = arguments;
        let nickname = params.pop().unwrap();
        self.attributes.insert("nickname", nickname);

        Ok(true)
    }

    fn user_logic(&mut self, arguments: CommandArgs) -> std::io::Result<bool> {
        let (_, mut params, trail) = arguments;

        let realname = trail.unwrap();
        let username = params.pop().unwrap();
        let servername = self.servername.to_string();
        let hostname = self.stream.peer_address()?.ip().to_string();

        self.attributes.insert("username", username);
        self.attributes.insert("hostname", hostname);
        self.attributes.insert("servername", servername);
        self.attributes.insert("realname", realname);

        let client = self.build_client();

        self.database.add_client(client.unwrap());

        self.connection_type = ConnectionType::Client;

        Ok(false)
    }

    fn server_logic(&mut self, arguments: CommandArgs) -> io::Result<bool> {
        let (_, mut params, trail) = arguments;

        let hopcount = params.remove(1);
        let servername = params.remove(0);
        let serverinfo = trail.unwrap();

        self.attributes.insert("servername", servername);
        self.attributes.insert("hopcount", hopcount);
        self.attributes.insert("serverinfo", serverinfo);

        let server = self.build_server().unwrap();
        self.database.add_server(server);

        self.connection_type = ConnectionType::Server;

        self.send_server_response()?;
        self.send_server_info()?;

        Ok(false)
    }

    fn quit_logic(&mut self, arguments: CommandArgs) -> io::Result<bool> {
        let (_, _, trail) = arguments;

        let message =
            trail.unwrap_or_else(|| self.attributes.remove("nickname").unwrap_or_default());

        let notification = Notification::Quit { message };

        self.send_response(&notification.to_string())?;

        Ok(false)
    }
}

impl<C: Connection> RegistrationHandler<C> {
    fn send_server_response(&mut self) -> Result<(), io::Error> {
        let server_notification = format!("SERVER {} {} :{}", self.servername, 1, "hola");
        self.send_response(&server_notification)?;
        Ok(())
    }

    fn send_server_info(&mut self) -> Result<(), io::Error> {
        for client in self.database.get_all_clients() {
            self.send_nick_notification(&client)?;
            self.send_user_notification(&client)?;
        }
        Ok(())
    }

    fn send_user_notification(&mut self, client: &ClientInfo) -> Result<(), io::Error> {
        let user_notification = Notification::User {
            nickname: client.nickname.clone(),
            username: client.username.clone(),
            hostname: client.hostname.clone(),
            servername: client.servername.clone(),
            realname: client.realname.clone(),
        };
        self.send_response(&user_notification)?;
        Ok(())
    }

    fn send_nick_notification(&mut self, client: &ClientInfo) -> Result<(), io::Error> {
        let nick_notification = Notification::Nick {
            nickname: client.nickname.clone(),
            hopcount: client.hopcount,
        };
        self.send_response(&nick_notification)?;
        Ok(())
    }
}
