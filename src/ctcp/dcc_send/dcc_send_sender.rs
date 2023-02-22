use std::{
    fs,
    io::{self, Write},
    net::{IpAddr, TcpListener, TcpStream},
    path::{Path, PathBuf},
    str::FromStr,
    thread,
};

use crate::message::CRLF;

use super::file_transfer::FileTransferer;

pub struct DccSendSender {
    server: TcpStream,
    client: String,
    listener: TcpListener,
    filepath: PathBuf,
}

impl DccSendSender {
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

    pub fn accept(self) -> io::Result<()> {
        let stream = self.listener.accept()?.0;
        let filesize = fs::metadata(&self.filepath)?.len();

        thread::spawn(move || FileTransferer::new(stream, self.filepath, filesize).upload_file());

        Ok(())
    }

    pub fn decline(self) {}

    pub fn resume(mut self, position: u64) -> io::Result<()> {
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

        thread::spawn(move || {
            FileTransferer::new(stream, self.filepath.clone(), filesize)
                .resume_upload_file(position)
                .unwrap();
        });

        Ok(())
    }
}

fn filename_from_pathbuf(path: &Path) -> String {
    path.file_name()
        .expect("filename must not terminate in \"..\"")
        .to_str()
        .expect("filename must be valid unicode")
        .to_string()
}
