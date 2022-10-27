use crate::server::client_handler::registration::RegistrationState;

use super::*;

#[test]
fn nick_sets_connection_nickname() {
    let mut handler = dummy_client_handler();

    let parameters = vec!["nick".to_string()];
    handler.nick_command(parameters).unwrap();

    assert_eq!("200 :success\r\n", handler.stream.read_wbuf_to_string());

    assert_eq!(handler.registration.nickname().unwrap(), "nick");
    assert!(handler.registration.state() == &RegistrationState::NicknameSent);
}

#[test]
fn registering_used_nick_returns_collision_error() {
    let mut handler = dummy_client_handler();
    handler.database.add_client(dummy_client("nick"));

    let parameters = vec!["nick".to_string()];
    handler.nick_command(parameters).unwrap();

    assert_eq!(
        "436 :nickname collision KILL\r\n",
        handler.stream.read_wbuf_to_string()
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
        handler.stream.read_wbuf_to_string()
    );
}
