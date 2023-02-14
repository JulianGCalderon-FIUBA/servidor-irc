use std::{
    fs,
    io::{self, Write},
    net::{TcpListener, TcpStream},
};

use crate::message::CRLF;

use super::file_transfer::FileTransferer;

struct DccSendSender {
    listener: TcpListener,
    filename: String,
    filesize: u64,
}

impl DccSendSender {
    pub fn send(mut server: TcpStream, client: String, filename: String) -> io::Result<Self> {
        let listener = TcpListener::bind("0.0.0.0:0")?;

        let address = listener.local_addr()?;

        let ip = address.ip();
        let port = address.port();

        let filesize = fs::metadata(filename.clone())?.len();

        write!(
            server,
            "CTCP {client} :DCC SEND {filename} {ip} {port} {filesize}"
        )?;
        server.write_all(CRLF)?;

        Ok(Self {
            listener,
            filename,
            filesize,
        })
    }

    pub fn accept(self) -> io::Result<()> {
        let stream = self.listener.accept()?.0;

        FileTransferer::new(stream, self.filename, self.filesize).upload_file()
    }

    pub fn close(self) {}
}
