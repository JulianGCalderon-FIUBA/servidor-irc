use super::*;

#[test]
fn privmsg_fails_with_no_recipient() {
    let mut handler = dummy_client_handler();

    let parameters = vec![];
    let trailing = Some("message!".to_string());
    handler.privmsg_command(parameters, trailing).unwrap();

    assert_eq!(
        "411 :No recipient given (PRIVMSG)\r\n",
        handler.stream.read_wbuf_to_string()
    );
}

#[test]
fn privmsg_fails_with_no_text() {
    let mut handler = dummy_client_handler();

    let parameters = vec!["nick1".to_string()];
    let trailing = None;
    handler.privmsg_command(parameters, trailing).unwrap();

    assert_eq!(
        "412 :No text to send\r\n",
        handler.stream.read_wbuf_to_string()
    );
}

#[test]
fn privmsg_fails_with_invalid_target() {
    let mut handler = dummy_client_handler();

    let parameters = vec!["nick1".to_string()];
    let trailing = Some("message!".to_string());
    handler.privmsg_command(parameters, trailing).unwrap();

    let responses = handler.stream.get_responses();

    assert_eq!("401 nick1 :No such nick/channel", responses[0]);
}

#[test]
fn privmsg_works_with_valid_target_client() {
    let mut handler = dummy_client_handler();

    handler.database.add_client(dummy_client("nick1"));

    let parameters = vec!["nick1".to_string()];
    let trailing = Some("message!".to_string());
    handler.privmsg_command(parameters, trailing).unwrap();

    assert_eq!(
        ":nickname PRIVMSG nick1 :message!\r\n",
        handler
            .database
            .get_stream("nick1")
            .unwrap()
            .read_wbuf_to_string()
    );
}

#[test]
fn privmsg_works_with_valid_target_channel() {
    let mut handler = dummy_client_handler();

    handler.database.add_client(dummy_client("nick1"));
    handler.database.add_client(dummy_client("nick2"));
    handler.database.add_client_to_channel("nick1", "#channel");
    handler.database.add_client_to_channel("nick2", "#channel");
    handler
        .database
        .add_client_to_channel("nickname", "#channel");

    let parameters = vec!["#channel".to_string()];
    let trailing = Some("message!".to_string());
    handler.privmsg_command(parameters, trailing).unwrap();

    let responses = handler.stream.get_responses();

    assert_eq!(":nickname PRIVMSG #channel :message!", responses[0]);

    assert_eq!(
        ":nickname PRIVMSG #channel :message!\r\n",
        handler
            .database
            .get_stream("nick1")
            .unwrap()
            .read_wbuf_to_string()
    );

    assert_eq!(
        ":nickname PRIVMSG #channel :message!\r\n",
        handler
            .database
            .get_stream("nick2")
            .unwrap()
            .read_wbuf_to_string()
    );
}

#[test]
fn privmsg_works_with_multiple_targets() {
    let mut handler = dummy_client_handler();

    handler.database.add_client(dummy_client("nick1"));
    handler.database.add_client(dummy_client("nick2"));

    let parameters = vec!["nick1,nick2".to_string()];
    let trailing = Some("message!".to_string());
    handler.privmsg_command(parameters, trailing).unwrap();

    assert_eq!(
        ":nickname PRIVMSG nick1 :message!\r\n",
        handler
            .database
            .get_stream("nick1")
            .unwrap()
            .read_wbuf_to_string()
    );

    assert_eq!(
        ":nickname PRIVMSG nick2 :message!\r\n",
        handler
            .database
            .get_stream("nick2")
            .unwrap()
            .read_wbuf_to_string()
    );
}

#[test]
fn privmsg_with_away_client_returns_away_message() {
    let mut handler = dummy_client_handler();

    handler.database.add_client(dummy_client("nick1"));
    handler
        .database
        .set_away_message(&Some("away message!".to_string()), "nick1");

    let parameters = vec!["nick1".to_string()];
    let trailing = Some("message!".to_string());
    handler.privmsg_command(parameters, trailing).unwrap();

    assert_eq!(
        "301 nick1 :away message!\r\n",
        handler.stream.read_wbuf_to_string(),
    );
}

#[test]
fn privmsg_fails_with_not_on_channel_with_flag_n() {
    let mut handler = dummy_client_handler();

    handler.database.add_client(dummy_client("nick1"));
    handler.database.add_client(dummy_client("nick2"));
    handler.database.add_client_to_channel("nick1", "#channel");
    handler.database.add_client_to_channel("nick2", "#channel");

    handler.database.set_channel_mode("#channel", 'n');

    let parameters = vec!["#channel".to_string()];
    let trailing = Some("message!".to_string());
    handler.privmsg_command(parameters, trailing).unwrap();

    let responses = handler.stream.get_responses();

    assert_eq!("404 #channel :Cannot send to channel", responses[0]);

    assert_eq!(
        "",
        handler
            .database
            .get_stream("nick1")
            .unwrap()
            .read_wbuf_to_string()
    );

    assert_eq!(
        "",
        handler
            .database
            .get_stream("nick2")
            .unwrap()
            .read_wbuf_to_string()
    );
}

#[test]
fn privmsg_fails_if_not_speaker_on_channel_with_flag_m() {
    let mut handler = dummy_client_handler();

    handler.database.add_client(dummy_client("nick1"));
    handler.database.add_client_to_channel("nick1", "#channel");
    handler
        .database
        .add_client_to_channel("nickname", "#channel");

    handler.database.set_channel_mode("#channel", 'm');

    let parameters = vec!["#channel".to_string()];
    let trailing = Some("message!".to_string());
    handler.privmsg_command(parameters, trailing).unwrap();

    let responses = handler.stream.get_responses();

    assert_eq!("404 #channel :Cannot send to channel", responses[0]);

    assert_eq!(
        "",
        handler
            .database
            .get_stream("nick1")
            .unwrap()
            .read_wbuf_to_string()
    );
}

#[test]
fn privmsg_works_on_channel_with_flag_n() {
    let mut handler = dummy_client_handler();

    handler.database.add_client(dummy_client("nick1"));
    handler.database.add_client(dummy_client("nick2"));
    handler
        .database
        .add_client_to_channel("nickname", "#channel");
    handler.database.add_client_to_channel("nick1", "#channel");
    handler.database.add_client_to_channel("nick2", "#channel");

    handler.database.set_channel_mode("#channel", 'n');

    let parameters = vec!["#channel".to_string()];
    let trailing = Some("message!".to_string());
    handler.privmsg_command(parameters, trailing).unwrap();

    let responses = handler.stream.get_responses();

    assert_eq!(":nickname PRIVMSG #channel :message!", responses[0]);

    assert_eq!(
        ":nickname PRIVMSG #channel :message!\r\n",
        handler
            .database
            .get_stream("nick1")
            .unwrap()
            .read_wbuf_to_string()
    );

    assert_eq!(
        ":nickname PRIVMSG #channel :message!\r\n",
        handler
            .database
            .get_stream("nick2")
            .unwrap()
            .read_wbuf_to_string()
    );
}

#[test]
fn privmsg_works_on_channel_with_flag_m() {
    let mut handler = dummy_client_handler();

    handler.database.add_client(dummy_client("nick1"));
    handler.database.add_client_to_channel("nick1", "#channel");
    handler
        .database
        .add_client_to_channel("nickname", "#channel");

    handler.database.set_channel_mode("#channel", 'm');
    handler.database.add_speaker("#channel", "nickname");

    let parameters = vec!["#channel".to_string()];
    let trailing = Some("message!".to_string());
    handler.privmsg_command(parameters, trailing).unwrap();

    let responses = handler.stream.get_responses();

    assert_eq!(":nickname PRIVMSG #channel :message!", responses[0]);

    assert_eq!(
        ":nickname PRIVMSG #channel :message!\r\n",
        handler
            .database
            .get_stream("nick1")
            .unwrap()
            .read_wbuf_to_string()
    );
}
