use std::fmt::Display;
use std::io;
use std::io::{Read, Write};
use std::net::{Shutdown, SocketAddr, TcpStream};
use std::time::Duration;

use crate::message::Message;

/// An implementor of Connection must behave as a TCPStream for testing purposes
/// For convenience, it also implements `send`
pub trait Connection: Read + Write + Sized + Send + 'static {
    fn try_clone(&self) -> io::Result<Self>;
    fn peer_address(&self) -> io::Result<SocketAddr>;
    fn shutdown(&self) -> io::Result<()>;
    fn set_read_timeout(&self, duration: Option<Duration>) -> io::Result<()>;

    fn send(&mut self, message: &dyn Display) -> io::Result<()> {
        if let Ok(message) = Message::new(&message.to_string()) {
            return message.send_to(self);
        }

        Ok(())
    }
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
