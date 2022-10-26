use std::sync::{Arc, Mutex};
mod join;
mod part;
mod privmsg;
mod registration;

use crate::server::{
    database::{ClientInfo, ClientInfoBuilder, Database},
    mock_stream::MockTcpStream,
};

use super::*;

fn dummy_client(nickname: &str) -> ClientInfo<MockTcpStream> {
    let mut builder = ClientInfoBuilder::new_with(
        nickname.to_string(),
        "username".to_string(),
        "hostname".to_string(),
        "servername".to_string(),
        "real name".to_string(),
    );

    builder.with_stream(Arc::new(Mutex::new(MockTcpStream::new())));

    builder.build()
}

fn dummy_client_handler() -> ClientHandler<MockTcpStream> {
    let database = Database::new();
    let handler_stream = MockTcpStream::new();
    let database_stream = handler_stream.clone();

    ClientHandler::new(Arc::new(database), handler_stream, database_stream).unwrap()
}

fn register_client(handler: &mut ClientHandler<MockTcpStream>, nick: &str) {
    let parameters = vec![nick.to_string()];
    handler.nick_command(parameters).unwrap();

    let parameters = vec!["user".to_string(), "".to_string(), "".to_string()];
    let trailing = Some("sol".to_string());
    handler.user_command(parameters, trailing).unwrap();

    handler.stream_client_handler.clear()
}
