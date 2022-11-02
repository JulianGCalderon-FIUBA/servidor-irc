use crate::server::client_handler::registration::RegistrationState;

use super::*;

#[test]
fn nick_fails_with_no_nickname_given() {
    let mut handler = dummy_client_handler();

    let parameters = vec![];
    handler.nick_command(parameters).unwrap();

    assert_eq!(
        "431 :no nickname given\r\n",
        handler.stream.read_wbuf_to_string()
    );
}

#[test]
fn changing_nick_fails_with_nickname_in_use() {
    let mut handler = dummy_client_handler();
    register_client(&mut handler, "nick");

    handler.database.add_client(dummy_client("nick2"));

    let parameters = vec!["nick2".to_string()];
    handler.nick_command(parameters).unwrap();

    assert_eq!(
        "433 nick2 :nickname is already in use\r\n",
        handler.stream.read_wbuf_to_string()
    );
}

#[test]
fn nick_fails_with_nickname_collision() {
    let mut handler = dummy_client_handler();
    handler.database.add_client(dummy_client("nick"));

    let parameters = vec!["nick".to_string()];
    handler.nick_command(parameters).unwrap();

    assert_eq!(
        "436 nick :nickname collision KILL\r\n",
        handler.stream.read_wbuf_to_string()
    );
}

#[test]
fn can_set_nickname() {
    let mut handler = dummy_client_handler();

    let parameters = vec!["nick".to_string()];
    handler.nick_command(parameters).unwrap();

    assert_eq!("200 :success\r\n", handler.stream.read_wbuf_to_string());

    assert_eq!("nick", handler.registration.nickname().unwrap());
    assert_eq!(
        &RegistrationState::NicknameSent,
        handler.registration.state()
    );
}

#[test]
fn can_update_nickname() {
    let mut handler = dummy_client_handler();
    register_client(&mut handler, "nick");

    let parameters = vec!["nick2".to_string()];
    handler.nick_command(parameters).unwrap();

    assert_eq!("200 :success\r\n", handler.stream.read_wbuf_to_string());

    assert_eq!("nick2", handler.registration.nickname().unwrap());

    // FALTA CAMBIAR BASE DE DATOS
}
