//! # Internet Relay Chat
//! This crate implements the Internet Relay Chat protocol, specifically RFC1459.

/// This module contains a client's functionality. A client is created connected to a stream; through it the client can send and receive messages.
pub mod client;

/// This module contains a server's functionality. A server listens through an address waiting to hear from and then handle a client.
pub mod server;

/// This module contains a message's functionality. A message can be sent to and read from a stream.
pub mod message;

pub mod thread_pool;

pub const ADDRESS: &str = "127.0.0.1:9001";
