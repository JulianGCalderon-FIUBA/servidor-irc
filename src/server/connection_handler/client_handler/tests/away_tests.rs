use crate::server::testing::dummy_server;

use super::*;

#[test]
fn can_set_away_message_for_client() {
    let mut handler = dummy_client_handler();

    let trailing = Some("away message!".to_string());
    handler.away_command((None, vec![], trailing)).unwrap();

    assert_eq!(
        "306 :You have been marked as being away\r\n",
        handler.stream.read_wbuf_to_string()
    );

    assert_eq!(
        Some("away message!".to_string()),
        handler.database.get_away_message("nickname")
    );
}

#[test]
fn can_unset_away_message_for_client() {
    let mut handler = dummy_client_handler();

    let trailing = Some("away message!".to_string());
    handler.away_command((None, vec![], trailing)).unwrap();
    handler.stream.clear();
    handler.away_command((None, vec![], None)).unwrap();

    assert_eq!(
        "305 :You are no longer marked as being away\r\n",
        handler.stream.read_wbuf_to_string()
    );

    assert_eq!(None, handler.database.get_away_message("nickname"));
}

#[test]
fn aways_are_relayed_to_all_servers() {
    let mut handler = dummy_client_handler();

    handler
        .database()
        .add_immediate_server(dummy_server("servername1"));
    handler
        .database()
        .add_immediate_server(dummy_server("servername2"));

    let trail = Some("message".to_string());
    handler.away_command((None, vec![], trail)).unwrap();
    handler.away_command((None, vec![], None)).unwrap();

    assert_eq!(
        ":nickname AWAY :message\r\n:nickname AWAY\r\n",
        handler
            .database
            .get_server_stream("servername1")
            .unwrap()
            .unwrap()
            .read_wbuf_to_string()
    );
    assert_eq!(
        ":nickname AWAY :message\r\n:nickname AWAY\r\n",
        handler
            .database
            .get_server_stream("servername2")
            .unwrap()
            .unwrap()
            .read_wbuf_to_string()
    );
}
