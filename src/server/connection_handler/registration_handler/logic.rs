use std::io;

use crate::server::connection::Connection;
use crate::server::connection_handler::{CommandArgs, ConnectionHandlerLogic};
use crate::server::responses::CommandResponse;
use crate::server::server_connection_setup::ServerConnectionSetup;

use super::connection_type::ConnectionType;
use super::RegistrationHandler;

impl<C: Connection> ConnectionHandlerLogic<C> for RegistrationHandler<C> {
    fn pass_logic(&mut self, arguments: CommandArgs) -> io::Result<bool> {
        let (_, mut params, _) = arguments;

        let password = params.remove(0);

        self.attributes.insert("password", password);

        Ok(true)
    }

    fn nick_logic(&mut self, arguments: CommandArgs) -> io::Result<bool> {
        let (_, mut params, _) = arguments;

        let nickname = params.remove(0);

        self.attributes.insert("nickname", nickname);

        Ok(true)
    }

    fn user_logic(&mut self, arguments: CommandArgs) -> std::io::Result<bool> {
        let (_, mut params, trail) = arguments;

        let realname = trail.expect("Verified in assert");
        let username = params.remove(0);
        let servername = self.database.get_server_name();
        let hostname = self.stream.peer_address()?.ip().to_string();

        self.attributes.insert("username", username);
        self.attributes.insert("hostname", hostname);
        self.attributes.insert("servername", servername);
        self.attributes.insert("realname", realname);

        let client = self
            .build_client()
            .expect("Client's information should be complete to build");

        self.send_new_client_notification(&client.get_info());
        self.database.add_local_client(client);

        self.connection_type = ConnectionType::Client;

        Ok(false)
    }

    fn server_logic(&mut self, arguments: CommandArgs) -> io::Result<bool> {
        let (_, mut params, trail) = arguments;

        let hopcount = params
            .remove(1)
            .parse::<usize>()
            .expect("Verified in assert");
        let servername = params.remove(0);
        let serverinfo = trail.expect("Verified in assert");

        self.send_server_notification(&servername, hopcount, &serverinfo);

        let mut registerer =
            ServerConnectionSetup::new(self.stream.try_clone()?, self.database.clone());
        registerer.register_incoming(servername, hopcount, serverinfo)?;

        self.connection_type = ConnectionType::Server;
        self.attributes
            .insert("servername", registerer.servername());

        Ok(false)
    }

    fn quit_logic(&mut self, arguments: CommandArgs) -> io::Result<bool> {
        let (_, _, trail) = arguments;

        let message =
            trail.unwrap_or_else(|| self.attributes.remove("nickname").unwrap_or_default());

        self.stream.send(&CommandResponse::quit(&message))?;

        Ok(false)
    }
}
