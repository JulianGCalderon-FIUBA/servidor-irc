use super::*;

#[test]
fn away_fails_with_unregistered_client() {
    let mut handler = dummy_registration_handler();

    let trailing = None;
    handler.away_command(trailing).unwrap();

    assert_eq!(
        "451 :You have not registered\r\n",
        handler.stream.read_wbuf_to_string()
    )
}

#[test]
fn invite_fails_with_unregistered_client() {
    let mut handler = dummy_registration_handler();

    let parameters = vec![];
    handler.invite_command(parameters).unwrap();

    assert_eq!(
        "451 :You have not registered\r\n",
        handler.stream.read_wbuf_to_string()
    )
}

#[test]
fn join_fails_with_unregistered_client() {
    let mut handler = dummy_registration_handler();

    let parameters = vec![];
    handler.join_command(parameters).unwrap();

    assert_eq!(
        "451 :You have not registered\r\n",
        handler.stream.read_wbuf_to_string()
    )
}

#[test]
fn kick_fails_with_unregistered_client() {
    let mut handler = dummy_registration_handler();

    let parameters = vec![];
    let trailing = None;
    handler.kick_command(parameters, trailing).unwrap();

    assert_eq!(
        "451 :You have not registered\r\n",
        handler.stream.read_wbuf_to_string()
    )
}

#[test]
fn list_fails_with_unregistered_client() {
    let mut handler = dummy_registration_handler();

    let parameters = vec![];
    handler.list_command(parameters).unwrap();

    assert_eq!(
        "451 :You have not registered\r\n",
        handler.stream.read_wbuf_to_string()
    )
}

#[test]
fn mode_fails_with_unregistered_client() {
    let mut handler = dummy_registration_handler();

    let parameters = vec![];
    handler.mode_command(parameters).unwrap();

    assert_eq!(
        "451 :You have not registered\r\n",
        handler.stream.read_wbuf_to_string()
    )
}

#[test]
fn names_fails_with_unregistered_client() {
    let mut handler = dummy_registration_handler();

    let parameters = vec![];
    handler.names_command(parameters).unwrap();

    assert_eq!(
        "451 :You have not registered\r\n",
        handler.stream.read_wbuf_to_string()
    )
}

#[test]
fn notice_fails_with_unregistered_client() {
    let mut handler = dummy_registration_handler();

    let parameters = vec![];
    let trailing = None;
    handler.notice_command(parameters, trailing).unwrap();

    assert_eq!(
        "451 :You have not registered\r\n",
        handler.stream.read_wbuf_to_string()
    )
}

#[test]
fn oper_fails_with_unregistered_client() {
    let mut handler = dummy_registration_handler();

    let parameters = vec![];
    handler.oper_command(parameters).unwrap();

    assert_eq!(
        "451 :You have not registered\r\n",
        handler.stream.read_wbuf_to_string()
    )
}

#[test]
fn part_fails_with_unregistered_client() {
    let mut handler = dummy_registration_handler();

    let parameters = vec![];
    handler.part_command(parameters).unwrap();

    assert_eq!(
        "451 :You have not registered\r\n",
        handler.stream.read_wbuf_to_string()
    )
}

#[test]
fn privmsg_fails_with_unregistered_client() {
    let mut handler = dummy_registration_handler();

    let parameters = vec![];
    let trailing = None;
    handler.privmsg_command(parameters, trailing).unwrap();

    assert_eq!(
        "451 :You have not registered\r\n",
        handler.stream.read_wbuf_to_string()
    )
}

#[test]
fn topic_fails_with_unregistered_client() {
    let mut handler = dummy_registration_handler();

    let parameters = vec![];
    handler.topic_command(parameters).unwrap();

    assert_eq!(
        "451 :You have not registered\r\n",
        handler.stream.read_wbuf_to_string()
    )
}

#[test]
fn who_fails_with_unregistered_client() {
    let mut handler = dummy_registration_handler();

    let parameters = vec![];
    handler.who_command(parameters).unwrap();

    assert_eq!(
        "451 :You have not registered\r\n",
        handler.stream.read_wbuf_to_string()
    )
}

#[test]
fn who_is_fails_with_unregistered_client() {
    let mut handler = dummy_registration_handler();

    let parameters = vec![];
    handler.whois_command(parameters).unwrap();

    assert_eq!(
        "451 :You have not registered\r\n",
        handler.stream.read_wbuf_to_string()
    )
}
