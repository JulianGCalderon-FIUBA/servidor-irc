use std::io;
use std::io::{Read, Write};
use std::net::TcpStream;

/// An implementor of ClientTrait must also implent the following traits:
///    - Read
///    - Write
///    - Sized
///    - Send
pub trait ClientTrait: Read + Write + Sized + Send + 'static {
    fn try_clone(&self) -> io::Result<Self>;
}

impl ClientTrait for TcpStream {
    fn try_clone(&self) -> io::Result<Self> {
        self.try_clone()
    }
}
