use crate::server::{
    connection_handler::connection_handler_trait::ConnectionHandlerCommands,
    testing::{dummy_client, dummy_external_client, dummy_server},
};

use super::dummy_server_handler;

#[test]
fn privmsg_is_ignored_without_enough_parameters() {
    let mut handler = dummy_server_handler();

    assert!(handler.privmsg_command((None, vec![], None)).is_ok());
}

#[test]
fn privmsg_is_ignored_with_unknown_target() {
    let mut handler = dummy_server_handler();

    let prefix = Some("sender".to_string());
    let params = vec!["target".to_string()];
    let trail = Some("message".to_string());
    assert!(handler.privmsg_command((prefix, params, trail)).is_ok());
}

#[test]
fn privmsg_is_relayed_to_client() {
    let mut handler = dummy_server_handler();
    handler.database.add_local_client(dummy_client("target"));
    handler
        .database
        .add_external_client(dummy_external_client("sender", "servername"));

    let prefix = Some("sender".to_string());
    let params = vec!["target".to_string()];
    let trail = Some("message".to_string());
    handler.privmsg_command((prefix, params, trail)).unwrap();

    assert_eq!(
        ":sender PRIVMSG target :message\r\n",
        handler
            .database
            .get_local_stream("target")
            .unwrap()
            .unwrap()
            .read_wbuf_to_string()
    )
}

#[test]
fn privmsg_to_client_is_relayed_to_necesary_server() {
    let mut handler = dummy_server_handler();
    handler
        .database
        .add_immediate_server(dummy_server("servername2"));
    handler
        .database
        .add_external_client(dummy_external_client("target", "servername2"));
    handler
        .database
        .add_external_client(dummy_external_client("sender", "servername"));

    let prefix = Some("sender".to_string());
    let params = vec!["target".to_string()];
    let trail = Some("message".to_string());
    handler.privmsg_command((prefix, params, trail)).unwrap();

    assert_eq!(
        ":sender PRIVMSG target :message\r\n",
        handler
            .database
            .get_server_stream("servername2")
            .unwrap()
            .unwrap()
            .read_wbuf_to_string()
    )
}

#[test]
fn privmsg_to_channel_is_relayed_to_all_local_clients_in_channel() {
    let mut handler = dummy_server_handler();
    handler.database.add_local_client(dummy_client("nickname1"));
    handler.database.add_local_client(dummy_client("nickname2"));

    handler
        .database
        .add_client_to_channel("nickname1", "#channel");
    handler
        .database
        .add_client_to_channel("nickname2", "#channel");
    handler
        .database
        .add_external_client(dummy_external_client("sender", "servername"));
    handler.database.add_client_to_channel("sender", "#channel");

    let prefix = Some("sender".to_string());
    let params = vec!["#channel".to_string()];
    let trail = Some("message".to_string());
    handler.privmsg_command((prefix, params, trail)).unwrap();

    assert_eq!(
        ":sender PRIVMSG #channel :message\r\n",
        handler
            .database
            .get_local_stream("nickname1")
            .unwrap()
            .unwrap()
            .read_wbuf_to_string()
    );

    assert_eq!(
        ":sender PRIVMSG #channel :message\r\n",
        handler
            .database
            .get_local_stream("nickname1")
            .unwrap()
            .unwrap()
            .read_wbuf_to_string()
    );
}

#[test]
fn privmsg_to_channel_is_relayed_to_each_necesary_server_once() {
    let mut handler = dummy_server_handler();
    handler
        .database
        .add_external_client(dummy_external_client("nickname1", "servername2"));
    handler
        .database
        .add_external_client(dummy_external_client("nickname2", "servername2"));
    handler
        .database
        .add_external_client(dummy_external_client("sender", "servername"));
    handler
        .database
        .add_immediate_server(dummy_server("servername2"));
    handler
        .database
        .add_client_to_channel("nickname1", "#channel");
    handler
        .database
        .add_client_to_channel("nickname2", "#channel");
    handler.database.add_client_to_channel("sender", "#channel");

    let prefix = Some("sender".to_string());
    let params = vec!["#channel".to_string()];
    let trail = Some("message".to_string());
    handler.privmsg_command((prefix, params, trail)).unwrap();

    assert_eq!(
        ":sender PRIVMSG #channel :message\r\n",
        handler
            .database
            .get_server_stream("servername2")
            .unwrap()
            .unwrap()
            .read_wbuf_to_string()
    );
}

#[test]
fn privmsg_is_never_relayed_to_sending_server() {
    let mut handler = dummy_server_handler();
    handler
        .database
        .add_external_client(dummy_external_client("sender", "servername1"));
    handler.database.add_client_to_channel("sender", "#channel");

    let prefix = Some("sender".to_string());
    let params = vec!["#channel".to_string()];
    let trail = Some("message".to_string());
    handler.privmsg_command((prefix, params, trail)).unwrap();

    assert_eq!("", handler.stream.read_wbuf_to_string());
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
