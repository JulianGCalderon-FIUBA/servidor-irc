use crate::server::connection::Connection;
use crate::server::connection_handler::connection_handler_trait::{
    CommandArgs, ConnectionHandlerLogic,
};
use crate::server::database::ExternalClient;

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
        let (_, mut params, trail) = arguments;

        let nickname = self.hopcounts.keys().next().unwrap().to_string();
        let hopcount = *self.hopcounts.get(&nickname).unwrap();
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
