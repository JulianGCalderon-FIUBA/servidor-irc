use crate::server::{
    connection_handler::connection_handler_trait::ConnectionHandlerCommands,
    testing::{dummy_external_client, dummy_server},
};

use super::dummy_server_handler;

#[test]
fn away_sets_away_message_for_client() {
    let mut handler = dummy_server_handler();
    handler
        .database
        .add_external_client(dummy_external_client("nickname", "servername1"));

    let prefix = Some("nickname".to_string());
    let trail = Some("message".to_string());
    handler.away_command((prefix, vec![], trail)).unwrap();

    assert_eq!(
        "message",
        handler.database.get_away_message("nickname").unwrap()
    );
}

#[test]
fn away_is_relayed_to_all_other_servers() {
    let mut handler = dummy_server_handler();
    handler
        .database
        .add_immediate_server(dummy_server("servername2"));
    handler
        .database
        .add_immediate_server(dummy_server("servername3"));

    handler
        .database
        .add_external_client(dummy_external_client("nickname", "servername1"));

    let prefix = Some("nickname".to_string());
    let trail = Some("message".to_string());
    handler.away_command((prefix, vec![], trail)).unwrap();

    assert_eq!(
        ":nickname AWAY :message\r\n",
        handler
            .database
            .get_server_stream("servername2")
            .unwrap()
            .unwrap()
            .read_wbuf_to_string()
    );

    assert_eq!(
        ":nickname AWAY :message\r\n",
        handler
            .database
            .get_server_stream("servername3")
            .unwrap()
            .unwrap()
            .read_wbuf_to_string()
    );
}
