use crate::server::connection::Connection;
use crate::server::connection_handler::connection_handler_trait::ConnectionHandlerLogic;

use super::RegistrationHandler;

impl<C: Connection> ConnectionHandlerLogic<C> for RegistrationHandler<C> {
    fn pass_logic(&mut self, mut params: Vec<String>) -> std::io::Result<()> {
        let password = params.pop().unwrap();
        self.attributes.insert("password", password);

        Ok(())
    }

    fn nick_logic(&mut self, mut params: Vec<String>) -> std::io::Result<()> {
        let nickname = params.pop().unwrap();
        self.attributes.insert("nickname", nickname);

        Ok(())
    }

    fn user_logic(
        &mut self,
        mut params: Vec<String>,
        trail: Option<String>,
    ) -> std::io::Result<()> {
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

        Ok(())
    }
}
