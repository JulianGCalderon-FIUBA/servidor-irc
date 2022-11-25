use crate::server::{
    connection_handler::{
        connection_handler_trait::ConnectionHandlerCommands,
        server_handler::tests::dummy_server_handler,
    },
    testing::dummy_server,
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
fn user_is_ignored_without_enough_parameters() {
    let mut handler = dummy_server_handler();
    handler.hopcounts.insert("nickname".to_string(), 1);

    let parameters = vec!["username".to_string()];
    handler.user_command((None, parameters, None)).unwrap();

    assert!(!handler.database.contains_client("nickname"));
}

#[test]
fn user_is_ignored_with_no_previuos_nick() {
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

#[test]
fn user_is_relayed_to_all_other_servers() {
    let mut handler = dummy_server_handler();
    handler
        .database
        .add_immediate_server(dummy_server("servername2"));
    handler
        .database
        .add_immediate_server(dummy_server("servername3"));

    let parameters = vec![
        "username".to_string(),
        "hostname".to_string(),
        "servername".to_string(),
    ];
    let prefix = Some("nickname".to_string());
    let trail = Some("realname".to_string());
    handler.user_command((prefix, parameters, trail)).unwrap();

    assert_eq!(
        ":nickname USER username hostname servername :realname\r\n",
        handler
            .database
            .get_server_stream("servername2")
            .unwrap()
            .unwrap()
            .read_wbuf_to_string()
    );
    assert_eq!(
        ":nickname USER username hostname servername :realname\r\n",
        handler
            .database
            .get_server_stream("servername3")
            .unwrap()
            .unwrap()
            .read_wbuf_to_string()
    );
}

#[test]
fn user_is_not_relayed_to_sending_server() {
    let mut handler = dummy_server_handler();

    let parameters = vec![
        "username".to_string(),
        "hostname".to_string(),
        "servername".to_string(),
    ];
    let prefix = Some("nickname".to_string());
    let trail = Some("realname".to_string());
    handler.user_command((prefix, parameters, trail)).unwrap();

    assert_eq!(
        "",
        handler
            .database
            .get_server_stream("servername1")
            .unwrap()
            .unwrap()
            .read_wbuf_to_string()
    );
}
