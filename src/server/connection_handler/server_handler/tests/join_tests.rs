use crate::server::{
    connection_handler::{
        connection_handler_trait::ConnectionHandlerCommands,
        server_handler::tests::dummy_server_handler,
    },
    testing::{dummy_client, dummy_external_client, dummy_server},
};

#[test]
fn join_with_invalid_arguments_is_ignored() {
    let mut handler = dummy_server_handler();
    handler
        .database
        .add_external_client(dummy_external_client("nickname1", "servername1"));
    handler
        .database
        .add_immediate_server(dummy_server("servername2"));

    let prefix = Some("nickname1".to_string());
    let parameters = vec!["#channel".to_string()];
    handler.join_command((None, parameters, None)).unwrap();
    handler.join_command((prefix, vec![], None)).unwrap();

    assert_eq!(
        "",
        handler
            .database
            .get_server_stream("servername2")
            .unwrap()
            .read_wbuf_to_string()
    );
}

#[test]
fn join_with_no_client_in_database_is_ignored() {
    let mut handler = dummy_server_handler();
    handler
        .database
        .add_immediate_server(dummy_server("servername2"));

    let prefix = Some("nickname1".to_string());
    let parameters = vec!["#channel".to_string()];
    handler.join_command((prefix, parameters, None)).unwrap();

    assert_eq!(
        "",
        handler
            .database
            .get_server_stream("servername2")
            .unwrap()
            .read_wbuf_to_string()
    );
}

#[test]
fn join_adds_client_to_channel() {
    let mut handler = dummy_server_handler();
    handler
        .database
        .add_external_client(dummy_external_client("nickname1", "servername1"));

    let prefix = Some("nickname1".to_string());
    let params = vec!["#channel".to_string()];
    handler.join_command((prefix, params, None)).unwrap();

    assert!(handler
        .database
        .is_client_in_channel("#channel", "nickname1"));
}

#[test]
fn join_is_relayed_to_all_other_servers() {
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
    let parameters = vec!["#channel".to_string()];
    handler.join_command((prefix, parameters, None)).unwrap();

    assert_eq!(
        ":nickname1 JOIN #channel\r\n",
        handler
            .database
            .get_server_stream("servername2")
            .unwrap()
            .read_wbuf_to_string()
    );
    assert_eq!(
        ":nickname1 JOIN #channel\r\n",
        handler
            .database
            .get_server_stream("servername3")
            .unwrap()
            .read_wbuf_to_string()
    );
}

#[test]
fn join_is_never_relayed_to_sending_server() {
    let mut handler = dummy_server_handler();
    handler
        .database
        .add_external_client(dummy_external_client("nickname1", "servername1"));

    let prefix = Some("nickname1".to_string());
    let parameters = vec!["#channel".to_string()];
    handler.join_command((prefix, parameters, None)).unwrap();

    assert_eq!("", handler.stream.read_wbuf_to_string());
}

#[test]
fn join_is_relayed_to_local_clients_on_channel() {
    let mut handler = dummy_server_handler();
    handler
        .database
        .add_external_client(dummy_external_client("nickname1", "servername1"));
    handler.database.add_local_client(dummy_client("nickname2"));
    handler.database.add_local_client(dummy_client("nickname3"));

    handler
        .database
        .add_client_to_channel("#channel", "nickname2");

    let prefix = Some("nickname1".to_string());
    let parameters = vec!["#channel".to_string()];
    handler.join_command((prefix, parameters, None)).unwrap();

    assert_eq!(
        ":nickname1 JOIN #channel\r\n",
        handler
            .database
            .get_local_stream("nickname2")
            .unwrap()
            .read_wbuf_to_string()
    );
    assert_eq!(
        "",
        handler
            .database
            .get_local_stream("nickname3")
            .unwrap()
            .read_wbuf_to_string()
    );
}
