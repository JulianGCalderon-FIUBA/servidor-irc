use crate::server::connection::Connection;
use crate::server::connection_handler::connection_handler_trait::ConnectionHandlerUtils;

use super::ServerHandler;

impl<C: Connection> ConnectionHandlerUtils<C> for ServerHandler<C> {}
