use std::{
    io::{self, Write},
    net::{SocketAddr, TcpStream},
};

use super::DccChat;
use crate::message::CRLF;

/// Handles the receiving end of a DCC CHAT request.
pub struct DccChatReceiver {
    server: TcpStream,
    client: String,
}

impl DccChatReceiver {
    /// Creates new [`DccChatReceiver`] with information about the client who sent the request.
    pub fn new(server: TcpStream, client: String) -> Self {
        Self { server, client }
    }

    /// Sends command to accept incoming DCC CHAT connection and connects a [`DccChat`] to the received address.
    pub fn accept_chat_command(mut self, address: SocketAddr) -> io::Result<DccChat> {
        write!(self.server, "CTCP {} :DCC CHAT accept", self.client)?;
        self.server.write_all(CRLF)?;

        DccChat::connect(address)
    }

    /// Sends command to decline incoming DCC CHAT connection.
    pub fn decline_chat_command(mut self) -> io::Result<()> {
        write!(self.server, "CTCP {} :DCC CHAT decline", self.client)?;
        self.server.write_all(CRLF)
    }
}
