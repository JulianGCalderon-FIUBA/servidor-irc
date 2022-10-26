use std::io::{Read, Write};
use std::sync::{Arc, Mutex, MutexGuard};

pub struct MockTcpStream {
    pub read_buffer: Arc<Mutex<Vec<u8>>>,
    pub write_buffer: Arc<Mutex<Vec<u8>>>,
}

impl Read for MockTcpStream {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.read_lock().as_slice().read(buf)
    }
}

impl Write for MockTcpStream {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.write_lock().write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.write_lock().flush()
    }
}

impl Clone for MockTcpStream {
    fn clone(&self) -> Self {
        Self {
            read_buffer: Arc::clone(&self.read_buffer),
            write_buffer: Arc::clone(&self.write_buffer),
        }
    }
}

impl MockTcpStream {
    pub fn new() -> Self {
        Self {
            read_buffer: Arc::new(Mutex::new(vec![])),
            write_buffer: Arc::new(Mutex::new(vec![])),
        }
    }

    pub fn clear(&mut self) {
        self.read_lock().drain(..);
        self.write_lock().drain(..);
    }

    fn read_lock(&self) -> MutexGuard<Vec<u8>> {
        self.read_buffer.lock().unwrap()
    }

    fn write_lock(&self) -> MutexGuard<Vec<u8>> {
        self.write_buffer.lock().unwrap()
    }

    pub fn read_wbuf(&self) -> Vec<u8> {
        self.write_lock().clone()
    }

    pub fn read_wbuf_to_string(&self) -> String {
        String::from_utf8(self.read_wbuf()).unwrap()
    }

    // pub fn write_rbuf(&self, buf: &[u8]) -> std::io::Result<usize> {
    //     self.read_lock().write(buf)
    // }
}
