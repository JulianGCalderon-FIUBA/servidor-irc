use std::io;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream};

/// An implementor of ClientTrait must also implent the following traits:
///    - Read
///    - Write
///    - Sized
///    - Send
pub trait Connection: Read + Write + Sized + Send + 'static {
    fn try_clone(&self) -> io::Result<Self>;
    fn peer_address(&self) -> io::Result<SocketAddr>;
}

impl Connection for TcpStream {
    fn try_clone(&self) -> io::Result<Self> {
        self.try_clone()
    }

    fn peer_address(&self) -> io::Result<SocketAddr> {
        self.peer_addr()
    }
}
