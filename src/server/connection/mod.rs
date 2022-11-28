mod responses;

use std::io;
use std::io::{Read, Write};
use std::net::{Shutdown, SocketAddr, TcpStream};
use std::time::Duration;

pub use responses::ConnectionResponses;

/// An implementor of ClientTrait must also implent the following traits:
///    - Read
///    - Write
///    - Sized
///    - Send
pub trait Connection: Read + Write + Sized + Send + 'static + ConnectionResponses {
    fn try_clone(&self) -> io::Result<Self>;
    fn peer_address(&self) -> io::Result<SocketAddr>;
    fn shutdown(&self) -> io::Result<()>;
    fn set_read_timeout(&self, duration: Option<Duration>) -> io::Result<()>;
}

impl Connection for TcpStream {
    fn try_clone(&self) -> io::Result<Self> {
        self.try_clone()
    }

    fn peer_address(&self) -> io::Result<SocketAddr> {
        self.peer_addr()
    }

    fn shutdown(&self) -> io::Result<()> {
        self.shutdown(Shutdown::Both)
    }

    fn set_read_timeout(&self, duration: Option<Duration>) -> io::Result<()> {
        self.set_read_timeout(duration)
    }
}