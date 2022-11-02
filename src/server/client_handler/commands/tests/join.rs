use super::*;

#[test]
fn join_fails_with_unregistered_client() {
    let mut handler = dummy_client_handler();

    let parameters = vec!["#sol".to_string()];
    handler.join_command(parameters).unwrap();

    assert_eq!(
        "200 :Unregistered\r\n",
        handler.stream.read_wbuf_to_string()
    )
}

#[test]
fn join_fails_with_empty_params() {
    let mut handler = dummy_client_handler();
    let parameters = vec![];

    let channels: Vec<String> = vec![];
    handler.join_command(parameters).unwrap();

    assert_eq!(
        "461 JOIN :Not enough parameters\r\n",
        handler.stream.read_wbuf_to_string()
    );
    assert_eq!(handler.database.get_channels(), channels);
}

#[test]
fn join_fails_with_invalid_channel_name() {
    let mut handler = dummy_client_handler();
    register_client(&mut handler, "nick");

    let parameters = vec!["hola,#ho'la".to_string()];

    handler.join_command(parameters).unwrap();

    let responses = handler.stream.get_responses();

    assert_eq!("403 hola :No such channel", responses[0]);
    assert_eq!("403 #ho'la :No such channel", responses[1]);
}

#[test]
fn join_fails_with_user_in_too_many_channels() {
    let mut handler = dummy_client_handler();
    register_client(&mut handler, "nick");

    let parameters =
        vec!["#uno,#dos,#tres,&cuatro,&cinco,&seis,#siete,#ocho,#nueve,&diez".to_string()];
    handler.join_command(parameters).unwrap();

    handler.stream.clear();

    let parameters2 = vec!["#once".to_string()];
    handler.join_command(parameters2).unwrap();

    assert_eq!(
        "405 #once :You have joined too many channels\r\n",
        handler.stream.read_wbuf_to_string()
    )
}

#[test]
fn join_fails_if_user_already_in_channel() {
    let mut handler = dummy_client_handler();
    register_client(&mut handler, "nick");

    let parameters = vec!["#hola".to_string()];
    handler.join_command(parameters).unwrap();

    handler.stream.clear();

    let parameters2 = vec!["#hola".to_string()];
    handler.join_command(parameters2).unwrap();

    assert_eq!(
        "443 nick #hola :Is already on channel\r\n",
        handler.stream.read_wbuf_to_string()
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

    let responses = handler.stream.get_responses();

    assert_eq!("331 #channel :No topic is set", responses[0]);
    assert_eq!("353 #channel :nick", responses[1]);
    assert_eq!(handler.database.get_channels_for_client("nick"), channels);
}

#[test]
fn can_join_multiple_channels() {
    let mut handler = dummy_client_handler();
    register_client(&mut handler, "nick");

    let parameters = vec!["#channel1,#channel2,#channel3".to_string()];
    handler.join_command(parameters).unwrap();

    let responses = handler.stream.get_responses();

    assert_eq!("331 #channel1 :No topic is set", responses[0]);
    assert_eq!("353 #channel1 :nick", responses[1]);
    assert_eq!("331 #channel2 :No topic is set", responses[2]);
    assert_eq!("353 #channel2 :nick", responses[3]);
    assert_eq!("331 #channel3 :No topic is set", responses[4]);
    assert_eq!("353 #channel3 :nick", responses[5]);

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

    handler.database.add_client_to_channel("nick2", "#channel");

    let channels = vec!["#channel".to_string()];

    handler.join_command(parameters).unwrap();

    let responses = handler.stream.get_responses();

    assert_eq!("331 #channel :No topic is set", responses[0]);
    assert_eq!("353 #channel :nick2 nick", responses[1]);

    assert_eq!(handler.database.get_channels_for_client("nick"), channels);
    assert_eq!(handler.database.get_channels_for_client("nick2"), channels);
}
