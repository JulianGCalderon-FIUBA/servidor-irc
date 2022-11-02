use super::*;

#[test]
fn oper_fails_with_less_than_two_parameters() {
    let mut handler = dummy_client_handler();

    let parameters = vec![];
    handler.oper_command(parameters).unwrap();

    assert_eq!(
        "461 OPER :Not enough parameters\r\n",
        handler.stream.read_wbuf_to_string()
    );
}

#[test]
fn oper_fails_with_unregistered_client() {
    let mut handler = dummy_client_handler();

    let parameters = vec!["admin".to_string(), "admin".to_string()];
    handler.oper_command(parameters).unwrap();

    assert_eq!(
        "200 :Unregistered\r\n",
        handler.stream.read_wbuf_to_string()
    );
}

#[test]
fn oper_fails_with_incorrect_credentials() {
    let mut handler = dummy_client_handler();
    register_client(&mut handler, "nick");

    let parameters = vec!["user".to_string(), "user".to_string()];
    handler.oper_command(parameters).unwrap();

    assert_eq!(
        "464 :Password incorrect\r\n",
        handler.stream.read_wbuf_to_string()
    );
}

#[test]
fn can_register_as_operator() {
    let mut handler = dummy_client_handler();
    register_client(&mut handler, "nick");

    let parameters = vec!["admin".to_string(), "admin".to_string()];
    handler.oper_command(parameters).unwrap();

    assert_eq!(
        "381 :You are now an IRC operator\r\n",
        handler.stream.read_wbuf_to_string()
    );

    assert!(handler.database.is_server_operator("nick"));
}
