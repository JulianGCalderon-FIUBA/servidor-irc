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
fn kick_fails_with_empty_params() {
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

#[test]
fn kick_fails_when_not_on_channel() {
    let mut handler = dummy_client_handler();
    register_client(&mut handler, "nick");

    handler.database.add_client(dummy_client("nick1"));
    handler.database.add_client_to_channel("nick1", "#channel1");

    let parameters = vec!["#channel1".to_string(), "nick1".to_string()];
    handler.kick_command(parameters, None).unwrap();

    assert_eq!(
        "442 #channel1 :You're not on that channel\r\n",
        handler.stream.read_wbuf_to_string()
    );
}

#[test]
fn kick_fails_when_not_operator() {
    let mut handler = dummy_client_handler();

    register_client(&mut handler, "nick1");
    handler.database.add_client(dummy_client("nick2"));
    handler.database.add_client_to_channel("nick2", "#channel1");
    handler.database.add_client_to_channel("nick1", "#channel1");

    let parameters = vec!["#channel1".to_string(), "nick1".to_string()];
    handler.kick_command(parameters, None).unwrap();

    assert_eq!(
        "482 #channel1 :You're not channel operator\r\n",
        handler.stream.read_wbuf_to_string()
    );
}

#[test]
fn kick_fails_when_channel_does_not_exist() {
    let mut handler = dummy_client_handler();

    register_client(&mut handler, "nick");

    let parameters = vec!["#channel1".to_string(), "nick1".to_string()];
    handler.kick_command(parameters, None).unwrap();

    assert_eq!(
        "403 #channel1 :No such channel\r\n",
        handler.stream.read_wbuf_to_string()
    );
}

#[test]
fn can_kick_user_from_channel() {
    let mut handler = dummy_client_handler();

    register_client(&mut handler, "nick1");
    handler.database.add_client(dummy_client("nick2"));
    handler.database.add_client_to_channel("nick1", "#channel");
    handler.database.add_client_to_channel("nick2", "#channel");

    let parameters = vec!["#channel".to_string(), "nick2".to_string()];
    handler.kick_command(parameters, None).unwrap();

    assert!(!handler.database.is_client_in_channel("nick2", "#channel"));

    assert_eq!(
        ":nick1 KICK #channel nick2\r\n",
        handler
            .database
            .get_stream("nick2")
            .unwrap()
            .read_wbuf_to_string()
    );
}
