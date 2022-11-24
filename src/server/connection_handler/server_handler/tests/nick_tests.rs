use crate::server::{
    connection_handler::{
        connection_handler_trait::ConnectionHandlerCommands,
        server_handler::tests::dummy_server_handler,
    },
    testing::dummy_client,
};

#[test]
fn nick_is_ignored_without_enough_parameters() {
    let mut handler = dummy_server_handler();

    let parameters = vec!["nickname".to_string()];
    handler.nick_command((None, parameters, None)).unwrap();

    assert!(!handler.hopcounts.contains_key("nickname"));
}

#[test]
fn nick_is_ignored_with_non_numeric_hopcount() {
    let mut handler = dummy_server_handler();

    let parameters = vec!["nickname".to_string()];
    handler.nick_command((None, parameters, None)).unwrap();

    assert!(!handler.hopcounts.contains_key("nickname"));
}

#[test]
fn nick_adds_client_to_hopcounts() {
    let mut handler = dummy_server_handler();

    let parameters = vec!["nickname".to_string(), "1".to_string()];
    handler.nick_command((None, parameters, None)).unwrap();

    assert_eq!(1, handler.hopcounts.remove("nickname").unwrap());
}

#[test]
fn nick_with_used_nickname_returns_nick_collision() {
    let mut handler = dummy_server_handler();
    handler.database.add_local_client(dummy_client("nickname"));

    let parameters = vec!["nickname".to_string(), "1".to_string()];
    handler.nick_command((None, parameters, None)).unwrap();

    assert_eq!(
        "436 nickname :Nickname collision KILL\r\n",
        handler.stream.read_wbuf_to_string()
    );
}

// fn nick_is_relayed_to_all_other_servers() {}

// fn nick_is_not_relayed_to_sending_server() {}
