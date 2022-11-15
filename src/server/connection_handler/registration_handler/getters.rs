use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use crate::server::connection::Connection;
use crate::server::connection_handler::connection_handler_trait::ConnectionHandlerGetters;
use crate::server::database::DatabaseHandle;

use super::RegistrationHandler;

impl<C: Connection> ConnectionHandlerGetters<C> for RegistrationHandler<C> {
    fn online(&self) -> &Arc<AtomicBool> {
        &self.online
    }

    fn connection(&mut self) -> &mut C {
        &mut self.stream
    }

    fn database(&self) -> &DatabaseHandle<C> {
        &self.database
    }
}
