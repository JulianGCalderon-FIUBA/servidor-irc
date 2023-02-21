use std::{
    fs,
    io::{self, Write},
    net::{IpAddr, TcpListener, TcpStream},
    path::PathBuf,
    str::FromStr,
    thread,
};

use crate::message::CRLF;

use super::file_transfer::FileTransferer;

pub struct DccSendSender {
    server: TcpStream,
    client: String,
    listener: TcpListener,
    path: PathBuf,
}

impl DccSendSender {
    pub fn send(mut server: TcpStream, client: String, path: PathBuf) -> io::Result<Self> {
        let listener = TcpListener::bind("0.0.0.0:0")?;

        let address = listener.local_addr()?;

        let ip = IpAddr::from_str("127.0.0.1").unwrap();
        let port = address.port();

        let filesize = fs::metadata(&path)?.len();
        let filename = path.as_path().file_name().unwrap().to_str().unwrap();

        write!(
            server,
            "CTCP {client} :DCC SEND {filename} {ip} {port} {filesize}"
        )?;
        server.write_all(CRLF)?;

        Ok(Self {
            listener,
            path,
            server,
            client,
        })
    }

    pub fn accept(self) -> io::Result<()> {
        let stream = self.listener.accept()?.0;

        let filesize = fs::metadata(self.path.clone())?.len();

        thread::spawn(move || FileTransferer::new(stream, self.path, filesize).upload_file());

        Ok(())
    }

    pub fn resume(mut self, position: u64) -> io::Result<()> {
        let address = self.listener.local_addr()?;

        let filename = self.path.as_path().file_name().unwrap().to_str().unwrap();

        write!(
            self.server,
            "CTCP {} :DCC ACCEPT {} {} {position}",
            self.client,
            filename,
            address.port(),
        )?;
        self.server.write_all(CRLF)?;

        let stream = self.listener.accept()?.0;
        let filesize = fs::metadata(self.path.clone())?.len();

        FileTransferer::new(stream, self.path, filesize).resume_upload_file(position)
    }

    pub fn close(self) {}
}
