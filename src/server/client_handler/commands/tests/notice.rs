use super::*;

#[test]
fn notice_works_with_valid_target_client() {
    let mut handler = dummy_client_handler();
    register_client(&mut handler, "nick");

    handler.database.add_client(dummy_client("nick1"));

    let parameters = vec!["nick1".to_string()];
    let trailing = Some("message!".to_string());
    handler.notice_command(parameters, trailing).unwrap();

    assert_eq!(
        ":nick NOTICE nick1 :message!\r\n",
        handler
            .database
            .get_stream("nick1")
            .unwrap()
            .read_wbuf_to_string()
    );
}

#[test]
fn notice_with_away_client_does_not_return_away_message() {
    let mut handler = dummy_client_handler();
    register_client(&mut handler, "nick");

    handler.database.add_client(dummy_client("nick1"));
    handler
        .database
        .set_away_message(&Some("away message!".to_string()), "nick1");

    let parameters = vec!["nick1".to_string()];
    let trailing = Some("message!".to_string());
    handler.notice_command(parameters, trailing).unwrap();

    assert!(handler.stream.read_wbuf_to_string().is_empty());
}
