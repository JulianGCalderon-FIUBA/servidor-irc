use crate::server::{
    connection_handler::{
        connection_handler_trait::ConnectionHandlerCommands,
        server_handler::tests::dummy_server_handler,
    },
    testing::{dummy_client, dummy_distant_server, dummy_external_client, dummy_server},
};

#[test]
fn squit_removes_server_from_database() {
    let mut handler = dummy_server_handler();
    handler
        .database
        .add_immediate_server(dummy_server("servername2"));

    let prefix = Some("oper".to_string());
    let parameters = vec!["servername2".to_string()];
    let trail = Some("Closing connection".to_string());

    assert!(handler.database.contains_server("servername2"));
    handler.squit_command((prefix, parameters, trail)).unwrap();
    assert!(!handler.database.contains_server("servername2"));
}

#[test]
fn squit_disconnects_external_client() {
    let mut handler = dummy_server_handler();
    handler
        .database
        .add_immediate_server(dummy_server("servername2"));
    let external = dummy_external_client("nickname", "servername2");
    handler.database.add_external_client(external);

    let prefix = Some("oper".to_string());
    let parameters = vec!["servername2".to_string()];
    let trail = Some("Closing connection".to_string());
    handler.squit_command((prefix, parameters, trail)).unwrap();

    assert!(handler.database.get_local_stream("nickname").is_err());
}

#[test]
fn squit_is_relayed_to_all_other_servers() {
    let mut handler = dummy_server_handler();
    handler
        .database
        .add_immediate_server(dummy_server("servername2"));
    handler
        .database
        .add_immediate_server(dummy_server("servername3"));

    let prefix = Some("oper".to_string());
    let parameters = vec!["servername2".to_string()];
    let trail = Some("Closing connection".to_string());

    let server2_stream = handler.database.get_server_stream("servername2");

    handler.squit_command((prefix, parameters, trail)).unwrap();

    assert_eq!(
        ":oper SQUIT servername2 :Closing connection\r\n",
        server2_stream.unwrap().read_wbuf_to_string()
    );
    assert_eq!(
        ":oper SQUIT servername2 :Closing connection\r\n",
        handler
            .database
            .get_server_stream("servername3")
            .unwrap()
            .read_wbuf_to_string()
    );
}

#[test]
fn squit_is_not_relayed_to_sending_server() {
    let mut handler = dummy_server_handler();

    let prefix = Some("oper".to_string());
    let parameters = vec!["servername2".to_string()];
    let trail = Some("Closing connection".to_string());

    handler.squit_command((prefix, parameters, trail)).unwrap();

    assert_eq!(
        "",
        handler
            .database
            .get_server_stream("servername1")
            .unwrap()
            .read_wbuf_to_string()
    );
}

#[test]
fn squit_relays_client_quit_to_servers() {
    let mut handler = dummy_server_handler();
    handler
        .database
        .add_immediate_server(dummy_server("servername2"));
    handler
        .database
        .add_immediate_server(dummy_server("servername3"));
    let external = dummy_external_client("nickname", "servername2");
    handler.database.add_external_client(external);

    let prefix = Some("oper".to_string());
    let parameters = vec!["servername2".to_string()];
    let trail = Some("Closing connection".to_string());

    let stream2 = handler.database.get_server_stream("servername2");

    handler.squit_command((prefix, parameters, trail)).unwrap();

    let stream3 = handler.database.get_server_stream("servername3");

    let responses2 = stream2.unwrap().get_responses();
    let responses3 = stream3.unwrap().get_responses();

    assert_eq!(":oper SQUIT servername2 :Closing connection", responses2[0]);
    assert_eq!(":oper SQUIT servername2 :Closing connection", responses3[0]);
    assert_eq!(":nickname QUIT :Net split", responses3[1]);
}

#[test]
fn squit_does_not_relay_client_quit_to_sending_server() {
    let mut handler = dummy_server_handler();
    handler
        .database
        .add_immediate_server(dummy_server("servername2"));

    let external = dummy_external_client("nickname", "servername2");
    handler.database.add_external_client(external);

    let prefix = Some("oper".to_string());
    let parameters = vec!["servername2".to_string()];
    let trail = Some("Closing connection".to_string());

    let stream2 = handler.database.get_server_stream("servername2");

    handler.squit_command((prefix, parameters, trail)).unwrap();

    assert_eq!(
        ":oper SQUIT servername2 :Closing connection\r\n",
        stream2.unwrap().read_wbuf_to_string()
    );

    assert_eq!(
        "",
        handler
            .database
            .get_server_stream("servername1")
            .unwrap()
            .read_wbuf_to_string()
    );
}

#[test]
fn squit_does_not_disconnect_distant_servers() {
    let mut handler = dummy_server_handler();

    let distant = dummy_distant_server("servername2");
    handler.database.add_distant_server(distant);

    let prefix = Some("sender".to_string());
    let parameters = vec!["servername2".to_string()];

    assert!(handler.database.contains_server("servername2"));

    handler.squit_command((prefix, parameters, None)).unwrap();

    assert!(handler.database.contains_server("servername2"));
}

#[test]
fn squit_relays_client_quit_to_local_clients_on_channel() {
    let mut handler = dummy_server_handler();
    handler
        .database
        .add_immediate_server(dummy_server("servername2"));

    let client1 = dummy_external_client("nickname1", "servername2");
    let client2 = dummy_client("nickname2");
    let client3 = dummy_client("nickname3");

    handler.database.add_local_client(client2);
    handler.database.add_local_client(client3);
    handler.database.add_external_client(client1);

    handler
        .database
        .add_client_to_channel("nickname1", "#channel");
    handler
        .database
        .add_client_to_channel("nickname2", "#channel");

    let prefix = Some("oper".to_string());
    let parameters = vec!["servername2".to_string()];

    handler.squit_command((prefix, parameters, None)).unwrap();

    assert_eq!(
        ":nickname1 QUIT :Net split\r\n",
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
