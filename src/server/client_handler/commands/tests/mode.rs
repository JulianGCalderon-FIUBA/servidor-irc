use super::*;

#[test]
fn mode_fails_with_unregistered_client() {
    let mut handler = dummy_client_handler();

    let parameters = vec!["#channel".to_string()];
    handler.mode_command(parameters).unwrap();

    assert_eq!(
        "200 :Unregistered\r\n",
        handler.stream.read_wbuf_to_string()
    )
}

#[test]
fn mode_fails_with_not_enough_parameters() {
    let mut handler = dummy_client_handler();

    let parameters = vec![];
    handler.mode_command(parameters).unwrap();

    assert_eq!(
        "461 MODE :Not enough parameters\r\n",
        handler.stream.read_wbuf_to_string()
    )
}

#[test]
fn mode_fails_with_nonexistent_channel() {
    let mut handler = dummy_client_handler();
    register_client(&mut handler, "nickname");

    let parameters = vec!["#channel".to_string()];
    handler.mode_command(parameters).unwrap();

    assert_eq!(
        "403 #channel :No such channel\r\n",
        handler.stream.read_wbuf_to_string()
    )
}

#[test]
fn mode_fails_with_user_not_on_channel() {
    let mut handler = dummy_client_handler();
    register_client(&mut handler, "nickname");

    handler.database.add_client(dummy_client("dummy"));
    handler.database.add_client_to_channel("dummy", "#channel");

    let parameters = vec!["#channel".to_string()];
    handler.mode_command(parameters).unwrap();

    assert_eq!(
        "442 #channel :You're not on that channel\r\n",
        handler.stream.read_wbuf_to_string()
    )
}

#[test]
fn mode_fails_with_user_not_operator() {
    let mut handler = dummy_client_handler();
    register_client(&mut handler, "nickname");

    handler.database.add_client(dummy_client("dummy"));
    handler.database.add_client_to_channel("dummy", "#channel");
    handler
        .database
        .add_client_to_channel("nickname", "#channel");

    let parameters = vec!["#channel".to_string(), "+i".to_string()];
    handler.mode_command(parameters).unwrap();

    assert_eq!(
        "482 #channel :You're not channel operator\r\n",
        handler.stream.read_wbuf_to_string()
    )
}

#[test]
fn mode_ignores_wrong_parameters() {
    let mut handler = dummy_client_handler();
    register_client(&mut handler, "nickname");

    handler
        .database
        .add_client_to_channel("nickname", "#channel");

    let parameters = vec!["#channel".to_string(), "i".to_string()];
    handler.mode_command(parameters).unwrap();

    assert_eq!("", handler.stream.read_wbuf_to_string())
}

#[test]
fn mode_adds_channop() {
    let mut handler = dummy_client_handler();
    register_client(&mut handler, "nickname");

    handler
        .database
        .add_client_to_channel("nickname", "#channel");

    handler.database.add_client(dummy_client("nick2"));
    handler.database.add_client_to_channel("nick2", "#channel");

    assert!(!handler.database.is_channel_operator("#channel", "nick2"));

    let parameters = vec![
        "#channel".to_string(),
        "+o".to_string(),
        "nick2".to_string(),
    ];
    handler.mode_command(parameters).unwrap();

    assert_eq!("", handler.stream.read_wbuf_to_string());
    assert!(handler.database.is_channel_operator("#channel", "nick2"));
}

#[test]
fn mode_adds_multiple_channops() {
    let mut handler = dummy_client_handler();
    register_client(&mut handler, "nickname");

    handler
        .database
        .add_client_to_channel("nickname", "#channel");

    handler.database.add_client(dummy_client("nick2"));
    handler.database.add_client_to_channel("nick2", "#channel");
    handler.database.add_client(dummy_client("nick3"));
    handler.database.add_client_to_channel("nick3", "#channel");
    handler.database.add_client(dummy_client("nick4"));
    handler.database.add_client_to_channel("nick4", "#channel");
    handler.database.add_client(dummy_client("nick5"));
    handler.database.add_client_to_channel("nick5", "#channel");

    assert!(!handler.database.is_channel_operator("#channel", "nick2"));
    assert!(!handler.database.is_channel_operator("#channel", "nick3"));
    assert!(!handler.database.is_channel_operator("#channel", "nick4"));
    assert!(!handler.database.is_channel_operator("#channel", "nick5"));

    let parameters = vec![
        "#channel".to_string(),
        "+o".to_string(),
        "nick2,nick3,nick4,nick5".to_string(),
    ];
    handler.mode_command(parameters).unwrap();

    assert_eq!("", handler.stream.read_wbuf_to_string());
    assert!(handler.database.is_channel_operator("#channel", "nickname"));
    assert!(handler.database.is_channel_operator("#channel", "nick2"));
    assert!(handler.database.is_channel_operator("#channel", "nick3"));
    assert!(handler.database.is_channel_operator("#channel", "nick4"));
    assert!(!handler.database.is_channel_operator("#channel", "nick5"));
}

#[test]
fn mode_removes_channop() {
    let mut handler = dummy_client_handler();
    register_client(&mut handler, "nickname");

    handler
        .database
        .add_client_to_channel("nickname", "#channel");

    handler.database.add_client(dummy_client("nick2"));
    handler.database.add_client_to_channel("nick2", "#channel");
    handler.database.add_channop("#channel", "nick2");

    assert!(handler.database.is_channel_operator("#channel", "nick2"));

    let parameters = vec![
        "#channel".to_string(),
        "-o".to_string(),
        "nick2".to_string(),
    ];
    handler.mode_command(parameters).unwrap();

    assert_eq!("", handler.stream.read_wbuf_to_string());
    assert!(handler.database.is_channel_operator("#channel", "nickname"));
    assert!(!handler.database.is_channel_operator("#channel", "nick2"));
}

#[test]
fn mode_removes_multiple_channops() {
    let mut handler = dummy_client_handler();
    register_client(&mut handler, "nickname");

    handler
        .database
        .add_client_to_channel("nickname", "#channel");

    handler.database.add_client(dummy_client("nick2"));
    handler.database.add_client_to_channel("nick2", "#channel");
    handler.database.add_client(dummy_client("nick3"));
    handler.database.add_client_to_channel("nick3", "#channel");

    assert!(!handler.database.is_channel_operator("#channel", "nick2"));
    assert!(!handler.database.is_channel_operator("#channel", "nick3"));

    let parameters = vec![
        "#channel".to_string(),
        "-o".to_string(),
        "nick2,nick3".to_string(),
    ];
    handler.mode_command(parameters).unwrap();

    assert_eq!("", handler.stream.read_wbuf_to_string());
    assert!(handler.database.is_channel_operator("#channel", "nickname"));
    assert!(!handler.database.is_channel_operator("#channel", "nick2"));
    assert!(!handler.database.is_channel_operator("#channel", "nick3"));
}

#[test]
fn mode_fails_with_no_oper_parameter() {
    let mut handler = dummy_client_handler();
    register_client(&mut handler, "nickname");

    handler
        .database
        .add_client_to_channel("nickname", "#channel");

    handler.database.add_client(dummy_client("nick2"));
    handler.database.add_client_to_channel("nick2", "#channel");

    assert!(!handler.database.is_channel_operator("#channel", "nick2"));

    let parameters = vec!["#channel".to_string(), "+o".to_string()];
    handler.mode_command(parameters).unwrap();

    assert_eq!(
        "461 MODE :Not enough parameters\r\n",
        handler.stream.read_wbuf_to_string()
    );
    assert!(!handler.database.is_channel_operator("#channel", "nick2"));
}

#[test]
fn mode_sets_limit_to_channel() {
    let mut handler = dummy_client_handler();
    register_client(&mut handler, "nickname");

    handler
        .database
        .add_client_to_channel("nickname", "#channel");

    assert!(handler.database.get_channel_limit("#channel").is_none());

    let parameters = vec!["#channel".to_string(), "+l".to_string(), "5".to_string()];
    handler.mode_command(parameters).unwrap();

    assert_eq!("", handler.stream.read_wbuf_to_string());
    assert_eq!(handler.database.get_channel_limit("#channel"), Some(5));
}

#[test]
fn mode_unsets_channel_limit() {
    let mut handler = dummy_client_handler();
    register_client(&mut handler, "nickname");

    handler
        .database
        .add_client_to_channel("nickname", "#channel");
    handler.database.set_channel_limit("#channel", Some(5));

    assert_eq!(handler.database.get_channel_limit("#channel"), Some(5));

    let parameters = vec!["#channel".to_string(), "-l".to_string()];
    handler.mode_command(parameters).unwrap();

    assert_eq!("", handler.stream.read_wbuf_to_string());
    assert!(handler.database.get_channel_limit("#channel").is_none());
}

#[test]
fn mode_fails_with_no_limit_parameter() {
    let mut handler = dummy_client_handler();
    register_client(&mut handler, "nickname");

    handler
        .database
        .add_client_to_channel("nickname", "#channel");

    assert!(handler.database.get_channel_limit("#channel").is_none());

    let parameters = vec!["#channel".to_string(), "+l".to_string()];
    handler.mode_command(parameters).unwrap();

    assert_eq!(
        "461 MODE :Not enough parameters\r\n",
        handler.stream.read_wbuf_to_string()
    );
    assert!(handler.database.get_channel_limit("#channel").is_none());
}
