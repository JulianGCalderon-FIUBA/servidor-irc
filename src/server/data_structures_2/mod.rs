mod channel;
mod client;
mod server;

type ClientId = usize;

pub use channel::Channel;
pub use channel::ChannelConfiguration;
pub use client::ClientInfo;
pub use client::ExternalClient;
pub use client::LocalClient;
pub use server::ImmediateServer;
pub use server::ServerInfo;
