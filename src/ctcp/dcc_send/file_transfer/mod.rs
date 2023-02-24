mod transfer_controller;

use std::{
    fs::File,
    io::{self, Read, Seek, SeekFrom, Write},
    net::TcpStream,
    path::PathBuf,
    sync::{atomic::AtomicBool, Arc},
};

pub use transfer_controller::TransferController;

pub struct FileTransferer {
    stream: TcpStream,
    filename: PathBuf,
    filesize: u64,
    cancelled: Arc<AtomicBool>,
}

impl FileTransferer {
    pub fn new(stream: TcpStream, filename: PathBuf, filesize: u64) -> (Self, TransferController) {
        let cancelled = Arc::new(AtomicBool::new(false));
        let file_transferer = Self {
            stream,
            filename,
            filesize,
            cancelled: Arc::clone(&cancelled),
        };

        let controller = TransferController::new(cancelled);

        (file_transferer, controller)
    }

    pub fn upload_file(self) -> io::Result<()> {
        self.resume_upload_file(0)
    }

    pub fn download_file(self) -> io::Result<()> {
        let file = File::create(self.filename.clone())?;

        self.to_file(file)
    }

    pub fn resume_upload_file(self, position: u64) -> io::Result<()> {
        let mut file = File::open(self.filename.clone())?;
        file.seek(SeekFrom::Start(position))?;

        self.to_stream(file)
    }

    pub fn resume_download_file(self, position: u64) -> io::Result<()> {
        let mut file = File::open(self.filename.clone())?;
        file.seek(SeekFrom::Start(position))?;

        self.to_file(file)
    }

    pub fn to_stream(mut self, mut file: File) -> io::Result<()> {
        let total_bytes_read = copy(self.cancelled, &mut file, &mut self.stream)?;

        if total_bytes_read != self.filesize {
            return Err(error_uploading());
        }

        Ok(())
    }

    pub fn to_file(mut self, mut file: File) -> io::Result<()> {
        let total_bytes_read = copy(self.cancelled, &mut self.stream, &mut file)?;

        if total_bytes_read != self.filesize {
            return Err(error_downloading());
        }

        Ok(())
    }
}

fn copy<R: Read, W: Write>(
    cancelled: Arc<AtomicBool>,
    from: &mut R,
    to: &mut W,
) -> Result<u64, io::Error> {
    let mut buffer = [0; 1024];
    let mut total_bytes_read = 0;
    while !cancelled.load(std::sync::atomic::Ordering::Relaxed) {
        let bytes_read = from.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        to.write_all(&buffer[..bytes_read])?;
        total_bytes_read += bytes_read as u64;
    }
    to.flush()?;

    Ok(total_bytes_read)
}

fn error_uploading() -> io::Error {
    io::Error::new(io::ErrorKind::Other, "Error uploading file")
}

fn error_downloading() -> io::Error {
    io::Error::new(io::ErrorKind::Other, "Error downloading file")
}
