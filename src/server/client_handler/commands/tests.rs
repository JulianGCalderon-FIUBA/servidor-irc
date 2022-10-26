use std::sync::Arc;

use crate::server::{database::Database, mock_stream::MockTcpStream};

use super::*;

fn dummy_client_handler() -> ClientHandler<MockTcpStream> {
    let database = Database::new();
    let handler_stream = MockTcpStream::new();
    let database_stream = handler_stream.clone();

    ClientHandler::new(Arc::new(database), handler_stream, database_stream).unwrap()
}

fn register_client(handler: &mut ClientHandler<MockTcpStream>) {
    let parameters = vec!["nick".to_string()];
    handler.nick_command(parameters).unwrap();

    let parameters = vec!["user".to_string(), "".to_string(), "".to_string()];
    let trailing = Some("sol".to_string());
    handler.user_command(parameters, trailing).unwrap();
}

#[test]
fn join_fails_with_unregistered_client() {
    let mut handler = dummy_client_handler();

    let parameters = vec!["sol".to_string()];
    handler.join_command(parameters).unwrap();

    assert_eq!(
        "200 :unregistered\r\n",
        handler.stream_client_handler.read_wbuf_to_string()
    )
}
#[test]
fn join_with_empty_params() {
    let mut handler = dummy_client_handler();
    let parameters = vec![];
    let channels: Vec<String> = vec![];

    handler.join_command(parameters).unwrap();

    assert_eq!(
        "461 JOIN :not enough parameters\r\n",
        handler.stream_client_handler.read_wbuf_to_string()
    );
    assert_eq!(handler.database.get_channels(), channels);
}
#[test]
fn join_fails_with_invalid_channel_name() {
    let mut handler = dummy_client_handler();
    register_client(&mut handler);

    handler.stream_client_handler.clear();

    let parameters = vec!["hola,#ho'la".to_string()];

    handler.join_command(parameters).unwrap();

    assert_eq!(
        "403 hola :no such channel\r\n403 #ho'la :no such channel\r\n",
        handler.stream_client_handler.read_wbuf_to_string()
    );
}
#[test]
fn join_fails_with_user_already_in_channel() {
    let mut handler = dummy_client_handler();
    register_client(&mut handler);

    let parameters =
        vec!["#uno,#dos,#tres,&cuatro,&cinco,&seis,#siete,#ocho,#nueve,&diez".to_string()];
    handler.join_command(parameters).unwrap();

    handler.stream_client_handler.clear();

    let parameters2 = vec!["#once".to_string()];
    handler.join_command(parameters2).unwrap();

    assert_eq!(
        "405 #once :you have joined too many channels\r\n",
        handler.stream_client_handler.read_wbuf_to_string()
    )
}
