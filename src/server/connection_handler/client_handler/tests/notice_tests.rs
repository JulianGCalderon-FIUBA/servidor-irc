use super::*;

#[test]
fn notice_works_with_valid_target_client() {
    let mut handler = dummy_client_handler();

    handler.database.add_local_client(dummy_client("nick1"));

    let parameters = vec!["nick1".to_string()];
    let trailing = Some("message!".to_string());
    handler
        .notice_command((None, parameters, trailing))
        .unwrap();

    assert_eq!(
        ":nickname NOTICE nick1 :message!\r\n",
        handler
            .database
            .get_local_stream("nick1")
            .unwrap()
            .read_wbuf_to_string()
    );
}

#[test]
fn notice_with_away_client_does_not_return_away_message() {
    let mut handler = dummy_client_handler();

    handler.database.add_local_client(dummy_client("nick1"));
    handler
        .database
        .set_away_message(&Some("away message!".to_string()), "nick1");

    let parameters = vec!["nick1".to_string()];
    let trailing = Some("message!".to_string());
    handler
        .notice_command((None, parameters, trailing))
        .unwrap();

    assert!(handler.stream.read_wbuf_to_string().is_empty());
}

#[test]
fn notice_fails_with_not_on_channel_with_flag_n() {
    let mut handler = dummy_client_handler();

    handler.database.add_local_client(dummy_client("nick1"));
    handler.database.add_local_client(dummy_client("nick2"));
    handler.database.add_client_to_channel("nick1", "#channel");
    handler.database.add_client_to_channel("nick2", "#channel");

    handler
        .database
        .set_channel_mode("#channel", ChannelFlag::NoOutsideMessages);

    let parameters = vec!["#channel".to_string()];
    let trailing = Some("message!".to_string());
    handler
        .notice_command((None, parameters, trailing))
        .unwrap();

    let responses = handler.stream.get_responses();

    assert_eq!("404 #channel :Cannot send to channel", responses[0]);

    assert_eq!(
        "",
        handler
            .database
            .get_local_stream("nick1")
            .unwrap()
            .read_wbuf_to_string()
    );

    assert_eq!(
        "",
        handler
            .database
            .get_local_stream("nick2")
            .unwrap()
            .read_wbuf_to_string()
    );
}

#[test]
fn notice_fails_if_not_speaker_on_channel_with_flag_m() {
    let mut handler = dummy_client_handler();

    handler.database.add_local_client(dummy_client("nick1"));
    handler.database.add_client_to_channel("nick1", "#channel");
    handler
        .database
        .add_client_to_channel("nickname", "#channel");

    handler
        .database
        .set_channel_mode("#channel", ChannelFlag::Moderated);

    let parameters = vec!["#channel".to_string()];
    let trailing = Some("message!".to_string());
    handler
        .notice_command((None, parameters, trailing))
        .unwrap();

    let responses = handler.stream.get_responses();

    assert_eq!("404 #channel :Cannot send to channel", responses[0]);

    assert_eq!(
        "",
        handler
            .database
            .get_local_stream("nick1")
            .unwrap()
            .read_wbuf_to_string()
    );
}

#[test]
fn notice_works_on_channel_with_flag_n() {
    let mut handler = dummy_client_handler();

    handler.database.add_local_client(dummy_client("nick1"));
    handler.database.add_local_client(dummy_client("nick2"));
    handler
        .database
        .add_client_to_channel("nickname", "#channel");
    handler.database.add_client_to_channel("nick1", "#channel");
    handler.database.add_client_to_channel("nick2", "#channel");

    handler
        .database
        .set_channel_mode("#channel", ChannelFlag::NoOutsideMessages);

    let parameters = vec!["#channel".to_string()];
    let trailing = Some("message!".to_string());
    handler
        .notice_command((None, parameters, trailing))
        .unwrap();

    let responses = handler.stream.get_responses();

    assert_eq!(":nickname NOTICE #channel :message!", responses[0]);

    assert_eq!(
        ":nickname NOTICE #channel :message!\r\n",
        handler
            .database
            .get_local_stream("nick1")
            .unwrap()
            .read_wbuf_to_string()
    );

    assert_eq!(
        ":nickname NOTICE #channel :message!\r\n",
        handler
            .database
            .get_local_stream("nick2")
            .unwrap()
            .read_wbuf_to_string()
    );
}

#[test]
fn notice_works_on_channel_with_flag_m() {
    let mut handler = dummy_client_handler();

    handler.database.add_local_client(dummy_client("nick1"));
    handler.database.add_client_to_channel("nick1", "#channel");
    handler
        .database
        .add_client_to_channel("nickname", "#channel");

    handler
        .database
        .set_channel_mode("#channel", ChannelFlag::Moderated);
    handler.database.add_speaker("#channel", "nickname");

    let parameters = vec!["#channel".to_string()];
    let trailing = Some("message!".to_string());
    handler
        .notice_command((None, parameters, trailing))
        .unwrap();

    let responses = handler.stream.get_responses();

    assert_eq!(":nickname NOTICE #channel :message!", responses[0]);

    assert_eq!(
        ":nickname NOTICE #channel :message!\r\n",
        handler
            .database
            .get_local_stream("nick1")
            .unwrap()
            .read_wbuf_to_string()
    );
}
