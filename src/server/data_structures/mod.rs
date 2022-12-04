/// This module contains a channel's structure and logic.
/// A channel contains different clients and configurations that can only be modified by a channel operator.
mod channel;
/// This module contains a client's structure and logic.
/// A server stores different client connections and their information.
mod client;
/// This module contains a server's structure and logic.
/// It is used by a local server to store incoming server connections and their information.
mod server;

pub use channel::Channel;
pub use channel::ChannelConfiguration;
pub use client::ClientBuilder;
pub use client::ClientInfo;
pub use client::ExternalClient;
pub use client::LocalClient;
pub use server::ImmediateServer;
pub use server::ServerInfo;
