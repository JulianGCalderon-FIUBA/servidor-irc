/// This module contains an immediate server's logic.
/// An immediate server is directly connected to the local server.
mod immediate_server;
/// This module contains the structure that stores a server's public information.
mod server_info;

pub use immediate_server::ImmediateServer;
pub use server_info::ServerInfo;
