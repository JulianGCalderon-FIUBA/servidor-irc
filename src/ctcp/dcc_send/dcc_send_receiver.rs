use std::{
    io::{self, Write},
    net::TcpStream,
};

use crate::message::CRLF;

use super::DccSend;

struct DccSendReceiver {
    server: TcpStream,
    client: String,
}

impl DccSendReceiver {
    pub fn new(server: TcpStream, client: String) -> Self {
        Self { server, client }
    }

    pub fn accept_send_command(mut self, address: &str) -> io::Result<String> {
        write!(self.server, "CTCP {} :DCC SEND accept", self.client)?;
        self.server.write_all(CRLF)?;

        DccSend::download_file(address)
    }

    pub fn decline_send_command(mut self) -> io::Result<()> {
        write!(self.server, "CTCP {} :DCC SEND decline", self.client)?;
        self.server.write_all(CRLF)
    }
}
