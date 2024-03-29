use crate::server::testing::dummy_server;

use super::*;

#[test]
fn nick_fails_with_no_nickname_given() {
    let mut handler = dummy_client_handler();

    let parameters = vec![];
    handler.nick_command((None, parameters, None)).unwrap();

    assert_eq!(
        "431 :No nickname given\r\n",
        handler.stream.read_wbuf_to_string()
    );
}

#[test]
fn changing_nick_fails_with_nickname_in_use() {
    let mut handler = dummy_client_handler();

    handler.database.add_local_client(dummy_client("nick2"));

    let parameters = vec!["nick2".to_string()];
    handler.nick_command((None, parameters, None)).unwrap();

    assert_eq!(
        "433 nick2 :Nickname is already in use\r\n",
        handler.stream.read_wbuf_to_string()
    );
}

#[test]
fn can_set_nickname() {
    let mut handler = dummy_client_handler();

    let parameters = vec!["nick".to_string()];
    handler.nick_command((None, parameters, None)).unwrap();

    assert_eq!("", handler.stream.read_wbuf_to_string());

    assert_eq!("nick", handler.nickname);
}

#[test]
fn can_update_nickname() {
    let mut handler = dummy_client_handler();

    let parameters = vec!["nick2".to_string()];
    handler.nick_command((None, parameters, None)).unwrap();

    assert_eq!("nick2", handler.nickname);

    assert!(!handler.database.contains_client("nickname"));
    assert!(handler.database.contains_client("nick2"));
}

#[test]
fn after_nick_update_channel_info_is_updated() {
    let mut handler = dummy_client_handler();

    handler
        .database
        .add_client_to_channel("#channel", "nickname");

    let parameters = vec!["nick2".to_string()];
    handler.nick_command((None, parameters, None)).unwrap();

    assert_eq!(
        vec!["nick2".to_string()],
        handler.database.get_channel_clients("#channel").unwrap()
    );
}

#[test]
fn nick_update_is_relayed_to_all_servers() {
    let mut handler = dummy_client_handler();
    handler
        .database()
        .add_immediate_server(dummy_server("servername1"));
    handler
        .database()
        .add_immediate_server(dummy_server("servername2"));

    let parameters = vec!["nick2".to_string()];
    handler.nick_command((None, parameters, None)).unwrap();

    assert_eq!(
        ":nickname NICK nick2\r\n",
        handler
            .database
            .get_server_stream("servername1")
            .unwrap()
            .read_wbuf_to_string()
    );
    assert_eq!(
        ":nickname NICK nick2\r\n",
        handler
            .database
            .get_server_stream("servername2")
            .unwrap()
            .read_wbuf_to_string()
    );
}

#[test]
fn nick_fails_with_long_nickname() {
    let mut handler = dummy_client_handler();

    let parameters = vec!["nicknamenickname".to_string()];
    handler.nick_command((None, parameters, None)).unwrap();

    assert_eq!(
        "432 nicknamenickname :Erroneous nickname\r\n",
        handler.stream.read_wbuf_to_string()
    );
}

#[test]
fn nick_fails_with_invalid_prefix() {
    let mut handler = dummy_client_handler();

    let parameters = vec!["#nickname".to_string()];
    handler.nick_command((None, parameters, None)).unwrap();

    assert_eq!(
        "432 #nickname :Erroneous nickname\r\n",
        handler.stream.read_wbuf_to_string()
    );

    let parameters = vec!["&nickname".to_string()];
    handler.nick_command((None, parameters, None)).unwrap();

    assert_eq!(
        "432 &nickname :Erroneous nickname\r\n",
        handler.stream.read_wbuf_to_string()
    );

    let parameters = vec!["$nickname".to_string()];
    handler.nick_command((None, parameters, None)).unwrap();

    assert_eq!(
        "432 $nickname :Erroneous nickname\r\n",
        handler.stream.read_wbuf_to_string()
    );

    let parameters = vec![":nickname".to_string()];
    handler.nick_command((None, parameters, None)).unwrap();

    assert_eq!(
        "432 :nickname :Erroneous nickname\r\n",
        handler.stream.read_wbuf_to_string()
    );
}

#[test]
fn nick_fails_with_invalid_character() {
    let mut handler = dummy_client_handler();

    let parameters = vec!["nick.name".to_string()];
    handler.nick_command((None, parameters, None)).unwrap();

    assert_eq!(
        "432 nick.name :Erroneous nickname\r\n",
        handler.stream.read_wbuf_to_string()
    );

    let parameters = vec!["nickname!".to_string()];
    handler.nick_command((None, parameters, None)).unwrap();

    assert_eq!(
        "432 nickname! :Erroneous nickname\r\n",
        handler.stream.read_wbuf_to_string()
    );

    let parameters = vec!["ni,ckname".to_string()];
    handler.nick_command((None, parameters, None)).unwrap();

    assert_eq!(
        "432 ni,ckname :Erroneous nickname\r\n",
        handler.stream.read_wbuf_to_string()
    );

    let parameters = vec!["nick*name".to_string()];
    handler.nick_command((None, parameters, None)).unwrap();

    assert_eq!(
        "432 nick*name :Erroneous nickname\r\n",
        handler.stream.read_wbuf_to_string()
    );

    let parameters = vec!["ni?ckname".to_string()];
    handler.nick_command((None, parameters, None)).unwrap();

    assert_eq!(
        "432 ni?ckname :Erroneous nickname\r\n",
        handler.stream.read_wbuf_to_string()
    );

    let parameters = vec!["nickname@".to_string()];
    handler.nick_command((None, parameters, None)).unwrap();

    assert_eq!(
        "432 nickname@ :Erroneous nickname\r\n",
        handler.stream.read_wbuf_to_string()
    );
}
