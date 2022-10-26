use super::*;

#[test]
fn part_fails_with_unregistered_client() {
    let mut handler = dummy_client_handler();

    let parameters = vec!["sol".to_string()];
    handler.part_command(parameters).unwrap();

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

    handler.part_command(parameters).unwrap();

    assert_eq!(
        "461 PART :not enough parameters\r\n",
        handler.stream_client_handler.read_wbuf_to_string()
    );
    assert_eq!(handler.database.get_channels(), channels);
}

#[test]
fn part_fails_with_invalid_channel_name() {
    let mut handler = dummy_client_handler();
    register_client(&mut handler);

    let parameters = vec!["hola,#ho'la,#hola".to_string()];

    handler.part_command(parameters).unwrap();

    assert_eq!(
        "403 hola :no such channel\r\n403 #ho'la :no such channel\r\n403 #hola :no such channel\r\n",
        handler.stream_client_handler.read_wbuf_to_string()
    );
}

#[test]
fn part_fails_with_user_not_on_channel() {
    let mut handler = dummy_client_handler();
    register_client(&mut handler);

    let parameters = vec!["#hola".to_string()];
    handler
        .database
        .add_client_to_channel("newnickname", "#hola");

    handler.part_command(parameters).unwrap();

    assert_eq!(
        "442 #hola :you're not on that channel\r\n",
        handler.stream_client_handler.read_wbuf_to_string()
    )
}
