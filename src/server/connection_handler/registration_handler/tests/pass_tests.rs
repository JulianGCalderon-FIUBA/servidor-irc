use super::*;

#[test]
fn pass_fails_with_empty_params() {
    let mut handler = dummy_registration_handler();

    let parameters = vec![];
    handler.pass_command(parameters).unwrap();

    assert_eq!(
        "461 PASS :Not enough parameters\r\n",
        handler.stream.read_wbuf_to_string()
    );
}

#[test]
fn pass_sets_connection_password() {
    let mut handler = dummy_registration_handler();

    let parameters = vec!["pass".to_string()];
    handler.pass_command(parameters).unwrap();

    let parameters = vec!["pass2".to_string()];
    handler.pass_command(parameters).unwrap();

    assert_eq!("pass2", handler.attributes.get("password").unwrap())
}

#[test]
fn pass_fails_after_nick() {
    let mut handler = dummy_registration_handler();

    let parameters = vec!["nick".to_string()];
    handler.nick_command(parameters).unwrap();

    let parameters = vec!["pass".to_string()];
    handler.pass_command(parameters).unwrap();

    let responses = handler.stream.get_responses();
    assert_eq!("462 :You may not reregister\r\n", responses[0]);
}
