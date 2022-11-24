use crate::server::testing::{dummy_client, dummy_server};

use super::*;

#[test]
fn server_fails_with_not_enough_parameters() {
    let mut handler = dummy_registration_handler();

    handler.server_command((None, vec![], None)).unwrap();

    assert_eq!(
        "461 SERVER :Not enough parameters\r\n",
        handler.stream.read_wbuf_to_string()
    );
}

#[test]
fn server_fails_with_no_numeric_hopcount() {
    let mut handler = dummy_registration_handler();

    let parameters = vec!["servername1".to_string(), "nonumeric".to_string()];
    let trail = Some("serverinfo".to_string());
    handler.server_command((None, parameters, trail)).unwrap();

    assert_eq!(
        "400 SERVER :Hopcount is not numeric\r\n",
        handler.stream.read_wbuf_to_string()
    );
}

#[test]
fn server_fails_when_servername_already_registered() {
    let mut handler = dummy_registration_handler();
    handler.database.add_server(dummy_server("servername1"));

    let parameters = vec!["servername1".to_string(), "1".to_string()];
    let trail = Some("serverinfo".to_string());
    let result = handler.server_command((None, parameters, trail));

    assert!(result.is_err());

    assert_eq!(
        "400 SERVER :Servername already registered\r\n",
        handler.stream.read_wbuf_to_string()
    );
}

#[test]
fn server_sets_connection_type() {
    let mut handler = dummy_registration_handler();

    let parameters = vec!["servername1".to_string(), "1".to_string()];
    let trail = Some("serverinfo".to_string());
    handler.server_command((None, parameters, trail)).unwrap();

    assert!(matches!(handler.connection_type, ConnectionType::Server));
}

#[test]
fn server_sends_back_server() {
    let mut handler = dummy_registration_handler();

    let parameters = vec!["servername1".to_string(), "1".to_string()];
    let trail = Some("serverinfo".to_string());
    handler.server_command((None, parameters, trail)).unwrap();

    assert_eq!(
        "SERVER servername 1 :serverinfo\r\n",
        handler.stream.read_wbuf_to_string()
    );
}

#[test]
fn server_sends_back_client_info() {
    let mut handler = dummy_registration_handler();

    handler.database.add_client(dummy_client("nickname1"));
    handler.database.add_client(dummy_client("nickname2"));

    let parameters = vec!["servername1".to_string(), "1".to_string()];
    let trail = Some("serverinfo".to_string());
    handler.server_command((None, parameters, trail)).unwrap();

    let responses = handler.stream.get_responses();
    assert_eq!("SERVER servername 1 :serverinfo", responses[0]);
    assert_eq!("NICK nickname1 1", responses[1]);
    assert_eq!(
        ":nickname1 USER username 127.0.0.1 servername :realname",
        responses[2]
    );
    assert_eq!("NICK nickname2 1", responses[3]);
    assert_eq!(
        ":nickname2 USER username 127.0.0.1 servername :realname",
        responses[4]
    );
}
