use crate::server::connection::Connection;
use crate::server::connection_handler::connection_handler_trait::ConnectionHandlerLogic;
use crate::server::database::ExternalClient;

use super::ServerHandler;

impl<C: Connection> ConnectionHandlerLogic<C> for ServerHandler<C> {
    fn nick_logic(&mut self, mut params: Vec<String>) -> std::io::Result<bool> {
        let hopcount = params.pop().unwrap().parse::<usize>().unwrap();
        let nickname = params.pop().unwrap();

        self.hopcounts.insert(nickname, hopcount);

        Ok(true)
    }

    fn user_logic(
        &mut self,
        mut params: Vec<String>,
        trail: Option<String>,
    ) -> std::io::Result<bool> {
        let nickname = self.hopcounts.keys().next().unwrap().to_string();
        let hopcount = *self.hopcounts.get("hopcount").unwrap();
        let servername = params.pop().unwrap();
        let hostname = params.pop().unwrap();
        let username = params.pop().unwrap();
        let realname = trail.unwrap();

        let client =
            ExternalClient::_new(nickname, username, hostname, servername, realname, hopcount);

        self.database.add_external_client(&self._servername, client);

        Ok(true)
    }
}
