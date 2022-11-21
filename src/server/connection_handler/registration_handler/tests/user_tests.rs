use super::*;

#[test]
fn user_fails_with_no_nickname_registered() {
    let mut handler = dummy_registration_handler();

    let parameters = vec!["username".to_string()];
    let trailing = Some("realname".to_string());
    handler.user_command((None, parameters, trailing)).unwrap();

    assert_eq!(
        "200 :No nickname registered\r\n",
        handler.stream.read_wbuf_to_string()
    );
}

#[test]
fn user_fails_with_empty_parameters() {
    let mut handler = dummy_registration_handler();

    let parameters = vec!["nickname".to_string()];
    handler.nick_command((None, parameters, None)).unwrap();
    handler.stream.clear();

    let parameters = vec![];
    let trailing = None;
    handler.user_command((None, parameters, trailing)).unwrap();

    assert_eq!(
        "461 USER :Not enough parameters\r\n",
        handler.stream.read_wbuf_to_string()
    );
}

#[test]
fn user_registers_client_correctly() {
    let mut handler = dummy_registration_handler();

    let parameters = vec!["nickname".to_string()];
    handler.nick_command((None, parameters, None)).unwrap();

    let parameters = vec!["username".to_string()];
    let trailing = Some("realname".to_string());
    handler.user_command((None, parameters, trailing)).unwrap();

    assert!(handler.database.contains_client("nickname"));
}
