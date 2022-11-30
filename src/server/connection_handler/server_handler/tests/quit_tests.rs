use crate::server::{
    connection_handler::{
        connection_handler_trait::ConnectionHandlerCommands,
        server_handler::tests::dummy_server_handler,
    },
    testing::{dummy_client, dummy_external_client, dummy_server},
};

// #[test]
// fn quit_sets_client_offline() {
//     let mut handler = dummy_server_handler();
//     handler
//         .database
//         .add_external_client(dummy_external_client("nickname1", "servername1"));

//     let prefix = Some("nickname1".to_string());
//     handler.quit_command((prefix, vec![], None)).unwrap();

//     assert!(handler.database.is_disconnected("nickname1"));
// }

#[test]
fn quit_with_invalid_arguments_is_ignored() {
    let mut handler = dummy_server_handler();
    handler
        .database
        .add_external_client(dummy_external_client("nickname1", "servername1"));
    handler
        .database
        .add_immediate_server(dummy_server("servername2"));

    let prefix = Some("nickname1".to_string());
    let trail = Some("going to lunch!".to_string());
    handler.quit_command((None, vec![], trail)).unwrap();
    handler.quit_command((prefix, vec![], None)).unwrap();

    assert_eq!(
        "",
        handler
            .database
            .get_server_stream("servername2")
            .unwrap()
            .read_wbuf_to_string(),
    );
}

#[test]
fn quit_with_unknown_client_is_ignored() {
    let mut handler = dummy_server_handler();

    handler
        .database
        .add_immediate_server(dummy_server("servername2"));

    let prefix = Some("nickname1".to_string());
    let trail = Some("going to lunch!".to_string());
    handler.quit_command((prefix, vec![], trail)).unwrap();

    assert_eq!(
        "",
        handler
            .database
            .get_server_stream("servername2")
            .unwrap()
            .read_wbuf_to_string(),
    );
}

#[test]
fn quit_is_sent_to_local_clients_on_shared_channels() {
    let mut handler = dummy_server_handler();
    handler
        .database
        .add_external_client(dummy_external_client("nickname1", "servername1"));
    handler.database.add_local_client(dummy_client("nickname2"));
    handler.database.add_local_client(dummy_client("nickname3"));

    handler
        .database
        .add_client_to_channel("nickname1", "#channel");
    handler
        .database
        .add_client_to_channel("nickname2", "#channel");
    let prefix = Some("nickname1".to_string());
    let trail = Some("nickname1".to_string());
    handler.quit_command((prefix, vec![], trail)).unwrap();

    assert_eq!(
        ":nickname1 QUIT :nickname1\r\n",
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
fn quit_is_relayed_to_all_other_servers() {
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
    let trail = Some("going to lunch!".to_string());
    handler.quit_command((prefix, vec![], trail)).unwrap();

    assert_eq!(
        ":nickname1 QUIT :going to lunch!\r\n",
        handler
            .database
            .get_server_stream("servername2")
            .unwrap()
            .read_wbuf_to_string(),
    );

    assert_eq!(
        ":nickname1 QUIT :going to lunch!\r\n",
        handler
            .database
            .get_server_stream("servername3")
            .unwrap()
            .read_wbuf_to_string(),
    );
}

#[test]
fn quit_is_never_relayed_to_sending_server() {
    let mut handler = dummy_server_handler();
    handler
        .database
        .add_external_client(dummy_external_client("nickname1", "servername1"));

    let prefix = Some("nickname1".to_string());
    let trail = Some("disconnecting".to_string());
    handler.quit_command((prefix, vec![], trail)).unwrap();

    assert_eq!("", handler.stream.read_wbuf_to_string());
}
