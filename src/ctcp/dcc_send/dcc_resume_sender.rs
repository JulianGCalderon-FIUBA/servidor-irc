use std::{
    io::{self, Write},
    net::{SocketAddr, TcpStream},
    path::PathBuf,
};

use crate::message::CRLF;

use super::file_transfer::{FileTransferer, TransferController};

pub struct DccResumeSender {
    address: SocketAddr,
    filesize: u64,
    path: PathBuf,
    filename: String,
}

impl DccResumeSender {
    pub fn send(
        mut server: TcpStream,
        client: String,
        address: SocketAddr,
        filename: String,
        filesize: u64,
        position: u64,
        path: PathBuf,
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
            path,
            filename,
        })
    }

    pub fn accept(self) -> io::Result<(FileTransferer, TransferController)> {
        let stream = TcpStream::connect(self.address)?;
        Ok(FileTransferer::new(stream, self.path, self.filesize))
    }

    pub fn close(self) {}

    pub fn original_name(&self) -> String {
        self.filename.clone()
    }

    pub fn path(&self) -> PathBuf {
        self.path.clone()
    }
}
