use crate::server::{
    connection::Connection, connection_handler::connection_handler_trait::ConnectionHandlerCommands,
};

use super::RegistrationHandler;

impl<C: Connection> ConnectionHandlerCommands<C> for RegistrationHandler<C> {}
