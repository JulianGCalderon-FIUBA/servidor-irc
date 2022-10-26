use super::*;

#[test]
fn join_fails_with_unregistered_client() {
    let mut handler = dummy_client_handler();

    let parameters = vec!["#sol".to_string()];
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
    register_client(&mut handler, "nick");

    let parameters = vec!["hola,#ho'la".to_string()];

    handler.join_command(parameters).unwrap();

    assert_eq!(
        "403 hola :no such channel\r\n403 #ho'la :no such channel\r\n",
        handler.stream_client_handler.read_wbuf_to_string()
    );
}

#[test]
fn join_fails_with_user_in_too_many_channels() {
    let mut handler = dummy_client_handler();
    register_client(&mut handler, "nick");

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

#[test]
fn join_fails_if_user_already_in_channel() {
    let mut handler = dummy_client_handler();
    register_client(&mut handler, "nick");

    let parameters = vec!["#hola".to_string()];
    handler.join_command(parameters).unwrap();

    handler.stream_client_handler.clear();

    let parameters2 = vec!["#hola".to_string()];
    handler.join_command(parameters2).unwrap();

    assert_eq!(
        "443 nick #hola :is already on channel\r\n",
        handler.stream_client_handler.read_wbuf_to_string()
    )
}

#[test]
fn can_join_one_channel() {
    let mut handler = dummy_client_handler();
    register_client(&mut handler, "nick");

    let parameters = vec!["#channel".to_string()];

    assert!(handler.database.get_channels().is_empty());

    handler.join_command(parameters).unwrap();

    let channels = vec!["#channel".to_string()];

    assert_eq!(
        "331 #channel :no topic is set\r\n353 #channel :nick\r\n",
        handler.stream_client_handler.read_wbuf_to_string()
    );
    assert_eq!(handler.database.get_channels_for_client("nick"), channels);
}

#[test]
fn can_join_multiple_channels() {
    let mut handler = dummy_client_handler();
    register_client(&mut handler, "nick");

    let parameters = vec!["#channel1,#channel2,#channel3".to_string()];
    handler.join_command(parameters).unwrap();

    assert_eq!(
        "331 #channel1 :no topic is set\r\n353 #channel1 :nick\r\n331 #channel2 :no topic is set\r\n353 #channel2 :nick\r\n331 #channel3 :no topic is set\r\n353 #channel3 :nick\r\n",
        handler.stream_client_handler.read_wbuf_to_string()
    );

    let mut channels = vec![
        "#channel1".to_string(),
        "#channel2".to_string(),
        "#channel3".to_string(),
    ];
    channels.sort();
    let mut channels_for_client = handler.database.get_channels_for_client("nick");
    channels_for_client.sort();
    assert_eq!(channels_for_client, channels);
}

#[test]
fn can_join_existing_channel() {
    let mut handler = dummy_client_handler();
    register_client(&mut handler, "nick");

    let parameters = vec!["#channel".to_string()];
    handler.join_command(parameters.clone()).unwrap();

    register_client(&mut handler, "nick2");

    let channels = vec!["#channel".to_string()];

    handler.join_command(parameters).unwrap();

    assert_eq!(
        "331 #channel :no topic is set\r\n353 #channel :nick nick2\r\n",
        handler.stream_client_handler.read_wbuf_to_string()
    );
    assert_eq!(handler.database.get_channels_for_client("nick"), channels);
    assert_eq!(handler.database.get_channels_for_client("nick2"), channels);
}
