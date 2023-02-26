use std::{
    io::{self, Write},
    net::{TcpListener, TcpStream},
};

use crate::message::CRLF;

use super::DccChat;

pub struct DccChatSender {
    _server: TcpStream,
    _client: String,
    listener: TcpListener,
}

impl DccChatSender {
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

    pub fn accept(self) -> io::Result<DccChat> {
        let stream = self.listener.accept()?.0;
        DccChat::new(stream)
    }

    pub fn close(self) {}
}
