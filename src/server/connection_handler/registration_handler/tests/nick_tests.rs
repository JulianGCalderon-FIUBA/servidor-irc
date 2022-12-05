use crate::server::testing::dummy_client;

use super::*;

#[test]
fn nick_fails_with_no_nickname_given() {
    let mut handler = dummy_registration_handler();

    let parameters = vec![];
    handler.nick_command((None, parameters, None)).unwrap();

    assert_eq!(
        "431 :No nickname given\r\n",
        handler.stream.read_wbuf_to_string()
    );
}

#[test]
fn nick_fails_with_nickname_collision() {
    let mut handler = dummy_registration_handler();
    handler.database.add_local_client(dummy_client("nickname"));

    let parameters = vec!["nickname".to_string()];
    handler.nick_command((None, parameters, None)).unwrap();

    assert_eq!(
        "436 nickname :Nickname collision KILL\r\n",
        handler.stream.read_wbuf_to_string()
    );
}

#[test]
fn can_set_nickname() {
    let mut handler = dummy_registration_handler();

    let parameters = vec!["nickname".to_string()];
    handler.nick_command((None, parameters, None)).unwrap();

    assert_eq!("", handler.stream.read_wbuf_to_string());

    assert_eq!("nickname", handler.attributes.get("nickname").unwrap());
}

#[test]
fn nick_fails_with_long_nickname() {
    let mut handler = dummy_registration_handler();

    let parameters = vec!["nicknamenickname".to_string()];
    handler.nick_command((None, parameters, None)).unwrap();

    assert_eq!(
        "432 nicknamenickname :Erroneous nickname\r\n",
        handler.stream.read_wbuf_to_string()
    );
}

#[test]
fn nick_fails_with_invalid_prefix() {
    let mut handler = dummy_registration_handler();

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
    let mut handler = dummy_registration_handler();

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
