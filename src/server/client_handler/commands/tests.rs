use std::{net::TcpStream, sync::Arc};

use crate::{server::database::Database, ADDRESS};

use super::*;

fn dummy_client_handler() -> ClientHandler {
    let database = Database::new();
    //habría que hacer un mock TcpStream para poder testear también las respuestas.
    let stream = TcpStream::connect(ADDRESS).unwrap();

    ClientHandler::new(Arc::new(database), stream).unwrap()
}

#[test]
fn join_with_empty_params_returns_need_more_params_error() {
    let mut handler = dummy_client_handler();
    let params = vec![];
    let channels: Vec<String> = vec![];

    //aca deberíamos testear una vez que se tenga el mock stream que se recibe la respuesta correcta
    // handler.stream.read(buffer)
    //assert!(buffer, "461 JOIN :not enough parameters")
    assert!(handler.join_command(params).is_ok());
    assert_eq!(handler.database.get_channels(), channels);
}
