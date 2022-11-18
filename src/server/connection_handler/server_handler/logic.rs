use crate::server::connection::Connection;
use crate::server::connection_handler::connection_handler_trait::ConnectionHandlerLogic;

use super::ServerHandler;

impl<C: Connection> ConnectionHandlerLogic<C> for ServerHandler<C> {}
