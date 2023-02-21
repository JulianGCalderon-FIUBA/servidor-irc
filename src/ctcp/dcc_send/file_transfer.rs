use std::{fs::File, io, net::TcpStream, path::PathBuf};

pub struct FileTransferer {
    stream: TcpStream,
    filename: PathBuf,
    filesize: u64,
}

impl FileTransferer {
    pub fn new(stream: TcpStream, filename: PathBuf, filesize: u64) -> Self {
        Self {
            stream,
            filename,
            filesize,
        }
    }

    pub fn upload_file(mut self) -> io::Result<()> {
        let mut file = File::open(self.filename)?;

        println!("comienzo la subida del archivo");

        let bytes_uploaded = io::copy(&mut file, &mut self.stream)?;

        if bytes_uploaded != self.filesize {
            return Err(error_uploading());
        }

        println!("finalizo la subida del archivo");

        Ok(())
    }

    pub fn download_file(mut self) -> io::Result<()> {
        let mut file = File::create(self.filename)?;

        println!("creo el archivo");

        let bytes_downloaded = io::copy(&mut self.stream, &mut file)?;

        if bytes_downloaded != self.filesize {
            return Err(error_downloading());
        }

        println!("termino la descarga el archivo");

        Ok(())
    }
}

fn error_uploading() -> io::Error {
    io::Error::new(io::ErrorKind::Other, "Error uploading file")
}

fn error_downloading() -> io::Error {
    io::Error::new(io::ErrorKind::Other, "Error downloading file")
}
