use std::{
    fs::{self, File},
    io::{self, Write},
    net::{TcpListener, TcpStream},
};

use crate::message::CRLF;

use super::DccSend;

struct DccSendSender {
    server: TcpStream,
    client: String,
    filename: String,
    listener: TcpListener,
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
            server,
            client,
            listener,
            filename,
        })
    }

    pub fn accept(self) -> io::Result<DccSend> {
        let stream = self.listener.accept()?.0;
        DccSend::upload_file(stream, self.filename)
    }

    pub fn close(self) {}
}
