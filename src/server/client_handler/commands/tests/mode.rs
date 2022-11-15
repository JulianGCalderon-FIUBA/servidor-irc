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
fn mode_sets_channop() {
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
