use crate::server::connection::Connection;
use crate::server::connection_handler::connection_handler_trait::ConnectionHandlerCommands;

use super::RegistrationHandler;

impl<C: Connection> ConnectionHandlerCommands<C> for RegistrationHandler<C> {}
