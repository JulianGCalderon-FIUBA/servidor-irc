use super::*;

#[test]
fn privmsg_fails_with_unregistered_client() {
    let mut handler = dummy_client_handler();

    let parameters = vec!["nick1".to_string()];
    let trailing = Some("message!".to_string());
    handler.privmsg_command(parameters, trailing).unwrap();

    assert_eq!(
        "200 :unregistered\r\n",
        handler.stream.read_wbuf_to_string()
    )
}

#[test]
fn privmsg_fails_with_invalid_target() {
    let mut handler = dummy_client_handler();
    register_client(&mut handler, "nick");

    let parameters = vec!["nick1".to_string()];
    let trailing = Some("message!".to_string());
    handler.privmsg_command(parameters, trailing).unwrap();

    assert_eq!(
        "401 nick1 :No such nick/channel\r\n200 :success\r\n",
        handler.stream.read_wbuf_to_string()
    )
}

#[test]
fn privmsg_works_with_valid_target_client() {
    let mut handler = dummy_client_handler();
    register_client(&mut handler, "nick");

    handler.database.add_client(dummy_client("nick1"));

    let parameters = vec!["nick1".to_string()];
    let trailing = Some("message!".to_string());
    handler.privmsg_command(parameters, trailing).unwrap();

    assert_eq!("200 :success\r\n", handler.stream.read_wbuf_to_string());

    assert_eq!(
        ":nick PRIVMSG nick1 :message!\r\n",
        handler
            .database
            .get_stream("nick1")
            .unwrap()
            .lock()
            .unwrap()
            .read_wbuf_to_string()
    );
}

#[test]
fn privmsg_works_with_valid_target_channel() {
    let mut handler = dummy_client_handler();
    register_client(&mut handler, "nick");

    handler.database.add_client(dummy_client("nick1"));
    handler.database.add_client(dummy_client("nick2"));
    handler.database.add_client_to_channel("nick", "#channel");
    handler.database.add_client_to_channel("nick1", "#channel");
    handler.database.add_client_to_channel("nick2", "#channel");

    let parameters = vec!["#channel".to_string()];
    let trailing = Some("message!".to_string());
    handler.privmsg_command(parameters, trailing).unwrap();

    assert_eq!(
        ":nick PRIVMSG #channel :message!\r\n200 :success\r\n",
        handler.stream.read_wbuf_to_string()
    );

    assert_eq!(
        ":nick PRIVMSG #channel :message!\r\n",
        handler
            .database
            .get_stream("nick1")
            .unwrap()
            .lock()
            .unwrap()
            .read_wbuf_to_string()
    );

    assert_eq!(
        ":nick PRIVMSG #channel :message!\r\n",
        handler
            .database
            .get_stream("nick2")
            .unwrap()
            .lock()
            .unwrap()
            .read_wbuf_to_string()
    );
}

#[test]
fn privmsg_works_with_multiple_targets() {
    let mut handler = dummy_client_handler();
    register_client(&mut handler, "nick");

    handler.database.add_client(dummy_client("nick1"));
    handler.database.add_client(dummy_client("nick2"));

    let parameters = vec!["nick1,nick2".to_string()];
    let trailing = Some("message!".to_string());
    handler.privmsg_command(parameters, trailing).unwrap();

    assert_eq!("200 :success\r\n", handler.stream.read_wbuf_to_string());

    assert_eq!(
        ":nick PRIVMSG nick1 :message!\r\n",
        handler
            .database
            .get_stream("nick1")
            .unwrap()
            .lock()
            .unwrap()
            .read_wbuf_to_string()
    );

    assert_eq!(
        ":nick PRIVMSG nick2 :message!\r\n",
        handler
            .database
            .get_stream("nick2")
            .unwrap()
            .lock()
            .unwrap()
            .read_wbuf_to_string()
    );
}

#[test]
fn privmsg_fails_when_not_on_channel() {
    let mut handler = dummy_client_handler();
    register_client(&mut handler, "nick");

    handler.database.add_client(dummy_client("nick1"));
    handler.database.add_client_to_channel("nick1", "#channel");

    let parameters = vec!["#channel".to_string()];
    let trailing = Some("message!".to_string());
    handler.privmsg_command(parameters, trailing).unwrap();

    assert_eq!(
        "404 #channel :cannot send to channel\r\n200 :success\r\n",
        handler.stream.read_wbuf_to_string()
    );
}
