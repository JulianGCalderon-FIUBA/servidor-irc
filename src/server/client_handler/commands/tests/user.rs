use crate::server::client_handler::connection::RegistrationState;

use super::*;

#[test]
fn user_adds_registers_client_correctly() {
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

    assert_eq!(
        handler.connection.get_attribute("username").unwrap(),
        "user"
    );
    assert_eq!(
        handler.connection.get_attribute("servername").unwrap(),
        "server"
    );
    assert_eq!(
        handler.connection.get_attribute("hostname").unwrap(),
        "host"
    );
    assert_eq!(
        handler.connection.get_attribute("realname").unwrap(),
        "real"
    );

    assert!(handler.database.contains_client("nick"));

    assert!(handler.connection.state() == &RegistrationState::Registered);
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
