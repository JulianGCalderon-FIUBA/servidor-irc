use super::*;

#[test]
fn nick_fails_with_no_nickname_given() {
    let mut handler = dummy_client_handler();

    let parameters = vec![];
    handler.nick_command(parameters).unwrap();

    assert_eq!(
        "431 :No nickname given\r\n",
        handler.stream.read_wbuf_to_string()
    );
}

#[test]
fn changing_nick_fails_with_nickname_in_use() {
    let mut handler = dummy_client_handler();

    handler.database.add_client(dummy_client("nick2"));

    let parameters = vec!["nick2".to_string()];
    handler.nick_command(parameters).unwrap();

    assert_eq!(
        "433 nick2 :Nickname is already in use\r\n",
        handler.stream.read_wbuf_to_string()
    );
}

#[test]
fn can_set_nickname() {
    let mut handler = dummy_client_handler();

    let parameters = vec!["nick".to_string()];
    handler.nick_command(parameters).unwrap();

    assert_eq!("", handler.stream.read_wbuf_to_string());

    assert_eq!("nick", handler.nickname);
}

#[test]
fn can_update_nickname() {
    let mut handler = dummy_client_handler();

    let parameters = vec!["nick2".to_string()];
    handler.nick_command(parameters).unwrap();

    assert_eq!("nick2", handler.nickname);

    assert!(!handler.database.contains_client("nickname"));
    assert!(handler.database.contains_client("nick2"));
}