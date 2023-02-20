use std::{
    io::{self, Write},
    net::{SocketAddr, TcpStream},
};

use crate::{ctcp::dcc_message::DccMessage, message::CRLF};

use super::{dcc_resume_sender::DccResumeSender, file_transfer::FileTransferer};

struct DccSendReceiver {
    server: TcpStream,
    client: String,
    address: SocketAddr,
    filename: String,
    filesize: u64,
}

impl DccSendReceiver {
    pub fn new(
        server: TcpStream,
        client: String,
        filename: String,
        filesize: u64,
        address: SocketAddr,
    ) -> Self {
        Self {
            server,
            client,
            address,
            filename,
            filesize,
        }
    }

    pub fn accept_send_command(mut self) -> io::Result<()> {
        write!(self.server, "CTCP {} :DCC SEND accept", self.client)?;
        self.server.write_all(CRLF)?;

        let stream = TcpStream::connect(self.address)?;

        FileTransferer::new(stream, self.filename, self.filesize).download_file()
    }

    pub fn resume_send_command(self, position: u64) -> io::Result<DccResumeSender> {
        DccResumeSender::send(
            self.server,
            self.client,
            self.address,
            self.filename,
            self.filesize,
            position,
        )
    }

    pub fn decline_send_command(mut self) -> io::Result<()> {
        write!(self.server, "CTCP {} :DCC SEND decline", self.client)?;
        self.server.write_all(CRLF)
    }
}
