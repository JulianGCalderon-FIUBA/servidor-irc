use super::*;

#[test]
fn pass_sets_connection_password() {
    let mut handler = dummy_client_handler();

    let parameters = vec!["pass".to_string()];
    handler.pass_command(parameters).unwrap();

    let parameters = vec!["pass2".to_string()];
    handler.pass_command(parameters).unwrap();

    assert_eq!(
        "200 :success\r\n200 :success\r\n",
        handler.stream_client_handler.read_wbuf_to_string()
    );

    assert_eq!(handler.connection.password, Some("pass2".to_string()));
}

#[test]
fn pass_is_only_valid_as_first_command() {
    let mut handler = dummy_client_handler();

    let parameters = vec!["nick".to_string()];
    handler.nick_command(parameters).unwrap();

    let parameters = vec!["pass".to_string()];
    handler.pass_command(parameters).unwrap();

    assert_eq!(
        "200 :success\r\n462 :may not reregister\r\n",
        handler.stream_client_handler.read_wbuf_to_string()
    )
}

#[test]
fn nick_sets_connection_nickname() {
    let mut handler = dummy_client_handler();

    let parameters = vec!["nick".to_string()];
    handler.nick_command(parameters).unwrap();

    assert_eq!(
        "200 :success\r\n",
        handler.stream_client_handler.read_wbuf_to_string()
    );

    assert_eq!(handler.connection.nickname(), "nick");
}

#[test]
fn registering_used_nick_returns_collision_error() {
    let mut handler = dummy_client_handler();
    handler.database.add_client(dummy_client("nick"));

    let parameters = vec!["nick".to_string()];
    handler.nick_command(parameters).unwrap();

    assert_eq!(
        "436 :nickname collision KILL\r\n",
        handler.stream_client_handler.read_wbuf_to_string()
    );
}

#[test]
fn changing_nick_used_nick_returns_in_use_error() {
    let mut handler = dummy_client_handler();
    register_client(&mut handler, "nick");

    handler.database.add_client(dummy_client("nick2"));

    let parameters = vec!["nick2".to_string()];
    handler.nick_command(parameters).unwrap();

    assert_eq!(
        "433 :nickname is already in use\r\n",
        handler.stream_client_handler.read_wbuf_to_string()
    );
}

#[test]
fn user_adds_client_to_database() {
    let mut handler = dummy_client_handler();

    let parameters = vec!["nick".to_string()];
    handler.nick_command(parameters).unwrap();

    let parameters = vec!["user".to_string(), "host".to_string(), "server".to_string()];
    let trailing = Some("real".to_string());
    handler.user_command(parameters, trailing).unwrap();

    assert_eq!(
        "200 :success\r\n200 :success\r\n",
        handler.stream_client_handler.read_wbuf_to_string()
    );

    assert_eq!(handler.connection.nickname(), "nick");

    assert!(handler.database.contains_client("nick"));
}

#[test]
fn user_is_only_valid_after_nick() {
    let mut handler = dummy_client_handler();

    let parameters = vec!["user".to_string(), "host".to_string(), "server".to_string()];
    let trailing = Some("real".to_string());
    handler.user_command(parameters, trailing).unwrap();

    assert_eq!(
        "200 :no nickname registered\r\n",
        handler.stream_client_handler.read_wbuf_to_string()
    );
}

#[test]
fn oper_with_valid_credential_sets_client_as_operator() {
    let mut handler = dummy_client_handler();
    register_client(&mut handler, "nick");

    let parameters = vec!["admin".to_string(), "admin".to_string()];
    handler.oper_command(parameters).unwrap();

    assert_eq!(
        "381 :You are now an IRC operator\r\n",
        handler.stream_client_handler.read_wbuf_to_string()
    );

    assert!(handler.database._is_server_operator("nick"));
}

#[test]
fn oper_is_only_valid_after_registration() {
    let mut handler = dummy_client_handler();

    let parameters = vec!["admin".to_string(), "admin".to_string()];
    handler.oper_command(parameters).unwrap();

    assert_eq!(
        "200 :unregistered\r\n",
        handler.stream_client_handler.read_wbuf_to_string()
    );
}
