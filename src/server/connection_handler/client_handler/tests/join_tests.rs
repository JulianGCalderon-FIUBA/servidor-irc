use crate::server::testing::dummy_server;

use super::*;

#[test]
fn join_fails_with_empty_params() {
    let mut handler = dummy_client_handler();
    let parameters = vec![];

    let channels: Vec<String> = vec![];
    handler.join_command((None, parameters, None)).unwrap();

    assert_eq!(
        "461 JOIN :Not enough parameters\r\n",
        handler.stream.read_wbuf_to_string()
    );
    assert_eq!(handler.database.get_all_channels(), channels);
}

#[test]
fn join_fails_with_invalid_channel_name() {
    let mut handler = dummy_client_handler();

    let parameters = vec!["hola,#ho'la".to_string()];

    handler.join_command((None, parameters, None)).unwrap();

    let responses = handler.stream.get_responses();

    assert_eq!("403 hola :No such channel", responses[0]);
    assert_eq!("403 #ho'la :No such channel", responses[1]);
}

#[test]
fn join_fails_with_user_in_too_many_channels() {
    let mut handler = dummy_client_handler();

    let parameters =
        vec!["#uno,#dos,#tres,&cuatro,&cinco,&seis,#siete,#ocho,#nueve,&diez".to_string()];
    handler.join_command((None, parameters, None)).unwrap();

    handler.stream.clear();

    let parameters = vec!["#once".to_string()];
    handler.join_command((None, parameters, None)).unwrap();

    assert_eq!(
        "405 #once :You have joined too many channels\r\n",
        handler.stream.read_wbuf_to_string()
    )
}

#[test]
fn join_fails_if_user_already_in_channel() {
    let mut handler = dummy_client_handler();

    let parameters = vec!["#hola".to_string()];
    handler.join_command((None, parameters, None)).unwrap();

    handler.stream.clear();

    let parameters = vec!["#hola".to_string()];
    handler.join_command((None, parameters, None)).unwrap();

    assert_eq!(
        "443 nickname #hola :Is already on channel\r\n",
        handler.stream.read_wbuf_to_string()
    )
}

#[test]
fn can_join_one_channel() {
    let mut handler = dummy_client_handler();

    let parameters = vec!["#channel".to_string()];

    assert!(handler.database.get_all_channels().is_empty());

    handler.join_command((None, parameters, None)).unwrap();

    let channels = vec!["#channel".to_string()];

    let responses = handler.stream.get_responses();

    assert_eq!("331 #channel :No topic is set", responses[0]);
    assert_eq!("353 #channel :nickname", responses[1]);
    assert_eq!(
        handler
            .database
            .get_channels_for_client("nickname")
            .unwrap(),
        channels
    );
}

#[test]
fn can_join_multiple_channels() {
    let mut handler = dummy_client_handler();

    let parameters = vec!["#channel1,#channel2,#channel3".to_string()];
    handler.join_command((None, parameters, None)).unwrap();

    let responses = handler.stream.get_responses();

    assert_eq!("331 #channel1 :No topic is set", responses[0]);
    assert_eq!("353 #channel1 :nickname", responses[1]);
    assert_eq!("331 #channel2 :No topic is set", responses[2]);
    assert_eq!("353 #channel2 :nickname", responses[3]);
    assert_eq!("331 #channel3 :No topic is set", responses[4]);
    assert_eq!("353 #channel3 :nickname", responses[5]);

    let mut channels = vec![
        "#channel1".to_string(),
        "#channel2".to_string(),
        "#channel3".to_string(),
    ];
    channels.sort();
    let mut channels_for_client = handler
        .database
        .get_channels_for_client("nickname")
        .unwrap();
    channels_for_client.sort();
    assert_eq!(channels_for_client, channels);
}

#[test]
fn can_join_existing_channel() {
    let mut handler = dummy_client_handler();

    let parameters = vec!["#channel".to_string()];

    handler.database.add_local_client(dummy_client("nick2"));

    handler.database.add_client_to_channel("#channel", "nick2");

    let channels = vec!["#channel".to_string()];

    handler.join_command((None, parameters, None)).unwrap();

    let responses = handler.stream.get_responses();

    assert_eq!("331 #channel :No topic is set", responses[0]);
    assert_eq!("353 #channel :nick2 nickname", responses[1]);

    assert_eq!(
        handler
            .database
            .get_channels_for_client("nickname")
            .unwrap(),
        channels
    );
    assert_eq!(
        handler.database.get_channels_for_client("nick2").unwrap(),
        channels
    );
}

#[test]
fn can_join_channel_with_topic() {
    let mut handler = dummy_client_handler();

    let parameters = vec!["#channel".to_string()];

    handler.database.add_local_client(dummy_client("nick2"));
    handler.database.add_client_to_channel("#channel", "nick2");
    handler
        .database
        .set_channel_topic("#channel", "topic for channel");

    let channels = vec!["#channel".to_string()];

    handler.join_command((None, parameters, None)).unwrap();

    let responses = handler.stream.get_responses();

    assert_eq!("332 #channel :topic for channel", responses[0]);
    assert_eq!("353 #channel :nick2 nickname", responses[1]);

    assert_eq!(
        handler
            .database
            .get_channels_for_client("nickname")
            .unwrap(),
        channels
    );
    assert_eq!(
        handler.database.get_channels_for_client("nick2").unwrap(),
        channels
    );
}

#[test]
fn join_notifies_users_in_channel() {
    let mut handler = dummy_client_handler();

    handler.database.add_local_client(dummy_client("nick2"));
    handler.database.add_client_to_channel("#channel", "nick2");

    let parameters = vec!["#channel".to_string()];
    handler.join_command((None, parameters, None)).unwrap();

    assert_eq!(
        ":nickname JOIN #channel\r\n",
        handler
            .database
            .get_local_stream("nick2")
            .unwrap()
            .read_wbuf_to_string()
    );
}

#[test]
fn join_fails_with_incorrect_key() {
    let mut handler = dummy_client_handler();

    handler.database.add_local_client(dummy_client("nick2"));
    handler.database.add_client_to_channel("#hola", "nick2");

    handler
        .database
        .set_channel_key("#hola", Some("key".to_string()));

    let parameters = vec!["#hola".to_string(), "wrong_key".to_string()];
    handler.join_command((None, parameters, None)).unwrap();

    assert_eq!(
        "475 #hola :Cannot join channel (+k)\r\n",
        handler.stream.read_wbuf_to_string()
    );

    assert!(!handler.database.is_client_in_channel("#hola", "nickname"))
}

#[test]
fn can_join_channel_with_key() {
    let mut handler = dummy_client_handler();

    handler.database.add_local_client(dummy_client("nick2"));
    handler.database.add_client_to_channel("#hola", "nick2");

    handler
        .database
        .set_channel_key("#hola", Some("key".to_string()));

    let parameters = vec!["#hola".to_string(), "key".to_string()];
    handler.join_command((None, parameters, None)).unwrap();

    let responses = handler.stream.get_responses();

    assert_eq!("331 #hola :No topic is set", responses[0]);
    assert_eq!("353 #hola :nick2 nickname", responses[1]);

    assert!(handler.database.is_client_in_channel("#hola", "nickname"))
}

#[test]
fn can_join_multiple_channels_with_keys() {
    let mut handler = dummy_client_handler();

    handler.database.add_local_client(dummy_client("nick2"));
    handler.database.add_local_client(dummy_client("nick3"));

    handler.database.add_client_to_channel("#channel1", "nick2");
    handler.database.add_client_to_channel("#channel2", "nick3");

    handler
        .database
        .set_channel_key("#channel1", Some("key1".to_string()));
    handler
        .database
        .set_channel_key("#channel2", Some("key2".to_string()));

    let parameters = vec![
        "#channel1,#channel2,#channel3".to_string(),
        "key1,key2".to_string(),
    ];
    handler.join_command((None, parameters, None)).unwrap();

    let responses = handler.stream.get_responses();

    assert_eq!("331 #channel1 :No topic is set", responses[0]);
    assert_eq!("353 #channel1 :nick2 nickname", responses[1]);
    assert_eq!("331 #channel2 :No topic is set", responses[2]);
    assert_eq!("353 #channel2 :nick3 nickname", responses[3]);
    assert_eq!("331 #channel3 :No topic is set", responses[4]);
    assert_eq!("353 #channel3 :nickname", responses[5]);

    assert!(handler
        .database
        .is_client_in_channel("#channel1", "nickname"));
    assert!(handler
        .database
        .is_client_in_channel("#channel2", "nickname"));
    assert!(handler
        .database
        .is_client_in_channel("#channel3", "nickname"))
}

#[test]
fn join_fails_with_user_limit_reached_on_limited_channel() {
    let mut handler = dummy_client_handler();

    handler.database.add_local_client(dummy_client("nick2"));
    handler.database.add_client_to_channel("#hola", "nick2");

    handler.database.set_channel_limit("#hola", Some(1));

    let parameters = vec!["#hola".to_string()];
    handler.join_command((None, parameters, None)).unwrap();

    assert_eq!(
        "471 #hola :Cannot join channel (+l)\r\n",
        handler.stream.read_wbuf_to_string()
    );

    assert!(!handler.database.is_client_in_channel("#hola", "nickname"))
}

#[test]
fn can_join_limited_channel_if_limit_not_reached() {
    let mut handler = dummy_client_handler();

    handler.database.add_local_client(dummy_client("nick2"));
    handler.database.add_client_to_channel("#hola", "nick2");

    handler.database.set_channel_limit("#hola", Some(4));

    let parameters = vec!["#hola".to_string()];
    handler.join_command((None, parameters, None)).unwrap();

    let responses = handler.stream.get_responses();

    assert_eq!("331 #hola :No topic is set", responses[0]);
    assert_eq!("353 #hola :nick2 nickname", responses[1]);

    assert!(handler.database.is_client_in_channel("#hola", "nickname"))
}

#[test]
fn join_fails_with_banmask() {
    let mut handler = dummy_client_handler();

    handler.database.add_local_client(dummy_client("user2"));
    handler.database.add_client_to_channel("#channel", "user2");

    handler.database.add_channel_banmask("#channel", "nickname");

    let parameters = vec!["#channel".to_string()];
    handler.join_command((None, parameters, None)).unwrap();

    assert_eq!(
        "474 #channel :Cannot join channel (+b)\r\n",
        handler.stream.read_wbuf_to_string()
    );

    assert!(!handler
        .database
        .is_client_in_channel("#channel", "nickname"))
}

#[test]
fn can_join_channel_with_banmask() {
    let mut handler = dummy_client_handler();

    handler.database.add_local_client(dummy_client("nick2"));
    handler.database.add_client_to_channel("#channel", "nick2");

    handler.database.add_channel_banmask("#channel", "user");

    let parameters = vec!["#channel".to_string()];
    handler.join_command((None, parameters, None)).unwrap();

    let responses = handler.stream.get_responses();

    assert_eq!("331 #channel :No topic is set", responses[0]);
    assert_eq!("353 #channel :nick2 nickname", responses[1]);

    assert!(handler
        .database
        .is_client_in_channel("#channel", "nickname"))
}

#[test]
fn joins_notifies_user_in_channel() {
    let mut handler = dummy_client_handler();

    handler.database.add_local_client(dummy_client("nick2"));
    handler.database.add_client_to_channel("#channel", "nick2");

    let parameters = vec!["#channel".to_string()];
    handler.join_command((None, parameters, None)).unwrap();

    assert_eq!(
        ":nickname JOIN #channel\r\n",
        handler
            .database
            .get_local_stream("nick2")
            .unwrap()
            .read_wbuf_to_string()
    );
}

#[test]
fn distributed_channels_joins_are_relayed_to_all_servers() {
    let mut handler = dummy_client_handler();

    handler
        .database()
        .add_immediate_server(dummy_server("servername1"));
    handler
        .database()
        .add_immediate_server(dummy_server("servername2"));

    let parameters = vec!["#channel".to_string()];
    handler.join_command((None, parameters, None)).unwrap();

    assert_eq!(
        ":nickname JOIN #channel\r\n:nickname MODE #channel +o nickname\r\n",
        handler
            .database
            .get_server_stream("servername1")
            .unwrap()
            .read_wbuf_to_string()
    );
    assert_eq!(
        ":nickname JOIN #channel\r\n:nickname MODE #channel +o nickname\r\n",
        handler
            .database
            .get_server_stream("servername2")
            .unwrap()
            .read_wbuf_to_string()
    );
}

#[test]
fn join_fails_with_invite_only_channel() {
    let mut handler = dummy_client_handler();

    handler.database.add_local_client(dummy_client("nick2"));
    handler.database.add_client_to_channel("#hola", "nick2");

    handler
        .database
        .set_channel_flag("#hola", ChannelFlag::InviteOnly);

    let parameters = vec!["#hola".to_string()];
    handler.join_command((None, parameters, None)).unwrap();

    assert_eq!(
        "473 #hola :Cannot join channel (+i)\r\n",
        handler.stream.read_wbuf_to_string()
    );

    assert!(!handler.database.is_client_in_channel("#hola", "nickname"))
}

#[test]
fn can_join_invite_only_channel_with_invite() {
    let mut handler = dummy_client_handler();

    handler.database.add_local_client(dummy_client("nick2"));
    handler.database.add_client_to_channel("#hola", "nick2");

    handler
        .database
        .set_channel_flag("#hola", ChannelFlag::InviteOnly);

    handler.database.add_channel_invite("#hola", "nickname");

    let parameters = vec!["#hola".to_string()];
    handler.join_command((None, parameters, None)).unwrap();

    let responses = handler.stream.get_responses();

    assert_eq!("331 #hola :No topic is set", responses[0]);
    assert_eq!("353 #hola :nick2 nickname", responses[1]);

    assert!(handler.database.is_client_in_channel("#hola", "nickname"))
}
