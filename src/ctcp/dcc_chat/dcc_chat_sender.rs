use std::{
    io::{self, Write},
    net::{TcpListener, TcpStream},
};

use crate::message::CRLF;

use super::DccChat;

/// Handles the sending end of a DCC CHAT request.
pub struct DccChatSender {
    _server: TcpStream,
    _client: String,
    listener: TcpListener,
}

impl DccChatSender {
    ///  Sends DCC CHAT request to a client and creates listener that waits for someone to connect. Returns a [`DccChatSender`] containing information about the client that will receive the request.
    pub fn send(mut server: TcpStream, client: String) -> io::Result<Self> {
        let listener = TcpListener::bind("127.0.0.1:9001")?;

        let address = listener.local_addr()?;

        let ip = address.ip();
        let port = address.port();

        write!(server, "CTCP {client} :DCC CHAT chat {ip} {port}")?;
        server.write_all(CRLF)?;

        Ok(Self {
            _server: server,
            _client: client,
            listener,
        })
    }

    /// Creates new [`DccChat`] with the stream that connected to the listener.
    pub fn accept(self) -> io::Result<DccChat> {
        let stream = self.listener.accept()?.0;
        DccChat::new(stream)
    }

    /// Called when the request is rejected. Drops [`DccChatSender`].
    pub fn close(self) {}
}
