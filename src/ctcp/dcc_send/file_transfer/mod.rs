mod transfer_controller;

use std::{
    fs::{File, OpenOptions},
    io::{self, Read, Seek, SeekFrom, Write},
    net::TcpStream,
    path::PathBuf,
    sync::{atomic::AtomicBool, Arc},
};

pub use transfer_controller::TransferController;

/// Responsible of sending and receiving file via a TcpStream
/// Can be cancelled using the corresponding controller
pub struct FileTransferer {
    stream: TcpStream,
    filename: PathBuf,
    filesize: u64,
    cancelled: Arc<AtomicBool>,
}

impl FileTransferer {
    /// Creates a file transferer
    /// returns itself, and it's corresponding controller.
    /// Each transferer is created from a stream and a file,
    /// the desired operation is choosed with the corresponding method
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
    /// Sends file through the stream
    /// File must exist
    pub fn upload_file(self) -> io::Result<()> {
        self.resume_upload_file(0)
    }
    /// Reads file from the stream and saves to corresponding file
    /// File will be created if not found, or truncated
    pub fn download_file(self) -> io::Result<()> {
        let file = File::create(self.filename.clone())?;

        self.send_to_file(file)
    }

    /// Sends file through the stream, starting at position.
    pub fn resume_upload_file(mut self, position: u64) -> io::Result<()> {
        let mut file = File::open(self.filename.clone())?;
        file.seek(SeekFrom::Start(position))?;

        self.filesize -= position;

        self.send_to_stream(file)
    }

    /// Reads file through the stream, starting at position.
    /// File must exist
    pub fn resume_download_file(mut self, position: u64) -> io::Result<()> {
        let mut file = OpenOptions::new()
            .append(true)
            .open(self.filename.clone())?;
        file.seek(SeekFrom::Start(position))?;

        self.filesize -= position;

        self.send_to_file(file)
    }

    fn send_to_stream(mut self, mut file: File) -> io::Result<()> {
        let total_bytes_read = copy(self.cancelled.clone(), &mut file, &mut self.stream)?;

        if total_bytes_read != self.filesize {
            return Err(eof());
        }

        Ok(())
    }

    fn send_to_file(mut self, mut file: File) -> io::Result<()> {
        let total_bytes_read = copy(self.cancelled, &mut self.stream, &mut file)?;

        if total_bytes_read != self.filesize {
            return Err(eof());
        }

        Ok(())
    }
}

/// Reads the entire content of 'from' and writes it to 'to' using buffer of size 1024.
/// Operation can be interrupted storing 'true' in 'cancelled'.
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

    if cancelled.load(std::sync::atomic::Ordering::Relaxed) {
        return Err(interrupted());
    }

    to.flush()?;

    Ok(total_bytes_read)
}

fn eof() -> io::Error {
    io::Error::new(
        io::ErrorKind::UnexpectedEof,
        "transfer could not be completed",
    )
}

fn interrupted() -> io::Error {
    io::Error::new(io::ErrorKind::Interrupted, "transfer was interrupted")
}
