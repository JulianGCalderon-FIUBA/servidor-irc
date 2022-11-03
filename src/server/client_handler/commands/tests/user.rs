use crate::server::client_handler::registration::RegistrationState;

use super::*;

// #[test]
// fn user_fails_with_already_registered() {
//     let mut handler = dummy_client_handler();

//     let parameters = vec!["nick".to_string()];
//     handler.nick_command(parameters).unwrap();

//     let parameters = vec!["user".to_string(), "host".to_string(), "server".to_string()];
//     let trailing = Some("real".to_string());
//     handler.user_command(parameters, trailing).unwrap();

//     handler.stream.clear();

//     let parameters = vec![
//         "user2".to_string(),
//         "host2".to_string(),
//         "server2".to_string(),
//     ];
//     let trailing = Some("real2".to_string());
//     handler.user_command(parameters, trailing).unwrap();

//     assert_eq!(
//         "462 :you may not reregister\r\n",
//         handler.stream.read_wbuf_to_string()
//     );
// }

#[test]
fn user_fails_with_no_nickname_registered() {
    let mut handler = dummy_client_handler();

    let parameters = vec!["user".to_string(), "host".to_string(), "server".to_string()];
    let trailing = Some("real".to_string());
    handler.user_command(parameters, trailing).unwrap();

    assert_eq!(
        "200 :No nickname registered\r\n",
        handler.stream.read_wbuf_to_string()
    );
}

#[test]
fn user_fails_with_empty_parameters() {
    let mut handler = dummy_client_handler();

    let parameters = vec!["nick".to_string()];
    handler.nick_command(parameters).unwrap();

    handler.stream.clear();

    let parameters = vec![];
    let trailing = Some("real".to_string());

    handler.user_command(parameters, trailing).unwrap();

    assert_eq!(
        "461 USER :Not enough parameters\r\n",
        handler.stream.read_wbuf_to_string()
    );
}

#[test]
fn user_registers_client_correctly() {
    let mut handler = dummy_client_handler();

    let parameters = vec!["nick".to_string()];
    handler.nick_command(parameters).unwrap();

    let parameters = vec!["user".to_string(), "host".to_string(), "server".to_string()];
    let trailing = Some("real".to_string());
    handler.user_command(parameters, trailing).unwrap();

    let responses = handler.stream.get_responses();

    assert_eq!("200 :Success", responses[0]);
    assert_eq!("200 :Success", responses[1]);
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
