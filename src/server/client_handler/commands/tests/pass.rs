use super::*;

#[test]
fn pass_sets_connection_password() {
    let mut handler = dummy_client_handler();

    let parameters = vec!["pass".to_string()];
    handler.pass_command(parameters).unwrap();

    let parameters = vec!["pass2".to_string()];
    handler.pass_command(parameters).unwrap();

    assert_eq!(
        "200 :success\r\n200 :success\r\n",
        handler.stream_client_handler.read_wbuf_to_string()
    );

    assert_eq!(
        handler.connection.get_attribute("password").unwrap(),
        "pass2"
    )
}

#[test]
fn pass_is_only_valid_as_first_command() {
    let mut handler = dummy_client_handler();

    let parameters = vec!["nick".to_string()];
    handler.nick_command(parameters).unwrap();

    let parameters = vec!["pass".to_string()];
    handler.pass_command(parameters).unwrap();

    assert_eq!(
        "200 :success\r\n462 :may not reregister\r\n",
        handler.stream_client_handler.read_wbuf_to_string()
    )
}
