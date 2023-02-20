use std::{
    fs,
    io::{self, Write},
    net::{IpAddr, TcpListener, TcpStream},
    str::FromStr,
};

use crate::message::CRLF;

use super::file_transfer::FileTransferer;

struct DccSendSender {
    server: TcpStream,
    client: String,
    listener: TcpListener,
    filename: String,
}

impl DccSendSender {
    pub fn send(mut server: TcpStream, client: String, filename: String) -> io::Result<Self> {
        let listener = TcpListener::bind("0.0.0.0:0")?;

        let address = listener.local_addr()?;

        let ip = IpAddr::from_str("127.0.0.1").unwrap();
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
            server,
            client,
        })
    }

    pub fn accept(self) -> io::Result<()> {
        let stream = self.listener.accept()?.0;

        let filesize = fs::metadata(self.filename.clone())?.len();

        FileTransferer::new(stream, self.filename, filesize).upload_file()
    }

    pub fn resume(mut self, position: u64) -> io::Result<()> {
        let address = self.listener.local_addr()?;

        write!(
            self.server,
            "CTCP {} :DCC ACCEPT {} {} {position}",
            self.client,
            self.filename,
            address.port(),
        )?;
        self.server.write_all(CRLF)?;

        let stream = self.listener.accept()?.0;
        let filesize = fs::metadata(self.filename.clone())?.len();

        FileTransferer::new(stream, self.filename, filesize).resume_upload_file(position)
    }

    pub fn close(self) {}
}
