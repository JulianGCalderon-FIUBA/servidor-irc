use super::*;

#[test]
fn oper_with_valid_credential_sets_client_as_operator() {
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

#[test]
fn oper_is_only_valid_after_registration() {
    let mut handler = dummy_client_handler();

    let parameters = vec!["admin".to_string(), "admin".to_string()];
    handler.oper_command(parameters).unwrap();

    assert_eq!(
        "200 :unregistered\r\n",
        handler.stream.read_wbuf_to_string()
    );
}
