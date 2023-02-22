use std::{
    fs::File,
    io::{self, Seek, SeekFrom},
    net::TcpStream,
    path::PathBuf,
};

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

        let bytes_uploaded = io::copy(&mut file, &mut self.stream)?;

        if bytes_uploaded != self.filesize {
            return Err(error_uploading());
        }

        Ok(())
    }

    pub fn download_file(mut self) -> io::Result<()> {
        let mut file = File::create(self.filename)?;

        let bytes_downloaded = io::copy(&mut self.stream, &mut file)?;

        if bytes_downloaded != self.filesize {
            return Err(error_downloading());
        }

        Ok(())
    }

    pub fn resume_upload_file(mut self, position: u64) -> io::Result<()> {
        let mut file = File::open(self.filename)?;
        file.seek(SeekFrom::Start(position))?;

        let bytes_uploaded = io::copy(&mut file, &mut self.stream)?;

        if bytes_uploaded != (self.filesize - position) {
            return Err(error_uploading());
        }

        Ok(())
    }

    pub fn resume_download_file(mut self, position: u64) -> Result<(), io::Error> {
        let mut file = File::open(self.filename)?;
        file.seek(SeekFrom::Start(position))?;

        let bytes_downloaded = io::copy(&mut self.stream, &mut file)?;

        if bytes_downloaded != (self.filesize - position) {
            return Err(error_downloading());
        }

        Ok(())
    }
}

fn error_uploading() -> io::Error {
    io::Error::new(io::ErrorKind::Other, "Error uploading file")
}

fn error_downloading() -> io::Error {
    io::Error::new(io::ErrorKind::Other, "Error downloading file")
}
