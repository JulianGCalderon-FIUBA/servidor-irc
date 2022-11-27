use crate::server::connection::Connection;

mod asserts;
mod commands;
mod getters;
mod logic;

mod structure;
mod utils;

pub use asserts::ConnectionHandlerAsserts;
pub use commands::ConnectionHandlerCommands;
pub use getters::ConnectionHandlerGetters;
pub use logic::ConnectionHandlerLogic;

pub use structure::CommandArgs;
pub use structure::ConnectionHandlerStructure;
pub use utils::ConnectionHandlerUtils;

const READ_FROM_STREAM_TIMEOUT_MS: u64 = 100;

pub trait ConnectionHandler<C: Connection>:
    Sized + ConnectionHandlerStructure<C> + ConnectionHandlerGetters<C> + ConnectionHandlerCommands<C>
{
    fn handle(mut self) {
        match self.try_handle() {
            Ok(()) => self.on_try_handle_success(),
            Err(_) => self.on_try_handle_error(),
        }
    }
}
