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

impl MockTcpStream {
    fn clear(&mut self) {
        let empty_buffer: Vec<u8> = vec![];
        self.write_buffer = empty_buffer;
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

fn dummy_client(handler: &mut ClientHandler<MockTcpStream>) {
    let parameters = vec!["nick".to_string()];
    handler.nick_command(parameters).unwrap();
    let parameters2 = vec!["user".to_string(), "".to_string(), "".to_string()];
    let trailing = Some("sol".to_string());
    handler.user_command(parameters2, trailing).unwrap();
}

#[test]
fn join_fails_if_client_not_registered() {
    let mut handler = dummy_client_handler();
    let parameters = vec!["sol".to_string()];

    handler.join_command(parameters).unwrap();

    assert_eq!(
        "300 :unregistered\r\n".to_string(),
        String::from_utf8(handler.stream_client_handler.write_buffer).unwrap()
    )
}
#[test]
fn join_with_empty_params_returns_need_more_params_error() {
    let mut handler = dummy_client_handler();
    let parameters = vec![];
    let channels: Vec<String> = vec![];

    handler.join_command(parameters).unwrap();

    assert_eq!(
        "461 JOIN :not enough parameters\r\n".to_string(),
        String::from_utf8(handler.stream_client_handler.write_buffer).unwrap()
    );
    assert_eq!(handler.database.get_channels(), channels);
}
#[test]
fn join_fails_with_invalid_channel_name() {
    let mut handler = dummy_client_handler();
    dummy_client(&mut handler);

    handler.stream_client_handler.clear();

    let parameters = vec!["hola,#ho'la".to_string()];

    handler.join_command(parameters).unwrap();

    assert_eq!(
        "403 hola :no such channel\r\n403 #ho'la :no such channel\r\n".to_string(),
        String::from_utf8(handler.stream_client_handler.write_buffer).unwrap()
    );
}
