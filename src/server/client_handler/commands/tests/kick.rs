use super::*;

#[test]
fn kick_fails_with_unregistered_client() {
    let mut handler = dummy_client_handler();

    let parameters = vec!["#canal1".to_string(), "user1".to_string()];
    handler.kick_command(parameters, None).unwrap();

    assert_eq!(
        "200 :Unregistered\r\n",
        handler.stream.read_wbuf_to_string()
    )
}

#[test]
fn join_fails_with_empty_params() {
    let mut handler = dummy_client_handler();
    register_client(&mut handler, "nick");

    handler.kick_command(vec![], None).unwrap();

    assert_eq!(
        "461 KICK :Not enough parameters\r\n",
        handler.stream.read_wbuf_to_string()
    );

    let channels: Vec<String> = vec![];
    assert_eq!(handler.database.get_all_channels(), channels);
}
