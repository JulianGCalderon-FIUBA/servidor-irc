use crate::server::{
    consts::modes::ChannelFlag,
    testing::{dummy_client, dummy_database},
};

#[test]
fn can_add_client() {
    let database = dummy_database();

    assert!(!database.contains_client("nickname"));
    database.add_client(dummy_client("nickname"));
    assert!(database.contains_client("nickname"));

    assert!(!database.contains_client("nickname2"));
    database.add_client(dummy_client("nickname2"));
    assert!(database.contains_client("nickname2"));
}

#[test]
fn can_set_server_operator() {
    let database = dummy_database();

    database.add_client(dummy_client("nickname"));

    assert!(!database.is_server_operator("nickname"));
    database.set_server_operator("nickname");
    assert!(database.is_server_operator("nickname"));
}

#[test]
fn can_get_client_stream() {
    let database = dummy_database();

    let client = dummy_client("nickname");

    let stream_ref_expected = client.get_stream().unwrap().unwrap();

    database.add_client(client);
    let stream_ref_actual = database.get_stream("nickname").unwrap().unwrap();

    assert_eq!(stream_ref_expected, stream_ref_actual);
}

#[test]
fn disconnecting_sets_client_not_online() {
    let database = dummy_database();

    let client = dummy_client("nickname");

    database.add_client(client);

    assert!(database.get_stream("nickname").is_some());
    database.disconnect_client("nickname");
    assert!(database.get_stream("nickname").is_none());
}

#[test]
fn can_add_client_to_channel() {
    let database = dummy_database();

    let client = dummy_client("nickname");
    database.add_client(client);

    assert!(!database.contains_channel("channel"));
    database.add_client_to_channel("nickname", "channel");
    assert!(database.contains_channel("channel"));
}

#[test]
fn after_adding_client_to_channel_it_contains_client() {
    let database = dummy_database();

    let client = dummy_client("nickname");
    database.add_client(client);
    database.add_client_to_channel("nickname", "channel");
    assert!(database.contains_channel("channel"));

    assert!(database.is_client_in_channel("nickname", "channel"));
}

#[test]
fn can_get_all_clients_from_channel() {
    let database = dummy_database();

    let client = dummy_client("nickname1");
    database.add_client(client);

    let client = dummy_client("nickname2");
    database.add_client(client);

    database.add_client_to_channel("nickname1", "channel");
    database.add_client_to_channel("nickname2", "channel");

    let mut value = database.get_clients_for_channel("channel");
    let expected = vec!["nickname1".to_string(), "nickname2".to_string()];
    value.sort();

    assert_eq!(value, expected)
}

#[test]
fn can_remove_client_from_channel() {
    let database = dummy_database();

    let client = dummy_client("nickname1");
    database.add_client(client);

    let client = dummy_client("nickname2");
    database.add_client(client);

    database.add_client_to_channel("nickname1", "channel");
    database.add_client_to_channel("nickname2", "channel");
    database.remove_client_from_channel("nickname1", "channel");

    let value = database.get_clients_for_channel("channel");
    let expected = vec!["nickname2".to_string()];

    assert_eq!(value, expected);
}

#[test]
fn removing_last_client_from_channel_deletes_channel() {
    let database = dummy_database();

    let client = dummy_client("nickname1");
    database.add_client(client);

    database.add_client_to_channel("nickname1", "channel");
    database.remove_client_from_channel("nickname1", "channel");

    assert!(!database.contains_channel("channel"));
}

#[test]
fn can_get_all_channels() {
    let database = dummy_database();

    let client = dummy_client("nickname");
    database.add_client(client);
    database.add_client_to_channel("nickname", "channel1");
    database.add_client_to_channel("nickname", "channel2");

    let mut channels_real = database.get_all_channels();
    let channels_expected = vec!["channel1".to_string(), "channel2".to_string()];

    channels_real.sort();

    assert_eq!(channels_real, channels_expected);
}

#[test]
fn can_get_all_channels_from_client() {
    let database = dummy_database();

    let client = dummy_client("nickname");
    database.add_client(client);
    database.add_client_to_channel("nickname", "channel1");
    database.add_client_to_channel("nickname", "channel2");

    let mut channels_real = database.get_channels_for_client("nickname");
    let channels_expected = vec!["channel1".to_string(), "channel2".to_string()];

    channels_real.sort();

    assert_eq!(channels_expected, channels_real);
}

// #[test]
// fn can_get_all_clients_matching_mask() {
//     let database = dummy_database();

//     let client = ClientBuilder::new()
//         .nickname("nickAname".to_string())
//         .username("userBname".to_string())
//         .hostname("hostCname".to_string())
//         .servername("serverDname".to_string())
//         .realname("realEname".to_string())
//         .stream(MockTcpStream::new())
//         .build()
//         .unwrap();

//     let clientinfo = client.get_info();

//     database.add_client(client);

//     database.add_client(dummy_client("othernick"));

//     let expected = vec![clientinfo];

//     assert_eq!(database.get_clients_for_mask("*A*"), expected);
//     assert_eq!(database.get_clients_for_mask("*B*"), expected);
//     assert_eq!(database.get_clients_for_mask("*C*"), expected);
//     assert_eq!(database.get_clients_for_mask("*D*"), expected);
//     assert_eq!(database.get_clients_for_mask("*E*"), expected);
// }

#[test]
fn can_get_all_clients_matching_nickmask() {
    let database = dummy_database();

    let client1 = dummy_client("nick1");
    let client2 = dummy_client("nick2");
    let client3 = dummy_client("nick3");

    let client1_info = client1.get_info();
    let client2_info = client2.get_info();
    let client3_info = client3.get_info();

    database.add_client(client1);
    database.add_client(client2);
    database.add_client(client3);

    let expected = vec![client1_info, client2_info, client3_info];
    let mut actual1 = database.get_clients_for_nickmask("nick*");
    let mut actual2 = database.get_clients_for_nickmask("*ni*");

    actual1.sort();
    actual2.sort();

    assert_eq!(actual1, expected);
    assert_eq!(actual2, expected);
}

#[test]
fn can_get_all_clients() {
    let database = dummy_database();

    let client1 = dummy_client("nick1");
    let client2 = dummy_client("nick2");

    let clientinfo1 = client1.get_info();
    let clientinfo2 = client2.get_info();

    database.add_client(client1);
    database.add_client(client2);

    let expected = vec![clientinfo1, clientinfo2];
    let mut real = database.get_all_clients();
    real.sort();

    assert_eq!(real, expected);
}

#[test]
fn can_update_nickname() {
    let database = dummy_database();

    let client = dummy_client("nick");

    database.add_client(client);

    database.update_nickname("nick", "new_nick");

    assert!(database.contains_client("new_nick"));
    assert!(!database.contains_client("nick"));
}

#[test]
fn can_set_and_get_channel_topic() {
    let database = dummy_database();

    let client = dummy_client("nick");
    database.add_client(client);
    database.add_client_to_channel("nick", "#channel");

    assert_eq!(database.get_topic_for_channel("#channel"), None);

    database.set_channel_topic("#channel", "topic");

    assert_eq!(
        database.get_topic_for_channel("#channel"),
        Some("topic".to_string())
    );

    database.set_channel_topic("#channel", "new topic");

    assert_eq!(
        database.get_topic_for_channel("#channel"),
        Some("new topic".to_string())
    );
}

#[test]
fn can_verify_channel_operator() {
    let database = dummy_database();

    database.add_client(dummy_client("nickname1"));
    database.add_client(dummy_client("nickname2"));
    database.add_client_to_channel("nickname1", "#channel");
    database.add_client_to_channel("nickname2", "#channel");

    assert!(database.is_channel_operator("#channel", "nickname1"));
    assert!(!database.is_channel_operator("#channel", "nickname2"));
}

#[test]
fn can_set_away_message_for_client() {
    let database = dummy_database();

    let client = dummy_client("nick");
    database.add_client(client);

    database.set_away_message(&Some("away".to_string()), "nick");
    assert_eq!(Some("away".to_string()), database.get_away_message("nick"));
}

#[test]
fn can_set_and_get_channel_key() {
    let database = dummy_database();

    let client = dummy_client("nick");
    database.add_client(client);
    database.add_client_to_channel("nick", "#channel");

    assert_eq!(database.get_channel_key("#channel"), None);

    database.set_channel_key("#channel", Some("key".to_string()));

    assert_eq!(
        database.get_channel_key("#channel"),
        Some("key".to_string())
    );

    database.set_channel_key("#channel", None);

    assert_eq!(database.get_channel_key("#channel"), None);
}

#[test]
fn can_set_and_unset_channel_mode() {
    let database = dummy_database();

    let client = dummy_client("nick");
    database.add_client(client);
    database.add_client_to_channel("nick", "#channel");

    assert!(!database.channel_has_mode("#channel", &ChannelFlag::Private));

    database.set_channel_mode("#channel", ChannelFlag::Private);

    assert!(database.channel_has_mode("#channel", &ChannelFlag::Private));

    database.unset_channel_mode("#channel", ChannelFlag::Private);

    assert!(!database.channel_has_mode("#channel", &ChannelFlag::Private));
}

#[test]
fn can_set_and_get_channel_limit() {
    let database = dummy_database();

    let client = dummy_client("nick");
    database.add_client(client);
    database.add_client_to_channel("nick", "#channel");

    assert_eq!(database.get_channel_limit("#channel"), None);

    database.set_channel_limit("#channel", Some(4));

    assert_eq!(database.get_channel_limit("#channel"), Some(4));

    database.set_channel_limit("#channel", None);

    assert_eq!(database.get_channel_limit("#channel"), None);
}

// #[test]
// fn can_add_and_remove_channel_operator() {
//     let database = dummy_database();

//     let client = dummy_client("nick");
//     database.add_client(client);
//     database.add_client_to_channel("nick", "#channel");

//     assert!(!database.is_channel_operator("#channel", "nick"));

//     database.add_channop("#channel", "nick");

//      assert!(database.is_channel_operator("#channel", "nick"));

//     database.remove_channop("#channel", "nick");

//      assert!(!database.is_channel_operator("#channel", "nick"));
// }

#[test]
fn can_add_and_remove_channel_speaker() {
    let database = dummy_database();

    let client = dummy_client("nick");
    database.add_client(client);
    database.add_client_to_channel("nick", "#channel");

    assert!(!database.is_channel_speaker("#channel", "nick"));

    database.add_speaker("#channel", "nick");

    assert!(database.is_channel_speaker("#channel", "nick"));

    database.remove_speaker("#channel", "nick");

    assert!(!database.is_channel_speaker("#channel", "nick"));
}

#[test]
fn can_set_and_unset_channel_banmask() {
    let database = dummy_database();

    let client = dummy_client("nick");
    database.add_client(client);
    database.add_client_to_channel("nick", "#channel");

    let mut banmasks: Vec<String> = vec![];

    assert_eq!(database.get_channel_banmask("#channel"), banmasks);

    database.add_channel_banmask("#channel", "banmask");

    banmasks.push("banmask".to_string());

    assert_eq!(database.get_channel_banmask("#channel"), banmasks);

    database.remove_channel_banmask("#channel", "banmask");

    banmasks.pop();

    assert_eq!(database.get_channel_banmask("#channel"), banmasks);
}
