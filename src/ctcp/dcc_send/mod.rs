use std::{
    fs::File,
    io::{self, Read, Write},
    net::TcpStream,
};

mod dcc_send_receiver;
mod dcc_send_sender;

pub struct DccSend {
    stream: TcpStream,
}

impl DccSend {
    // me gusta upload porque combina con download pero quizás es más correcto send y receive
    pub fn upload_file(mut stream: TcpStream, filename: String) -> io::Result<Self> {
        let mut file = File::open(filename)?;

        let filesize = file.metadata()?.len();

        let mut buffer = vec![0; filesize as usize];
        let bytes_read = file.read(&mut buffer)?;

        let bytes_written = stream.write(&buffer)?;

        if bytes_read != bytes_written {
            return Err(io::Error::new(io::ErrorKind::Other, "Error uploading file"));
        }

        Ok(Self { stream })
    }

    pub fn download_file(address: &str) -> io::Result<String> {
        let mut stream = TcpStream::connect(address)?;

        // hay que cambiar el tamaño del buffer para que lea hasta el final sin conocer el tamaño del archivo
        let mut buffer = vec![0; 4096];
        let bytes_read = stream.read(&mut buffer)?;

        if bytes_read != buffer.len() {
            return error_downloading();
        }

        let content = match String::from_utf8((&buffer[0..bytes_read]).to_vec()) {
            Ok(content) => content,
            Err(_) => return error_downloading(),
        };

        Ok(content)
    }
}

fn error_downloading() -> Result<String, io::Error> {
    Err(io::Error::new(
        io::ErrorKind::Other,
        "Error downloading file",
    ))
}
