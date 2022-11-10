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

    let channels: Vec<String> = vec![];
    handler.topic_command(parameters).unwrap();

    assert_eq!(
        "461 TOPIC :Not enough parameters\r\n",
        handler.stream.read_wbuf_to_string()
    );
    assert_eq!(handler.database.get_all_channels(), channels);
}
