use std::{
    io::{self, Write},
    net::{SocketAddr, TcpStream},
};

use crate::message::CRLF;

use super::file_transfer::FileTransferer;

struct DccSendReceiver {
    server: TcpStream,
    client: String,
}

impl DccSendReceiver {
    pub fn new(server: TcpStream, client: String) -> Self {
        Self { server, client }
    }

    pub fn accept_send_command(
        mut self,
        address: SocketAddr,
        filename: String,
        filesize: u64,
    ) -> io::Result<()> {
        write!(self.server, "CTCP {} :DCC SEND accept", self.client)?;
        self.server.write_all(CRLF)?;

        let stream = TcpStream::connect(address)?;

        FileTransferer::new(stream, filename, filesize).download_file()
    }

    pub fn decline_send_command(mut self) -> io::Result<()> {
        write!(self.server, "CTCP {} :DCC SEND decline", self.client)?;
        self.server.write_all(CRLF)
    }
}
