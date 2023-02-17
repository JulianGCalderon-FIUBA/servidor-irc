use std::{
    io::{self, Write},
    net::TcpStream,
};

use super::dcc_chat::DccChat;
use crate::message::CRLF;

pub struct DccChatReceiver {
    server: TcpStream,
    client: String,
}

impl DccChatReceiver {
    pub fn new(server: TcpStream, client: String) -> Self {
        Self { server, client }
    }

    pub fn accept_chat_command(mut self, address: &str) -> io::Result<DccChat> {
        write!(self.server, "CTCP {} :DCC CHAT accept", self.client)?;
        self.server.write_all(CRLF)?;

        DccChat::connect(address)
    }

    pub fn decline_chat_command(mut self) -> io::Result<()> {
        write!(self.server, "CTCP {} :DCC CHAT decline", self.client)?;
        self.server.write_all(CRLF)
    }
}
