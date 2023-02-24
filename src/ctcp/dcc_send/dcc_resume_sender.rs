use std::{
    io::{self, Write},
    net::{SocketAddr, TcpStream},
    path::PathBuf,
};

use crate::message::CRLF;

use super::file_transfer::FileTransferer;

pub struct DccResumeSender {
    address: SocketAddr,
    filesize: u64,
    position: u64,
}

impl DccResumeSender {
    pub fn send(
        mut server: TcpStream,
        client: String,
        address: SocketAddr,
        filename: String,
        filesize: u64,
        position: u64,
    ) -> io::Result<Self> {
        write!(
            server,
            "CTCP {} :DCC RESUME {} {} {position}",
            client,
            filename,
            address.port(),
        )?;
        server.write_all(CRLF)?;

        Ok(Self {
            address,
            filesize,
            position,
        })
    }

    pub fn accept(self, filepath: PathBuf) -> io::Result<()> {
        let stream = TcpStream::connect(self.address)?;
        FileTransferer::new(stream, filepath, self.filesize)
            .0
            .resume_download_file(self.position)
    }

    pub fn close(self) {}
}
