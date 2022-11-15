use crate::server::connection::Connection;
use crate::server::connection_handler::connection_handler_trait::ConnectionHandlerStructure;

use super::RegistrationHandler;

impl<C: Connection> ConnectionHandlerStructure<C> for RegistrationHandler<C> {
    fn on_try_handle_error(&mut self) {}
    fn on_try_handle_success(&mut self) {}
}
