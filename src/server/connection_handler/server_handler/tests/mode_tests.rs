use crate::server::{
    connection_handler::{
        connection_handler_trait::ConnectionHandlerCommands,
        server_handler::tests::dummy_server_handler,
    },
    consts::modes::ChannelFlag,
    testing::{dummy_client, dummy_external_client, dummy_server},
};

#[test]
fn mode_is_relayed_to_all_other_servers() {
    let mut handler = dummy_server_handler();
    handler
        .database
        .add_external_client(dummy_external_client("nickname1", "servername1"));
    handler
        .database
        .add_immediate_server(dummy_server("servername2"));
    handler
        .database
        .add_immediate_server(dummy_server("servername3"));

    let prefix = Some("sender".to_string());
    let parameters = vec![
        "#channel".to_string(),
        "+b".to_string(),
        "banmask".to_string(),
    ];
    handler.mode_command((prefix, parameters, None)).unwrap();

    assert_eq!(
        ":sender MODE #channel +b banmask\r\n",
        handler
            .database
            .get_server_stream("servername2")
            .unwrap()
            .read_wbuf_to_string()
    );
    assert_eq!(
        ":sender MODE #channel +b banmask\r\n",
        handler
            .database
            .get_server_stream("servername3")
            .unwrap()
            .read_wbuf_to_string()
    );
}

#[test]
fn mode_is_never_relayed_to_sending_server() {
    let mut handler = dummy_server_handler();
    handler
        .database
        .add_external_client(dummy_external_client("nickname1", "servername1"));

    let parameters = vec!["#channel".to_string(), "+p".to_string()];
    let prefix = Some("sender".to_string());
    handler.mode_command((prefix, parameters, None)).unwrap();

    assert_eq!("", handler.stream.read_wbuf_to_string());
}

#[test]
fn mode_is_relayed_to_local_clients_on_channel() {
    let mut handler = dummy_server_handler();
    handler
        .database
        .add_external_client(dummy_external_client("nickname1", "servername1"));
    handler.database.add_local_client(dummy_client("nickname2"));
    handler.database.add_local_client(dummy_client("nickname3"));

    handler
        .database
        .add_client_to_channel("#channel", "nickname2");

    let parameters = vec!["#channel".to_string(), "+s".to_string()];
    let prefix = Some("sender".to_string());
    handler.mode_command((prefix, parameters, None)).unwrap();

    assert_eq!(
        ":sender MODE #channel +s\r\n",
        handler
            .database
            .get_local_stream("nickname2")
            .unwrap()
            .read_wbuf_to_string()
    );
    assert_eq!(
        "",
        handler
            .database
            .get_local_stream("nickname3")
            .unwrap()
            .read_wbuf_to_string()
    );
}

#[test]
fn mode_adds_channop() {
    let mut handler = dummy_server_handler();
    handler.database.add_local_client(dummy_client("nickname"));
    handler
        .database
        .add_client_to_channel("#channel", "nickname");

    handler.database.add_local_client(dummy_client("nick2"));
    handler.database.add_client_to_channel("#channel", "nick2");

    assert!(!handler.database.is_channel_operator("#channel", "nick2"));

    let parameters = vec![
        "#channel".to_string(),
        "+o".to_string(),
        "nick2".to_string(),
    ];
    let prefix = Some("sender".to_string());
    handler.mode_command((prefix, parameters, None)).unwrap();
    assert!(handler.database.is_channel_operator("#channel", "nick2"));
}

#[test]
fn mode_removes_channop() {
    let mut handler = dummy_server_handler();
    handler.database.add_local_client(dummy_client("nickname"));
    handler
        .database
        .add_client_to_channel("#channel", "nickname");

    handler.database.add_local_client(dummy_client("nick2"));
    handler.database.add_client_to_channel("#channel", "nick2");
    handler.database.add_channel_operator("#channel", "nick2");

    assert!(handler.database.is_channel_operator("#channel", "nick2"));

    let parameters = vec![
        "#channel".to_string(),
        "-o".to_string(),
        "nick2".to_string(),
    ];
    let prefix = Some("sender".to_string());
    handler.mode_command((prefix, parameters, None)).unwrap();

    assert!(handler.database.is_channel_operator("#channel", "nickname"));
    assert!(!handler.database.is_channel_operator("#channel", "nick2"));
}

#[test]
fn mode_sets_limit_to_channel() {
    let mut handler = dummy_server_handler();
    handler.database.add_local_client(dummy_client("nickname"));

    handler
        .database
        .add_client_to_channel("#channel", "nickname");

    assert!(handler
        .database
        .get_channel_limit("#channel")
        .unwrap()
        .is_none());

    let parameters = vec!["#channel".to_string(), "+l".to_string(), "5".to_string()];
    let prefix = Some("sender".to_string());
    handler.mode_command((prefix, parameters, None)).unwrap();

    assert_eq!(
        handler.database.get_channel_limit("#channel").unwrap(),
        Some(5)
    );
}

#[test]
fn mode_unsets_channel_limit() {
    let mut handler = dummy_server_handler();
    handler.database.add_local_client(dummy_client("nickname"));

    handler
        .database
        .add_client_to_channel("#channel", "nickname");
    handler.database.set_channel_limit("#channel", Some(5));

    assert_eq!(
        handler.database.get_channel_limit("#channel").unwrap(),
        Some(5)
    );

    let parameters = vec!["#channel".to_string(), "-l".to_string()];
    let prefix = Some("sender".to_string());
    handler.mode_command((prefix, parameters, None)).unwrap();

    assert!(handler
        .database
        .get_channel_limit("#channel")
        .unwrap()
        .is_none());
}

#[test]
fn mode_sets_banmask() {
    let mut handler = dummy_server_handler();
    handler.database.add_local_client(dummy_client("nickname"));

    handler
        .database
        .add_client_to_channel("#channel", "nickname");

    assert!(handler
        .database
        .get_channel_banmask("#channel")
        .unwrap()
        .is_empty());

    let parameters = vec![
        "#channel".to_string(),
        "+b".to_string(),
        "banmask".to_string(),
    ];
    let prefix = Some("sender".to_string());
    handler.mode_command((prefix, parameters, None)).unwrap();

    let masks = vec!["banmask".to_string()];

    assert_eq!(
        masks,
        handler.database.get_channel_banmask("#channel").unwrap()
    )
}

#[test]
fn mode_unsets_banmask() {
    let mut handler = dummy_server_handler();
    handler.database.add_local_client(dummy_client("nickname"));

    handler
        .database
        .add_client_to_channel("#channel", "nickname");

    handler.database.add_channel_banmask("#channel", "banmask");
    handler.database.add_channel_banmask("#channel", "banmask2");
    assert!(!handler
        .database
        .get_channel_banmask("#channel")
        .unwrap()
        .is_empty());

    let parameters = vec![
        "#channel".to_string(),
        "-b".to_string(),
        "banmask".to_string(),
    ];
    let prefix = Some("sender".to_string());
    handler.mode_command((prefix, parameters, None)).unwrap();

    let masks = vec!["banmask2".to_string()];

    assert_eq!(
        masks,
        handler.database.get_channel_banmask("#channel").unwrap()
    );

    let parameters = vec![
        "#channel".to_string(),
        "-b".to_string(),
        "banmask2".to_string(),
    ];
    let prefix = Some("sender".to_string());
    handler.mode_command((prefix, parameters, None)).unwrap();

    assert!(handler
        .database
        .get_channel_banmask("#channel")
        .unwrap()
        .is_empty())
}

#[test]
fn mode_adds_speaker_to_channel() {
    let mut handler = dummy_server_handler();
    handler.database.add_local_client(dummy_client("nickname"));

    handler
        .database
        .add_client_to_channel("#channel", "nickname");

    handler.database.add_local_client(dummy_client("nick2"));
    handler.database.add_client_to_channel("#channel", "nick2");

    assert!(!handler.database.is_channel_speaker("#channel", "nick2"));

    let parameters = vec![
        "#channel".to_string(),
        "+v".to_string(),
        "nick2".to_string(),
    ];
    let prefix = Some("sender".to_string());
    handler.mode_command((prefix, parameters, None)).unwrap();

    assert!(handler.database.is_channel_speaker("#channel", "nick2"));
}

#[test]
fn mode_removes_speakers_from_channel() {
    let mut handler = dummy_server_handler();
    handler.database.add_local_client(dummy_client("nickname"));

    handler
        .database
        .add_client_to_channel("#channel", "nickname");

    handler.database.add_local_client(dummy_client("nick2"));
    handler.database.add_client_to_channel("#channel", "nick2");
    handler.database.add_channel_speaker("#channel", "nick2");

    assert!(handler.database.is_channel_speaker("#channel", "nick2"));

    let parameters = vec![
        "#channel".to_string(),
        "-v".to_string(),
        "nick2".to_string(),
    ];
    let prefix = Some("sender".to_string());
    handler.mode_command((prefix, parameters, None)).unwrap();

    assert!(!handler.database.is_channel_speaker("#channel", "nick2"));
}

#[test]
fn mode_sets_channel_key() {
    let mut handler = dummy_server_handler();
    handler.database.add_local_client(dummy_client("nickname"));

    handler
        .database
        .add_client_to_channel("#channel", "nickname");

    assert!(handler
        .database
        .get_channel_key("#channel")
        .unwrap()
        .is_none());

    let parameters = vec!["#channel".to_string(), "+k".to_string(), "key".to_string()];
    let prefix = Some("sender".to_string());
    handler.mode_command((prefix, parameters, None)).unwrap();

    assert_eq!(
        handler.database.get_channel_key("#channel").unwrap(),
        Some("key".to_string())
    );
}

#[test]
fn mode_unsets_channel_key() {
    let mut handler = dummy_server_handler();
    handler.database.add_local_client(dummy_client("nickname"));

    handler
        .database
        .add_client_to_channel("#channel", "nickname");
    handler
        .database
        .set_channel_key("#channel", Some("key".to_string()));

    assert_eq!(
        handler.database.get_channel_key("#channel").unwrap(),
        Some("key".to_string())
    );

    let parameters = vec!["#channel".to_string(), "-k".to_string()];
    let prefix = Some("sender".to_string());
    handler.mode_command((prefix, parameters, None)).unwrap();

    assert!(handler
        .database
        .get_channel_key("#channel")
        .unwrap()
        .is_none());
}

#[test]
fn mode_sets_and_unsets_private_flag() {
    let mut handler = dummy_server_handler();
    handler.database.add_local_client(dummy_client("nickname"));

    handler
        .database
        .add_client_to_channel("#channel", "nickname");

    assert!(!handler
        .database
        .channel_has_flag("#channel", ChannelFlag::Private));

    let mut parameters = vec!["#channel".to_string(), "+p".to_string()];
    let prefix = Some("sender".to_string());
    handler.mode_command((prefix, parameters, None)).unwrap();

    assert_eq!("", handler.stream.read_wbuf_to_string());
    assert!(handler
        .database
        .channel_has_flag("#channel", ChannelFlag::Private));

    parameters = vec!["#channel".to_string(), "-p".to_string()];

    let prefix = Some("sender".to_string());
    handler.mode_command((prefix, parameters, None)).unwrap();

    assert_eq!("", handler.stream.read_wbuf_to_string());
    assert!(!handler
        .database
        .channel_has_flag("#channel", ChannelFlag::Private));
}

#[test]
fn mode_sets_and_unsets_secret_flag() {
    let mut handler = dummy_server_handler();
    handler.database.add_local_client(dummy_client("nickname"));

    handler
        .database
        .add_client_to_channel("#channel", "nickname");

    assert!(!handler
        .database
        .channel_has_flag("#channel", ChannelFlag::Secret));

    let mut parameters = vec!["#channel".to_string(), "+s".to_string()];
    let prefix = Some("sender".to_string());
    handler.mode_command((prefix, parameters, None)).unwrap();

    assert_eq!("", handler.stream.read_wbuf_to_string());
    assert!(handler
        .database
        .channel_has_flag("#channel", ChannelFlag::Secret));

    parameters = vec!["#channel".to_string(), "-s".to_string()];

    let prefix = Some("sender".to_string());
    handler.mode_command((prefix, parameters, None)).unwrap();

    assert_eq!("", handler.stream.read_wbuf_to_string());
    assert!(!handler
        .database
        .channel_has_flag("#channel", ChannelFlag::Secret));
}

#[test]
fn mode_sets_and_unsets_invite_only_flag() {
    let mut handler = dummy_server_handler();
    handler.database.add_local_client(dummy_client("nickname"));

    handler
        .database
        .add_client_to_channel("#channel", "nickname");

    assert!(!handler
        .database
        .channel_has_flag("#channel", ChannelFlag::InviteOnly));

    let mut parameters = vec!["#channel".to_string(), "+i".to_string()];
    let prefix = Some("sender".to_string());
    handler.mode_command((prefix, parameters, None)).unwrap();

    assert_eq!("", handler.stream.read_wbuf_to_string());
    assert!(handler
        .database
        .channel_has_flag("#channel", ChannelFlag::InviteOnly));

    parameters = vec!["#channel".to_string(), "-i".to_string()];

    let prefix = Some("sender".to_string());
    handler.mode_command((prefix, parameters, None)).unwrap();

    assert_eq!("", handler.stream.read_wbuf_to_string());
    assert!(!handler
        .database
        .channel_has_flag("#channel", ChannelFlag::InviteOnly));
}

#[test]
fn mode_sets_and_unsets_topic_flag() {
    let mut handler = dummy_server_handler();
    handler.database.add_local_client(dummy_client("nickname"));

    handler
        .database
        .add_client_to_channel("#channel", "nickname");

    assert!(!handler
        .database
        .channel_has_flag("#channel", ChannelFlag::TopicByOperatorOnly));

    let mut parameters = vec!["#channel".to_string(), "+t".to_string()];
    let prefix = Some("sender".to_string());
    handler.mode_command((prefix, parameters, None)).unwrap();

    assert_eq!("", handler.stream.read_wbuf_to_string());
    assert!(handler
        .database
        .channel_has_flag("#channel", ChannelFlag::TopicByOperatorOnly));

    parameters = vec!["#channel".to_string(), "-t".to_string()];

    let prefix = Some("sender".to_string());
    handler.mode_command((prefix, parameters, None)).unwrap();

    assert_eq!("", handler.stream.read_wbuf_to_string());
    assert!(!handler
        .database
        .channel_has_flag("#channel", ChannelFlag::TopicByOperatorOnly));
}
#[test]
fn mode_sets_and_unsets_no_outside_messages_flag() {
    let mut handler = dummy_server_handler();
    handler.database.add_local_client(dummy_client("nickname"));

    handler
        .database
        .add_client_to_channel("#channel", "nickname");

    assert!(!handler
        .database
        .channel_has_flag("#channel", ChannelFlag::NoOutsideMessages));

    let mut parameters = vec!["#channel".to_string(), "+n".to_string()];
    let prefix = Some("sender".to_string());
    handler.mode_command((prefix, parameters, None)).unwrap();

    assert_eq!("", handler.stream.read_wbuf_to_string());
    assert!(handler
        .database
        .channel_has_flag("#channel", ChannelFlag::NoOutsideMessages));

    parameters = vec!["#channel".to_string(), "-n".to_string()];

    let prefix = Some("sender".to_string());
    handler.mode_command((prefix, parameters, None)).unwrap();

    assert_eq!("", handler.stream.read_wbuf_to_string());
    assert!(!handler
        .database
        .channel_has_flag("#channel", ChannelFlag::NoOutsideMessages));
}

#[test]
fn mode_sets_and_unsets_moderated_flag() {
    let mut handler = dummy_server_handler();
    handler.database.add_local_client(dummy_client("nickname"));

    handler
        .database
        .add_client_to_channel("#channel", "nickname");

    assert!(!handler
        .database
        .channel_has_flag("#channel", ChannelFlag::Moderated));

    let mut parameters = vec!["#channel".to_string(), "+m".to_string()];
    let prefix = Some("sender".to_string());
    handler.mode_command((prefix, parameters, None)).unwrap();

    assert_eq!("", handler.stream.read_wbuf_to_string());
    assert!(handler
        .database
        .channel_has_flag("#channel", ChannelFlag::Moderated));

    parameters = vec!["#channel".to_string(), "-m".to_string()];

    let prefix = Some("sender".to_string());
    handler.mode_command((prefix, parameters, None)).unwrap();

    assert_eq!("", handler.stream.read_wbuf_to_string());
    assert!(!handler
        .database
        .channel_has_flag("#channel", ChannelFlag::Moderated));
}
