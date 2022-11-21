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
    handler.database.add_client(dummy_client("nickname"));

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
