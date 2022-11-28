use crate::server::testing::dummy_server;

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

#[test]
fn user_notifies_all_servers() {
    let mut handler = dummy_registration_handler();
    handler
        .database
        .add_immediate_server(dummy_server("servername1"));
    handler
        .database
        .add_immediate_server(dummy_server("servername2"));

    let parameters = vec!["nickname".to_string()];
    handler.nick_command((None, parameters, None)).unwrap();

    let parameters = vec!["username".to_string()];
    let trailing = Some("realname".to_string());
    handler.user_command((None, parameters, trailing)).unwrap();

    assert_eq!(
        "NICK nickname 1\r\n:nickname USER username 127.0.0.1 servername :realname\r\n",
        handler
            .database
            .get_server_stream("servername1")
            .unwrap()
            .read_wbuf_to_string()
    );
    assert_eq!(
        "NICK nickname 1\r\n:nickname USER username 127.0.0.1 servername :realname\r\n",
        handler
            .database
            .get_server_stream("servername2")
            .unwrap()
            .read_wbuf_to_string()
    );
}
