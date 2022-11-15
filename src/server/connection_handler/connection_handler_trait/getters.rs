use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use crate::server::connection::Connection;
use crate::server::database::DatabaseHandle;

pub trait ConnectionHandlerGetters<C: Connection> {
    fn online(&self) -> &Arc<AtomicBool>;
    fn connection(&mut self) -> &mut C;
    fn database(&self) -> &DatabaseHandle<C>;
}
