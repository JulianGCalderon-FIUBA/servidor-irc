use crate::server::connection_handler::{
    connection_handler_trait::ConnectionHandlerCommands,
    server_handler::tests::dummy_server_handler,
};

#[test]
fn nick_adds_client_to_hopcounts() {
    let mut handler = dummy_server_handler();

    let parameters = vec!["nickname".to_string(), "1".to_string()];
    handler.nick_command((None, parameters, None)).unwrap();

    assert_eq!(1, handler.hopcounts.remove("nickname").unwrap());
}
