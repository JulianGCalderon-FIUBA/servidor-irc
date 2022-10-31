//! # Internet Relay Chat
//! This crate implements the Internet Relay Chat protocol, specifically RFC1459.

/// This module contains a client's functionality. A client is created connected to a stream; through it the client can send and receive messages.
pub mod client;

/// This module contains a server's functionality. A server listens through an address waiting to hear from and then handle a client.
pub mod server;

/// This module contains a message's functionality. A message can be sent to and read from a stream.
pub mod message;

/// This module contains a threadPool's functionality. A threadPool creates n threads to handle n requests simultaneously.
pub mod thread_pool;

pub mod views;

pub mod controller_register;

pub mod view_register;

pub const ADDRESS: &str = "127.0.0.1:9002";
