use crate::server::connection_handler::{
    connection_handler_trait::ConnectionHandlerCommands,
    server_handler::tests::dummy_server_handler,
};

#[test]
fn user_adds_client_to_database() {
    let mut handler = dummy_server_handler();

    let parameters = vec!["nickname".to_string(), "1".to_string()];
    handler.nick_command((None, parameters, None)).unwrap();

    let prefix = Some("nickname".to_string());
    let parameters = vec![
        "nickname".to_string(),
        "hostname".to_string(),
        "servername".to_string(),
    ];
    let trailing = Some("realname".to_string());
    handler
        .user_command((prefix, parameters, trailing))
        .unwrap();

    assert!(handler.database.contains_client("nickname"));
}
