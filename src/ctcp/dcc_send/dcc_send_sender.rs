use std::{
    fs,
    io::{self, Write},
    net::{IpAddr, TcpListener, TcpStream},
    path::{Path, PathBuf},
    str::FromStr,
};

use crate::message::CRLF;

use super::file_transfer::{FileTransferer, TransferController};

/// Handles the sending end of a DCC SEND request.
pub struct DccSendSender {
    server: TcpStream,
    client: String,
    listener: TcpListener,
    filepath: PathBuf,
}

impl DccSendSender {
    ///  Sends DCC SEND request to a client and creates a listener that waits for someone to connect. Returns a [`DccSendSender`] containing information about the client that will receive the request and the file they will receive.
    pub fn send(mut server: TcpStream, client: String, filepath: PathBuf) -> io::Result<Self> {
        let listener = TcpListener::bind("0.0.0.0:0")?;

        let address = listener.local_addr()?;

        let ip = IpAddr::from_str("127.0.0.1").unwrap();
        let port = address.port();

        let filesize = fs::metadata(&filepath)?.len();
        let filename = filename_from_pathbuf(&filepath);

        write!(
            server,
            "CTCP {client} :DCC SEND {filename} {ip} {port} {filesize}"
        )?;
        server.write_all(CRLF)?;

        Ok(Self {
            listener,
            filepath,
            server,
            client,
        })
    }

    /// Creates new [`FileTransferer`] with information about the file and the stream to send it through.
    pub fn accept(self) -> io::Result<(FileTransferer, TransferController)> {
        let stream = self.listener.accept()?.0;
        let filesize = fs::metadata(&self.filepath)?.len();

        Ok(FileTransferer::new(stream, self.filepath, filesize))
    }

    /// Called when the request is rejected. Drops [`DccSendSender`].
    pub fn decline(self) {}

    /// Accepts a request to resume the process of sending a file. Creates a new [`FileTransferer`] with updated information about the file and the stream to send it through.
    pub fn resume(mut self, position: u64) -> io::Result<(FileTransferer, TransferController)> {
        let port = self.listener.local_addr()?.port();
        let filename = filename_from_pathbuf(&self.filepath);

        write!(
            self.server,
            "CTCP {} :DCC ACCEPT {} {} {position}",
            self.client, filename, port,
        )?;
        self.server.write_all(CRLF)?;

        let stream = self.listener.accept()?.0;
        let filesize = fs::metadata(&self.filepath)?.len();

        Ok(FileTransferer::new(stream, self.filepath.clone(), filesize))
    }
}

fn filename_from_pathbuf(path: &Path) -> String {
    path.file_name()
        .expect("filename must not terminate in \"..\"")
        .to_str()
        .expect("filename must be valid unicode")
        .to_string()
}
