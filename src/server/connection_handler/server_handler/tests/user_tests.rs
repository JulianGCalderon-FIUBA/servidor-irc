use crate::server::connection_handler::{
    connection_handler_trait::ConnectionHandlerCommands,
    server_handler::tests::dummy_server_handler,
};

#[test]
fn user_adds_client_to_database() {
    let mut handler = dummy_server_handler();
    handler.hopcounts.insert("nickname".to_string(), 1);

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

#[test]
fn user_fails_without_enough_parameters() {
    let mut handler = dummy_server_handler();
    handler.hopcounts.insert("nickname".to_string(), 1);

    let parameters = vec!["username".to_string()];
    handler.user_command((None, parameters, None)).unwrap();

    assert!(!handler.database.contains_client("nickname"));
}

#[test]
fn user_fails_with_no_previuos_nick() {
    let mut handler = dummy_server_handler();

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

    assert!(!handler.database.contains_client("nickname"));
}

// fn user_is_relayed_to_all_other_servers() {}

// fn nick_is_not_relayed_to_sending_server() {}
