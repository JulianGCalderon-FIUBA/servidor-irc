//! # Internet Relay Chat
//! This crate implements the Internet Relay Chat protocol, specifically RFC1459.

/// This module contains a client's functionality. A client is created connected to a stream;
/// through it the client can send and receive messages.
pub mod client;

/// This module contains a server's functionality.
/// A server listens through an address waiting to hear from and then handle a client.
/// Multiple clients can connect to a single server, a handler thread is created for each client.
/// A server han handle connection from other servers, sharing information throughout the network.
pub mod server;

/// This module contains a message's functionality.
/// A message can be sent to and read from a stream.
/// Each message is parsed in: prefix, command, parameters, trailing
pub mod message;

/// This module contains a threadPool's functionality.
/// A threadPool creates n threads to handle n requests simultaneously.
/// This prevents an overflow attack on the server
pub mod thread_pool;

pub mod controller;
pub mod views;

/// This module contains useful macros used across the project
mod macros;

/// Default parameters for client-server connection
pub const ADDRESS: &str = "127.0.0.1:9000";
pub const SERVERNAME: &str = "lemonpie";
