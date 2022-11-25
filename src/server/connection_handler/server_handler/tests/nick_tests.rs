use crate::server::{
    connection_handler::{
        connection_handler_trait::ConnectionHandlerCommands,
        server_handler::tests::dummy_server_handler,
    },
    testing::{dummy_client, dummy_external_client, dummy_server},
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
fn nick_with_prefix_updates_nick() {
    let mut handler = dummy_server_handler();
    handler
        .database
        .add_external_client(dummy_external_client("nickname1", "servername1"));

    let parameters = vec!["nickname2".to_string()];
    let prefix = Some("nickname1".to_string());
    handler.nick_command((prefix, parameters, None)).unwrap();

    handler.database.contains_client("nickname2");
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

#[test]
fn nick_is_relayed_to_all_other_servers() {
    let mut handler = dummy_server_handler();
    handler
        .database
        .add_immediate_server(dummy_server("servername2"));
    handler
        .database
        .add_immediate_server(dummy_server("servername3"));

    let parameters = vec!["nickname".to_string(), "1".to_string()];
    handler.nick_command((None, parameters, None)).unwrap();

    assert_eq!(
        "NICK nickname 2\r\n",
        handler
            .database
            .get_server_stream("servername2")
            .unwrap()
            .unwrap()
            .read_wbuf_to_string()
    );
    assert_eq!(
        "NICK nickname 2\r\n",
        handler
            .database
            .get_server_stream("servername3")
            .unwrap()
            .unwrap()
            .read_wbuf_to_string()
    );
}

#[test]
fn nick_is_not_relayed_to_sending_server() {
    let mut handler = dummy_server_handler();

    let parameters = vec!["nickname".to_string(), "1".to_string()];
    handler.nick_command((None, parameters, None)).unwrap();

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

#[test]
fn nick_update_is_relayed_to_all_other_servers() {
    let mut handler = dummy_server_handler();
    handler
        .database
        .add_external_client(dummy_external_client("nickname1", "servername1"));
    handler
        .database
        .add_immediate_server(dummy_server("servername2"));
    handler
        .database
        .add_immediate_server(dummy_server("servername3"));

    let prefix = Some("nickname1".to_string());
    let parameters = vec!["nickname2".to_string()];
    handler.nick_command((prefix, parameters, None)).unwrap();

    assert_eq!(
        ":nickname1 NICK nickname2\r\n",
        handler
            .database
            .get_server_stream("servername2")
            .unwrap()
            .unwrap()
            .read_wbuf_to_string()
    );
    assert_eq!(
        ":nickname1 NICK nickname2\r\n",
        handler
            .database
            .get_server_stream("servername3")
            .unwrap()
            .unwrap()
            .read_wbuf_to_string()
    );
}
