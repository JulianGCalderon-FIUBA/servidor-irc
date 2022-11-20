use crate::server::connection::Connection;
use crate::server::connection_handler::connection_handler_trait::ConnectionHandlerLogic;

use super::ServerHandler;

impl<C: Connection> ConnectionHandlerLogic<C> for ServerHandler<C> {
    fn pass_logic(&mut self, _params: Vec<String>) -> std::io::Result<bool> {
        println!("sos un servidor");

        Ok(true)
    }
}
