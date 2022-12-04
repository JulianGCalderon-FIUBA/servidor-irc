/// Contains Client Handler's logic.
/// A Client Handler is a type of Connection Handler and must implement it's functionalities.
/// Handles a connection with a client and the commands they might send.
mod client_handler;
/// Contains all logic regarding the mode command, this includes channel modes and user modes.
mod mode_requests;
/// Contains Registration Handler's logic.
/// A Registration Handler is a type of Connection Handler and must implement it's functionalities.
/// Handles a new connection and registration commands, then creates a Client Handler or Server Handler depending on who registered.
mod registration_handler;
/// Contains Server Handler's logic.
/// A Server Handler is a type of Connection Handler and must implement it's functionalities.
/// Handles a connection with a server and the commands they might send.
mod server_handler;

pub use registration_handler::RegistrationHandler;
pub use server_handler::ServerHandler;

use crate::server::connection::Connection;

/// Contains asserts for every command.
/// Each handler must implement the asserts it needs.
mod asserts;
/// Contains all commands.
/// Each handler must implement the ones it needs.
mod commands;
/// Contains getters for a server's information.
/// Each handler must implement the ones it needs.
mod getters;
/// Contains each command's logic.
/// Each handler must implement the asserts it needs.
mod logic;

/// Contains the structure and main functionalities all handlers share.
mod structure;
/// Contains auxiliars all handlers use.
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
