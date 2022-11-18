use crate::server::connection::Connection;
use crate::server::connection_handler::connection_handler_trait::ConnectionHandlerUtils;

use super::ClientHandler;

impl<C: Connection> ConnectionHandlerUtils<C> for ClientHandler<C> {}

impl<C: Connection> ClientHandler<C> {}
