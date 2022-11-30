use crate::server::{
    connection_handler::{
        connection_handler_trait::ConnectionHandlerCommands,
        server_handler::tests::dummy_server_handler,
    },
    testing::{dummy_client, dummy_external_client, dummy_server},
};

#[test]
fn kick_with_invalid_arguments_is_ignored() {
    let mut handler = dummy_server_handler();

    handler
        .database
        .add_immediate_server(dummy_server("servername2"));

    handler.database.add_local_client(dummy_client("kicked"));
    handler
        .database
        .add_external_client(dummy_external_client("kicked", "servername"));

    handler.database.add_client_to_channel("kicked", "#channel");

    let params = vec!["#channel".to_string(), "kicked".to_string()];
    handler.kick_command((None, params, None)).unwrap();
    let prefix = Some("kicker".to_string());
    let params = vec!["#channel".to_string()];
    handler
        .kick_command((prefix.clone(), params, None))
        .unwrap();
    handler.kick_command((prefix, vec![], None)).unwrap();

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
fn kick_with_no_client_in_database_is_ignored() {
    let mut handler = dummy_server_handler();

    handler
        .database
        .add_immediate_server(dummy_server("servername2"));

    handler.database.add_client_to_channel("kicked", "#channel");

    let prefix = Some("kicker".to_string());
    let params = vec!["#channel".to_string(), "kicked".to_string()];
    handler.kick_command((prefix, params, None)).unwrap();

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
fn kick_with_no_client_in_channel_is_ignored() {
    let mut handler = dummy_server_handler();

    handler
        .database
        .add_immediate_server(dummy_server("servername2"));

    handler
        .database
        .add_client_to_channel("nickname", "#channel");

    handler.database.add_local_client(dummy_client("kicked"));

    let prefix = Some("kicker".to_string());
    let params = vec!["#channel".to_string(), "kicked".to_string()];
    handler.kick_command((prefix, params, None)).unwrap();

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
fn kick_with_no_channel_in_database_is_ignored() {
    let mut handler = dummy_server_handler();

    handler
        .database
        .add_immediate_server(dummy_server("servername2"));

    handler.database.add_local_client(dummy_client("kicked"));

    let prefix = Some("kicker".to_string());
    let params = vec!["#channel".to_string(), "kicked".to_string()];
    handler.kick_command((prefix, params, None)).unwrap();

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
fn kick_removes_client_from_channel() {
    let mut handler = dummy_server_handler();
    handler.database.add_local_client(dummy_client("kicked"));
    handler.database.add_client_to_channel("kicked", "#channel");

    let prefix = Some("kicker".to_string());
    let params = vec!["#channel".to_string(), "kicked".to_string()];
    handler.kick_command((prefix, params, None)).unwrap();

    assert!(!handler.database.is_client_in_channel("kicked", "#channel"));
}

#[test]
fn kick_is_relayed_to_all_other_servers() {
    let mut handler = dummy_server_handler();
    handler.database.add_local_client(dummy_client("kicked"));

    handler
        .database
        .add_immediate_server(dummy_server("servername2"));
    handler
        .database
        .add_immediate_server(dummy_server("servername3"));

    handler.database.add_client_to_channel("kicked", "#channel");

    let prefix = Some("kicker".to_string());
    let params = vec!["#channel".to_string(), "kicked".to_string()];
    handler.kick_command((prefix, params, None)).unwrap();

    assert_eq!(
        ":kicker KICK #channel kicked\r\n",
        handler
            .database
            .get_server_stream("servername2")
            .unwrap()
            .read_wbuf_to_string()
    );
    assert_eq!(
        ":kicker KICK #channel kicked\r\n",
        handler
            .database
            .get_server_stream("servername3")
            .unwrap()
            .read_wbuf_to_string()
    );
}

#[test]
fn kick_is_never_relayed_to_sending_server() {
    let mut handler = dummy_server_handler();
    handler
        .database
        .add_external_client(dummy_external_client("nickname1", "servername1"));
    handler
        .database
        .add_external_client(dummy_external_client("nickname2", "servername1"));
    handler
        .database
        .add_client_to_channel("nickname1", "#channel");
    handler
        .database
        .add_client_to_channel("nickname2", "#channel");

    let prefix = Some("nickname1".to_string());
    let parameters = vec!["#channel".to_string(), "nickname2".to_string()];
    handler.kick_command((prefix, parameters, None)).unwrap();

    assert_eq!("", handler.stream.read_wbuf_to_string());
}

#[test]
fn kick_is_relayed_to_local_clients_on_channel() {
    let mut handler = dummy_server_handler();
    handler
        .database
        .add_external_client(dummy_external_client("nickname1", "servername1"));
    handler.database.add_local_client(dummy_client("nickname2"));
    handler.database.add_local_client(dummy_client("nickname3"));
    handler.database.add_local_client(dummy_client("nickname4"));

    handler
        .database
        .add_client_to_channel("nickname1", "#channel");
    handler
        .database
        .add_client_to_channel("nickname2", "#channel");
    handler
        .database
        .add_client_to_channel("nickname3", "#channel");

    let prefix = Some("nickname1".to_string());
    let parameters = vec!["#channel".to_string(), "nickname3".to_string()];
    handler.kick_command((prefix, parameters, None)).unwrap();

    assert_eq!(
        ":nickname1 KICK #channel nickname3\r\n",
        handler
            .database
            .get_local_stream("nickname2")
            .unwrap()
            .read_wbuf_to_string()
    );
    assert_eq!(
        ":nickname1 KICK #channel nickname3\r\n",
        handler
            .database
            .get_local_stream("nickname3")
            .unwrap()
            .read_wbuf_to_string()
    );
    assert_eq!(
        "",
        handler
            .database
            .get_local_stream("nickname4")
            .unwrap()
            .read_wbuf_to_string()
    );
}
