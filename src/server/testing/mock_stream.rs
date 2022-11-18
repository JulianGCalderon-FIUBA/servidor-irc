use std::io;
use std::io::{Read, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::{Arc, Mutex, MutexGuard};

use crate::server::connection::Connection;

#[derive(Debug)]
/// Used for testing. It allows the programmer to create a connection and use both ends of the stream.
pub struct MockTcpStream {
    read_buffer: Arc<Mutex<Vec<u8>>>,
    write_buffer: Arc<Mutex<Vec<u8>>>,
}

impl Read for MockTcpStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.read_lock().as_slice().read(buf)
    }
}

impl Write for MockTcpStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.write_lock().write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.write_lock().flush()
    }
}

impl Connection for MockTcpStream {
    fn try_clone(&self) -> io::Result<Self> {
        let clone = Self {
            read_buffer: Arc::clone(&self.read_buffer),
            write_buffer: Arc::clone(&self.write_buffer),
        };

        Ok(clone)
    }

    fn peer_address(&self) -> io::Result<std::net::SocketAddr> {
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
        Ok(addr)
    }

    fn shutdown(&self) -> io::Result<()> {
        Ok(())
    }
}

impl PartialEq for MockTcpStream {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.read_buffer, &other.read_buffer)
            && Arc::ptr_eq(&self.write_buffer, &other.write_buffer)
    }
}

impl MockTcpStream {
    /// Creates new MockTcpStream.
    pub fn new() -> Self {
        Self {
            read_buffer: Arc::new(Mutex::new(vec![])),
            write_buffer: Arc::new(Mutex::new(vec![])),
        }
    }
    /// Clears buffers.
    pub fn clear(&mut self) {
        self.read_lock().clear();
        self.write_lock().clear();
    }

    fn read_lock(&self) -> MutexGuard<Vec<u8>> {
        self.read_buffer.lock().unwrap()
    }

    fn write_lock(&self) -> MutexGuard<Vec<u8>> {
        self.write_buffer.lock().unwrap()
    }

    /// Obtains what is in write buffer.
    pub fn read_wbuf(&self) -> Vec<u8> {
        self.write_lock().clone()
    }

    /// Obtains what is in write buffer and returns it as string.
    pub fn read_wbuf_to_string(&self) -> String {
        String::from_utf8(self.read_wbuf()).unwrap()
    }

    /// Obtains what is in write buffer and returns it as a vector of strings.
    pub fn get_responses(&self) -> Vec<String> {
        let mut responses: Vec<String> = self
            .read_wbuf_to_string()
            .split("\r\n")
            .map(|string| string.to_string())
            .collect();

        responses.pop();
        responses
    }

    // pub fn write_rbuf(&self, buf: &[u8]) -> std::io::Result<usize> {
    //     self.read_lock().write(buf)
    // }
}
