use super::*;

#[test]
fn part_fails_with_unregistered_client() {
    let mut handler = dummy_client_handler();

    let parameters = vec!["sol".to_string()];
    handler.join_command(parameters).unwrap();

    assert_eq!(
        "200 :unregistered\r\n",
        handler.stream_client_handler.read_wbuf_to_string()
    )
}

#[test]
fn part_with_empty_params() {
    let mut handler = dummy_client_handler();
    let parameters = vec![];
    let channels: Vec<String> = vec![];

    handler.join_command(parameters).unwrap();

    assert_eq!(
        "461 JOIN :not enough parameters\r\n",
        handler.stream_client_handler.read_wbuf_to_string()
    );
    assert_eq!(handler.database.get_channels(), channels);
}
