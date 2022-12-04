/// This module contains the logic to store and build a client.
mod builder;
/// This module contains the structure that stores a clients' public information.
mod client_info;
/// This module contains an external client's logic.
mod external_client;
/// This module contains a local client's logic.
mod local_client;

pub use client_info::ClientInfo;
pub use external_client::ExternalClient;
pub use local_client::LocalClient;

pub use builder::ClientBuilder;
