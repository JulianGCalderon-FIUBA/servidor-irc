use crate::server::{
    connection_handler::ConnectionHandlerCommands,
    testing::{dummy_client, dummy_external_client, dummy_server},
};

use super::dummy_client_handler;

#[test]
fn quit_without_message_returns_nickname() {
    let mut handler = dummy_client_handler();

    handler.quit_command((None, vec![], None)).unwrap();

    assert_eq!("QUIT :nickname\r\n", handler.stream.read_wbuf_to_string())
}

#[test]
fn quit_with_message_returns_message() {
    let mut handler = dummy_client_handler();

    let trail = Some("message".to_string());
    handler.quit_command((None, vec![], trail)).unwrap();

    assert_eq!("QUIT :message\r\n", handler.stream.read_wbuf_to_string())
}

#[test]
fn quit_notifies_all_users_in_clients_channels() {
    let mut handler = dummy_client_handler();

    handler.database.add_local_client(dummy_client("nickname1"));
    handler
        .database
        .add_client_to_channel("#channel", "nickname1");
    handler
        .database
        .add_client_to_channel("#channel", "nickname");

    let trail = Some("message".to_string());
    handler.quit_command((None, vec![], trail)).unwrap();

    assert_eq!("QUIT :message\r\n", handler.stream.read_wbuf_to_string());
    assert_eq!(
        ":nickname QUIT :message\r\n",
        handler
            .database
            .get_local_stream("nickname1")
            .unwrap()
            .read_wbuf_to_string()
    )
}

#[test]
fn quit_is_sent_to_all_servers() {
    let mut handler = dummy_client_handler();

    handler
        .database
        .add_immediate_server(dummy_server("servername1"));
    handler
        .database
        .add_immediate_server(dummy_server("servername2"));

    let trail = Some("message".to_string());
    handler.quit_command((None, vec![], trail)).unwrap();

    assert_eq!(
        ":nickname QUIT :message\r\n",
        handler
            .database
            .get_server_stream("servername1")
            .unwrap()
            .read_wbuf_to_string()
    );
    assert_eq!(
        ":nickname QUIT :message\r\n",
        handler
            .database
            .get_server_stream("servername2")
            .unwrap()
            .read_wbuf_to_string()
    );
}

#[test]
fn quit_is_sent_to_servers_only_once() {
    let mut handler = dummy_client_handler();

    handler
        .database
        .add_immediate_server(dummy_server("servername1"));

    handler
        .database
        .add_external_client(dummy_external_client("nickname1", "servername1"));

    let trail = Some("message".to_string());
    handler.quit_command((None, vec![], trail)).unwrap();

    assert_eq!(
        ":nickname QUIT :message\r\n",
        handler
            .database
            .get_server_stream("servername1")
            .unwrap()
            .read_wbuf_to_string()
    );
}
