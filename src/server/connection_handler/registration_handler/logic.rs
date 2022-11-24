use std::io;

use crate::server::connection::Connection;
use crate::server::connection_handler::connection_handler_trait::{
    CommandArgs, ConnectionHandlerLogic,
};
use crate::server::registerer::Register;
use crate::server::responses::CommandResponse;

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
        let servername = self.database.get_server_name();
        let hostname = self.stream.peer_address()?.ip().to_string();

        self.attributes.insert("username", username);
        self.attributes.insert("hostname", hostname);
        self.attributes.insert("servername", servername);
        self.attributes.insert("realname", realname);

        let client = self.build_client();

        self.database.add_local_client(client.unwrap());

        self.connection_type = ConnectionType::Client;

        Ok(false)
    }

    fn server_logic(&mut self, arguments: CommandArgs) -> io::Result<bool> {
        let (_, mut params, trail) = arguments;

        let hopcount = params.remove(1).parse::<usize>().unwrap();
        let servername = params.remove(0);
        let serverinfo = trail.unwrap();

        self.connection_type = ConnectionType::Server;

        let mut registerer = Register::new(self.stream.try_clone()?, self.database.clone());
        registerer.register_incoming(servername, hopcount, serverinfo)?;

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
