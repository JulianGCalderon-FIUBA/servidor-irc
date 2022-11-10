use super::*;

#[test]
fn topic_fails_with_unregistered_client() {
    let mut handler = dummy_client_handler();

    let parameters = vec!["nick".to_string(), "#hola".to_string()];

    handler.topic_command(parameters).unwrap();

    assert_eq!(
        "200 :Unregistered\r\n",
        handler.stream.read_wbuf_to_string()
    )
}

#[test]
fn topic_fails_with_empty_params() {
    let mut handler = dummy_client_handler();
    let parameters = vec![];

    handler.topic_command(parameters).unwrap();

    assert_eq!(
        "461 TOPIC :Not enough parameters\r\n",
        handler.stream.read_wbuf_to_string()
    );
}

#[test]
fn cannot_modify_topic_if_not_in_channel() {
    let mut handler = dummy_client_handler();
    register_client(&mut handler, "nick");

    handler.database.add_client(dummy_client("dummy"));
    handler.database.add_client_to_channel("dummy", "#canal");

    let parameters = vec!["#canal".to_string(), "topic".to_string()];

    handler.topic_command(parameters).unwrap();

    assert_eq!(
        "442 #canal :You're not on that channel\r\n",
        handler.stream.read_wbuf_to_string()
    )
}
