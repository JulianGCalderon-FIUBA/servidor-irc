use crate::server::testing::dummy_server;

use super::*;

#[test]
fn oper_fails_with_less_than_two_parameters() {
    let mut handler = dummy_client_handler();

    let parameters = vec![];
    handler.oper_command((None, parameters, None)).unwrap();

    assert_eq!(
        "461 OPER :Not enough parameters\r\n",
        handler.stream.read_wbuf_to_string()
    );
}

#[test]
fn oper_fails_with_incorrect_credentials() {
    let mut handler = dummy_client_handler();

    let parameters = vec!["user".to_string(), "user".to_string()];
    handler.oper_command((None, parameters, None)).unwrap();

    assert_eq!(
        "464 :Password incorrect\r\n",
        handler.stream.read_wbuf_to_string()
    );
    assert!(!handler.database.is_server_operator("nickname"));
}

#[test]
fn can_register_as_operator() {
    let mut handler = dummy_client_handler();

    let parameters = vec!["admin".to_string(), "admin".to_string()];
    handler.oper_command((None, parameters, None)).unwrap();

    assert_eq!(
        "381 :You are now an IRC operator\r\n",
        handler.stream.read_wbuf_to_string()
    );

    assert!(handler.database.is_server_operator("nickname"));
}

#[test]
fn oper_is_relayed_as_mode_to_all_servers() {
    let mut handler = dummy_client_handler();

    handler
        .database
        .add_immediate_server(dummy_server("servername2"));
    handler
        .database
        .add_immediate_server(dummy_server("servername3"));

    let parameters = vec!["admin".to_string(), "admin".to_string()];
    handler.oper_command((None, parameters, None)).unwrap();

    assert_eq!(
        ":nickname MODE nickname +o\r\n",
        handler
            .database
            .get_server_stream("servername2")
            .unwrap()
            .read_wbuf_to_string()
    );

    assert_eq!(
        ":nickname MODE nickname +o\r\n",
        handler
            .database
            .get_server_stream("servername3")
            .unwrap()
            .read_wbuf_to_string()
    );
}
