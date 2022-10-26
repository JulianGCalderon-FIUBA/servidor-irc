use super::*;

#[test]
fn join_fails_with_unregistered_client() {
    let mut handler = dummy_client_handler();

    let parameters = vec!["sol".to_string()];
    handler.join_command(parameters).unwrap();

    assert_eq!(
        "200 :unregistered\r\n",
        handler.stream_client_handler.read_wbuf_to_string()
    )
}

#[test]
fn join_with_empty_params() {
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

#[test]
fn join_fails_with_invalid_channel_name() {
    let mut handler = dummy_client_handler();
    register_client(&mut handler);

    let parameters = vec!["hola,#ho'la".to_string()];

    handler.join_command(parameters).unwrap();

    assert_eq!(
        "403 hola :no such channel\r\n403 #ho'la :no such channel\r\n",
        handler.stream_client_handler.read_wbuf_to_string()
    );
}
#[test]
fn join_fails_with_user_already_in_channel() {
    let mut handler = dummy_client_handler();
    register_client(&mut handler);

    let parameters =
        vec!["#uno,#dos,#tres,&cuatro,&cinco,&seis,#siete,#ocho,#nueve,&diez".to_string()];
    handler.join_command(parameters).unwrap();

    handler.stream_client_handler.clear();

    let parameters2 = vec!["#once".to_string()];
    handler.join_command(parameters2).unwrap();

    assert_eq!(
        "405 #once :you have joined too many channels\r\n",
        handler.stream_client_handler.read_wbuf_to_string()
    )
}
