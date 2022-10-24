use std::{
    io::{Read, Write},
    sync::Arc,
};

use crate::server::database::Database;

use super::*;

#[derive(Clone)]
struct MockTcpStream {
    read_buffer: Vec<u8>,
    write_buffer: Vec<u8>,
}

impl Read for MockTcpStream {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.read_buffer.as_slice().read(buf)
    }
}

impl Write for MockTcpStream {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.write_buffer.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.write_buffer.flush()
    }
}

fn dummy_client_handler() -> ClientHandler<MockTcpStream> {
    let database = Database::new();
    let mock = MockTcpStream {
        read_buffer: Vec::new(),
        write_buffer: Vec::new(),
    };

    ClientHandler::new(Arc::new(database), mock.clone(), mock).unwrap()
}

#[test]
fn join_with_empty_params_returns_need_more_params_error() {
    let mut handler = dummy_client_handler();
    let params = vec![];
    let channels: Vec<String> = vec![];

    handler.join_command(params).unwrap();

    assert_eq!(
        "461 JOIN :not enough parameters\r\n".to_string(),
        String::from_utf8(handler.stream_client_handler.write_buffer).unwrap()
    );
    assert_eq!(handler.database.get_channels(), channels);
}
