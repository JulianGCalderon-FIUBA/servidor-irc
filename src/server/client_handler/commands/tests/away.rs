use super::*;

#[test]
fn away_fails_with_unregistered_client() {
    let mut handler = dummy_client_handler();

    let trailing = Some("away message!".to_string());
    handler.away_command(trailing).unwrap();

    assert_eq!(
        "200 :Unregistered\r\n",
        handler.stream.read_wbuf_to_string()
    )
}

#[test]
fn can_set_away_message_for_client() {
    let mut handler = dummy_client_handler();
    register_client(&mut handler, "nick");

    let trailing = Some("away message!".to_string());
    handler.away_command(trailing).unwrap();

    assert_eq!(
        "306 :You have been marked as being away\r\n",
        handler.stream.read_wbuf_to_string()
    );

    assert_eq!(
        Some("away message!".to_string()),
        handler.database.get_away_message("nick")
    );
}

#[test]
fn can_unset_away_message_for_client() {
    let mut handler = dummy_client_handler();
    register_client(&mut handler, "nick");

    let trailing = Some("away message!".to_string());
    handler.away_command(trailing).unwrap();
    handler.stream.clear();
    handler.away_command(None).unwrap();

    assert_eq!(
        "305 :You are no longer marked as being away\r\n",
        handler.stream.read_wbuf_to_string()
    );

    assert_eq!(None, handler.database.get_away_message("nick"));
}
