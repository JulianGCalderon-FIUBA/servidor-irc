use crate::server::{
    connection_handler::{
        connection_handler_trait::ConnectionHandlerCommands,
        server_handler::tests::dummy_server_handler,
    },
    testing::{dummy_client, dummy_external_client, dummy_server},
};

#[test]
fn topic_sets_channel_topic() {
    let mut handler = dummy_server_handler();
    handler
        .database
        .add_external_client(dummy_external_client("nickname1", "servername1"));
    handler
        .database
        .add_client_to_channel("nickname1", "#channel");

    let prefix = Some("nickname1".to_string());
    let params = vec!["#channel".to_string(), "new_topic".to_string()];
    handler.topic_command((prefix, params, None)).unwrap();

    assert_eq!(
        Some("new_topic".to_string()),
        handler.database.get_topic_for_channel("#channel")
    )
}

#[test]
fn topic_is_sent_to_local_clients() {
    let mut handler = dummy_server_handler();
    handler
        .database
        .add_external_client(dummy_external_client("nickname1", "servername1"));
    handler.database.add_local_client(dummy_client("nickname2"));
    handler.database.add_local_client(dummy_client("nickname3"));

    handler
        .database
        .add_client_to_channel("nickname2", "#channel");

    let prefix = Some("nickname1".to_string());
    let parameters = vec!["#channel".to_string(), "new_topic".to_string()];
    handler.topic_command((prefix, parameters, None)).unwrap();

    assert_eq!(
        ":nickname1 TOPIC #channel new_topic\r\n",
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

#[test]
fn topic_is_relayed_to_all_other_servers() {
    let mut handler = dummy_server_handler();
    handler
        .database
        .add_external_client(dummy_external_client("nickname1", "servername1"));
    handler
        .database
        .add_client_to_channel("nickname1", "#channel");
    handler
        .database
        .add_immediate_server(dummy_server("servername2"));
    handler
        .database
        .add_immediate_server(dummy_server("servername3"));

    let prefix = Some("nickname1".to_string());
    let params = vec!["#channel".to_string(), "new_topic".to_string()];
    handler.topic_command((prefix, params, None)).unwrap();

    assert_eq!(
        ":nickname1 TOPIC #channel new_topic\r\n",
        handler
            .database
            .get_server_stream("servername2")
            .unwrap()
            .read_wbuf_to_string(),
    );

    assert_eq!(
        ":nickname1 TOPIC #channel new_topic\r\n",
        handler
            .database
            .get_server_stream("servername3")
            .unwrap()
            .read_wbuf_to_string(),
    );
}

#[test]
fn topic_is_never_relayed_to_sending_server() {
    let mut handler = dummy_server_handler();
    handler
        .database
        .add_external_client(dummy_external_client("nickname1", "servername1"));
    handler
        .database
        .add_client_to_channel("nickname1", "#channel");

    let prefix = Some("nickname1".to_string());
    let params = vec!["#channel".to_string(), "new_topic".to_string()];
    handler.topic_command((prefix, params, None)).unwrap();

    assert_eq!("", handler.stream.read_wbuf_to_string());
}
