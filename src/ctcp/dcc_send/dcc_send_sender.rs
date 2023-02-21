use std::{
    fs,
    io::{self, Write},
    net::{TcpListener, TcpStream},
    path::PathBuf,
    thread,
};

use crate::message::CRLF;

use super::file_transfer::FileTransferer;

pub struct DccSendSender {
    listener: TcpListener,
    file: PathBuf,
    filesize: u64,
}

impl DccSendSender {
    pub fn send(mut server: TcpStream, client: String, file: PathBuf) -> io::Result<Self> {
        let listener = TcpListener::bind("0.0.0.0:0")?;

        let address = listener.local_addr()?;

        let ip = address.ip();
        let port = address.port();

        let filesize = fs::metadata(&file)?.len();
        let filename = file.as_path().file_name().unwrap().to_str().unwrap();

        write!(
            server,
            "CTCP {client} :DCC SEND {filename} {ip} {port} {filesize}"
        )?;
        server.write_all(CRLF)?;

        Ok(Self {
            listener,
            file,
            filesize,
        })
    }

    pub fn accept(self) -> io::Result<()> {
        let stream = self.listener.accept()?.0;

        thread::spawn(move || FileTransferer::new(stream, self.file, self.filesize).upload_file());

        Ok(())
    }

    pub fn close(self) {}
}
