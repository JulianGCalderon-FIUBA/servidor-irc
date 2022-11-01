use std::io;
use std::io::{Read, Write};
use std::net::TcpStream;

pub trait ClientTrait: Read + Write + Sized + Send {
    fn try_clone(&self) -> io::Result<Self>;
}

impl ClientTrait for TcpStream {
    fn try_clone(&self) -> io::Result<Self> {
        self.try_clone()
    }
}
