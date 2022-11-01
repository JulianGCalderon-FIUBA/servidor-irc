use crate::server::client_handler::registration::RegistrationState;

use super::*;

#[test]
fn user_adds_registers_client_correctly() {
    let mut handler = dummy_client_handler();

    let parameters = vec!["nick".to_string()];
    handler.nick_command(parameters).unwrap();

    let parameters = vec!["user".to_string(), "host".to_string(), "server".to_string()];
    let trailing = Some("real".to_string());
    handler.user_command(parameters, trailing).unwrap();

    let responses = handler.stream.get_responses();

    assert_eq!("200 :success", responses[0]);
    assert_eq!("200 :success", responses[1]);
    assert_eq!(
        "user",
        handler.registration.get_attribute("username").unwrap()
    );
    assert_eq!(
        "server",
        handler.registration.get_attribute("servername").unwrap()
    );
    assert_eq!(
        "host",
        handler.registration.get_attribute("hostname").unwrap()
    );
    assert_eq!(
        "real",
        handler.registration.get_attribute("realname").unwrap()
    );

    assert!(handler.database.contains_client("nick"));
    assert!(handler.registration.state() == &RegistrationState::Registered);
}

#[test]
fn user_is_only_valid_after_nick() {
    let mut handler = dummy_client_handler();

    let parameters = vec!["user".to_string(), "host".to_string(), "server".to_string()];
    let trailing = Some("real".to_string());
    handler.user_command(parameters, trailing).unwrap();

    assert_eq!(
        "200 :no nickname registered\r\n",
        handler.stream.read_wbuf_to_string()
    );
}
