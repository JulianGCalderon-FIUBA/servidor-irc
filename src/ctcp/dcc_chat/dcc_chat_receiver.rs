use std::{
    io::{self, Write},
    net::{SocketAddr, TcpStream},
};

use super::DccChat;
use crate::message::CRLF;

struct DccChatReceiver {
    server: TcpStream,
    client: String,
}

impl DccChatReceiver {
    pub fn new(server: TcpStream, client: String) -> Self {
        Self { server, client }
    }

    pub fn accept_chat_command(mut self, address: SocketAddr) -> io::Result<DccChat> {
        write!(self.server, "CTCP {} :DCC CHAT accept", self.client)?;
        self.server.write_all(CRLF)?;

        DccChat::connect(address)
    }

    pub fn decline_chat_command(mut self) -> io::Result<()> {
        write!(self.server, "CTCP {} :DCC CHAT decline", self.client)?;
        self.server.write_all(CRLF)
    }
}
