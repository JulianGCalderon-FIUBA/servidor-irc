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
fn privmsg_fails_with_no_recipient() {
    let mut handler = dummy_client_handler();
    register_client(&mut handler, "nick");

    let parameters = vec![];
    let trailing = Some("message!".to_string());
    handler.privmsg_command(parameters, trailing).unwrap();

    assert_eq!(
        "411 :no recipient given (PRIVMSG)\r\n",
        handler.stream.read_wbuf_to_string()
    );
}

#[test]
fn privmsg_fails_with_no_text() {
    let mut handler = dummy_client_handler();
    register_client(&mut handler, "nick");

    let parameters = vec!["nick1".to_string()];
    let trailing = None;
    handler.privmsg_command(parameters, trailing).unwrap();

    assert_eq!(
        "412 :no text to send\r\n",
        handler.stream.read_wbuf_to_string()
    );
}

#[test]
fn privmsg_fails_with_invalid_target() {
    let mut handler = dummy_client_handler();
    register_client(&mut handler, "nick");

    let parameters = vec!["nick1".to_string()];
    let trailing = Some("message!".to_string());
    handler.privmsg_command(parameters, trailing).unwrap();

    let responses = handler.stream.get_responses();

    assert_eq!("401 nick1 :No such nick/channel", responses[0]);
    assert_eq!("200 :success", responses[1]);
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

    let responses = handler.stream.get_responses();

    assert_eq!("404 #channel :cannot send to channel", responses[0]);
    assert_eq!("200 :success", responses[1]);
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

    let responses = handler.stream.get_responses();

    assert_eq!(":nick PRIVMSG #channel :message!", responses[0]);

    assert_eq!("200 :success", responses[1]);

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
