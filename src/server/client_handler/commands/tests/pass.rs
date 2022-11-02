use super::*;

#[test]
fn pass_fails_with_already_registered() {
    let mut handler = dummy_client_handler();

    let parameters = vec!["nick".to_string()];
    handler.nick_command(parameters).unwrap();

    let parameters = vec!["pass".to_string()];
    handler.pass_command(parameters).unwrap();

    let responses = handler.stream.get_responses();

    assert_eq!("200 :success", responses[0]);
    assert_eq!("462 :you may not reregister", responses[1]);
}

#[test]
fn pass_fails_with_empty_params() {
    let mut handler = dummy_client_handler();

    let parameters = vec!["nick".to_string()];
    handler.nick_command(parameters).unwrap();

    handler.stream.clear();

    let parameters = vec![];
    handler.pass_command(parameters).unwrap();

    assert_eq!(
        "461 PASS :not enough parameters\r\n",
        handler.stream.read_wbuf_to_string()
    );
}

#[test]
fn pass_sets_connection_password() {
    let mut handler = dummy_client_handler();

    let parameters = vec!["pass".to_string()];
    handler.pass_command(parameters).unwrap();

    let parameters = vec!["pass2".to_string()];
    handler.pass_command(parameters).unwrap();

    let responses = handler.stream.get_responses();

    assert_eq!("200 :success", responses[0]);
    assert_eq!("200 :success", responses[1]);

    assert_eq!(
        "pass2",
        handler.registration.get_attribute("password").unwrap()
    )
}
