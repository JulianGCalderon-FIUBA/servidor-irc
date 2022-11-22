use super::*;

#[test]
fn mode_fails_with_not_enough_parameters() {
    let mut handler = dummy_client_handler();

    let parameters = vec![];
    handler.mode_command((None, parameters, None)).unwrap();

    assert_eq!(
        "461 MODE :Not enough parameters\r\n",
        handler.stream.read_wbuf_to_string()
    )
}

#[test]
fn mode_fails_with_nonexistent_channel() {
    let mut handler = dummy_client_handler();

    let parameters = vec!["#channel".to_string()];
    handler.mode_command((None, parameters, None)).unwrap();

    assert_eq!(
        "403 #channel :No such channel\r\n",
        handler.stream.read_wbuf_to_string()
    )
}

#[test]
fn mode_fails_with_user_not_on_channel() {
    let mut handler = dummy_client_handler();

    handler.database.add_client(dummy_client("dummy"));
    handler.database.add_client_to_channel("dummy", "#channel");

    let parameters = vec!["#channel".to_string()];
    handler.mode_command((None, parameters, None)).unwrap();

    assert_eq!(
        "442 #channel :You're not on that channel\r\n",
        handler.stream.read_wbuf_to_string()
    )
}

#[test]
fn mode_fails_with_user_not_operator() {
    let mut handler = dummy_client_handler();

    handler.database.add_client(dummy_client("dummy"));
    handler.database.add_client_to_channel("dummy", "#channel");
    handler
        .database
        .add_client_to_channel("nickname", "#channel");

    let parameters = vec!["#channel".to_string(), "+i".to_string()];
    handler.mode_command((None, parameters, None)).unwrap();

    assert_eq!(
        "482 #channel :You're not channel operator\r\n",
        handler.stream.read_wbuf_to_string()
    )
}

#[test]
fn mode_adds_channop() {
    let mut handler = dummy_client_handler();

    handler
        .database
        .add_client_to_channel("nickname", "#channel");

    handler.database.add_client(dummy_client("nick2"));
    handler.database.add_client_to_channel("nick2", "#channel");

    assert!(!handler.database.is_channel_operator("#channel", "nick2"));

    let parameters = vec![
        "#channel".to_string(),
        "+o".to_string(),
        "nick2".to_string(),
    ];
    handler.mode_command((None, parameters, None)).unwrap();

    assert_eq!("", handler.stream.read_wbuf_to_string());
    assert!(handler.database.is_channel_operator("#channel", "nick2"));
}

#[test]
fn mode_removes_channop() {
    let mut handler = dummy_client_handler();

    handler
        .database
        .add_client_to_channel("nickname", "#channel");

    handler.database.add_client(dummy_client("nick2"));
    handler.database.add_client_to_channel("nick2", "#channel");
    handler.database.add_channop("#channel", "nick2");

    assert!(handler.database.is_channel_operator("#channel", "nick2"));

    let parameters = vec![
        "#channel".to_string(),
        "-o".to_string(),
        "nick2".to_string(),
    ];
    handler.mode_command((None, parameters, None)).unwrap();

    assert_eq!("", handler.stream.read_wbuf_to_string());
    assert!(handler.database.is_channel_operator("#channel", "nickname"));
    assert!(!handler.database.is_channel_operator("#channel", "nick2"));
}

#[test]
fn mode_fails_with_no_oper_parameter() {
    let mut handler = dummy_client_handler();

    handler
        .database
        .add_client_to_channel("nickname", "#channel");

    handler.database.add_client(dummy_client("nick2"));
    handler.database.add_client_to_channel("nick2", "#channel");

    assert!(!handler.database.is_channel_operator("#channel", "nick2"));

    let parameters = vec!["#channel".to_string(), "+o".to_string()];
    handler.mode_command((None, parameters, None)).unwrap();

    assert_eq!(
        "461 MODE :Not enough parameters\r\n",
        handler.stream.read_wbuf_to_string()
    );
    assert!(!handler.database.is_channel_operator("#channel", "nick2"));
}

#[test]
fn mode_fails_with_nonexistent_oper() {
    let mut handler = dummy_client_handler();

    handler
        .database
        .add_client_to_channel("nickname", "#channel");

    let parameters = vec![
        "#channel".to_string(),
        "+o".to_string(),
        "nick2".to_string(),
    ];
    handler.mode_command((None, parameters, None)).unwrap();

    assert_eq!(
        "401 nick2 :No such nick/channel\r\n",
        handler.stream.read_wbuf_to_string()
    );
    assert!(!handler.database.is_channel_operator("#channel", "nick2"));
}

#[test]
fn mode_oper_fails_with_nick_not_on_channel() {
    let mut handler = dummy_client_handler();

    handler
        .database
        .add_client_to_channel("nickname", "#channel");

    handler.database.add_client(dummy_client("nick2"));

    let parameters = vec![
        "#channel".to_string(),
        "+o".to_string(),
        "nick2".to_string(),
    ];
    handler.mode_command((None, parameters, None)).unwrap();

    assert_eq!(
        "442 #channel :You're not on that channel\r\n",
        handler.stream.read_wbuf_to_string()
    );
    assert!(!handler.database.is_channel_operator("#channel", "nick2"));
}

#[test]
fn mode_sets_limit_to_channel() {
    let mut handler = dummy_client_handler();

    handler
        .database
        .add_client_to_channel("nickname", "#channel");

    assert!(handler.database.get_channel_limit("#channel").is_none());

    let parameters = vec!["#channel".to_string(), "+l".to_string(), "5".to_string()];
    handler.mode_command((None, parameters, None)).unwrap();

    assert_eq!("", handler.stream.read_wbuf_to_string());
    assert_eq!(handler.database.get_channel_limit("#channel"), Some(5));
}

#[test]
fn mode_unsets_channel_limit() {
    let mut handler = dummy_client_handler();

    handler
        .database
        .add_client_to_channel("nickname", "#channel");
    handler.database.set_channel_limit("#channel", Some(5));

    assert_eq!(handler.database.get_channel_limit("#channel"), Some(5));

    let parameters = vec!["#channel".to_string(), "-l".to_string()];
    handler.mode_command((None, parameters, None)).unwrap();

    assert_eq!("", handler.stream.read_wbuf_to_string());
    assert!(handler.database.get_channel_limit("#channel").is_none());
}

#[test]
fn mode_fails_with_no_limit_parameter() {
    let mut handler = dummy_client_handler();

    handler
        .database
        .add_client_to_channel("nickname", "#channel");

    assert!(handler.database.get_channel_limit("#channel").is_none());

    let parameters = vec!["#channel".to_string(), "+l".to_string()];
    handler.mode_command((None, parameters, None)).unwrap();

    assert_eq!(
        "461 MODE :Not enough parameters\r\n",
        handler.stream.read_wbuf_to_string()
    );
    assert!(handler.database.get_channel_limit("#channel").is_none());
}

#[test]
fn mode_sets_banmask() {
    let mut handler = dummy_client_handler();

    handler
        .database
        .add_client_to_channel("nickname", "#channel");

    assert!(handler.database.get_channel_banmask("#channel").is_empty());

    let parameters = vec![
        "#channel".to_string(),
        "+b".to_string(),
        "banmask".to_string(),
    ];
    handler.mode_command((None, parameters, None)).unwrap();

    assert_eq!("", handler.stream.read_wbuf_to_string());

    let masks = vec!["banmask".to_string()];

    assert_eq!(masks, handler.database.get_channel_banmask("#channel"))
}

#[test]
fn mode_unsets_banmask() {
    let mut handler = dummy_client_handler();

    handler
        .database
        .add_client_to_channel("nickname", "#channel");

    handler.database.add_channel_banmask("#channel", "banmask");
    handler.database.add_channel_banmask("#channel", "banmask2");
    assert!(!handler.database.get_channel_banmask("#channel").is_empty());

    let parameters = vec![
        "#channel".to_string(),
        "-b".to_string(),
        "banmask".to_string(),
    ];
    handler.mode_command((None, parameters, None)).unwrap();

    assert_eq!("", handler.stream.read_wbuf_to_string());

    let masks = vec!["banmask2".to_string()];

    assert_eq!(masks, handler.database.get_channel_banmask("#channel"));

    let parameters = vec![
        "#channel".to_string(),
        "-b".to_string(),
        "banmask2".to_string(),
    ];
    handler.mode_command((None, parameters, None)).unwrap();

    assert!(handler.database.get_channel_banmask("#channel").is_empty())
}

#[test]
fn mode_returns_ban_list_with_no_parameters() {
    let mut handler = dummy_client_handler();

    handler
        .database
        .add_client_to_channel("nickname", "#channel");

    assert!(handler.database.get_channel_banmask("#channel").is_empty());

    handler.database.add_channel_banmask("#channel", "banmask1");
    handler.database.add_channel_banmask("#channel", "banmask2");
    handler.database.add_channel_banmask("#channel", "banmask3");

    let parameters = vec!["#channel".to_string(), "+b".to_string()];
    handler.mode_command((None, parameters, None)).unwrap();

    let responses = handler.stream.get_responses();

    assert_eq!("367 #channel banmask1", responses[0]);
    assert_eq!("367 #channel banmask2", responses[1]);
    assert_eq!("367 #channel banmask3", responses[2]);
    assert_eq!("368 #channel :End of channel ban list", responses[3]);
}

#[test]
fn mode_fails_with_no_banmask_parameter() {
    let mut handler = dummy_client_handler();

    handler
        .database
        .add_client_to_channel("nickname", "#channel");

    handler.database.add_channel_banmask("#channel", "banmask");
    assert!(!handler.database.get_channel_banmask("#channel").is_empty());

    let parameters = vec!["#channel".to_string(), "-b".to_string()];
    handler.mode_command((None, parameters, None)).unwrap();

    assert_eq!(
        "461 MODE :Not enough parameters\r\n",
        handler.stream.read_wbuf_to_string()
    );
}

#[test]
fn mode_ignores_unknown_banmask_when_unsetting() {
    let mut handler = dummy_client_handler();

    handler
        .database
        .add_client_to_channel("nickname", "#channel");

    let parameters = vec![
        "#channel".to_string(),
        "-b".to_string(),
        "banmask".to_string(),
    ];
    handler.mode_command((None, parameters, None)).unwrap();

    assert_eq!("", handler.stream.read_wbuf_to_string());
    assert!(handler.database.get_channel_banmask("#channel").is_empty());
}

#[test]
fn mode_adds_speaker_to_channel() {
    let mut handler = dummy_client_handler();

    handler
        .database
        .add_client_to_channel("nickname", "#channel");

    handler.database.add_client(dummy_client("nick2"));
    handler.database.add_client_to_channel("nick2", "#channel");

    assert!(!handler.database.is_channel_speaker("#channel", "nick2"));

    let parameters = vec![
        "#channel".to_string(),
        "+v".to_string(),
        "nick2".to_string(),
    ];
    handler.mode_command((None, parameters, None)).unwrap();

    assert_eq!("", handler.stream.read_wbuf_to_string());
    assert!(handler.database.is_channel_speaker("#channel", "nick2"));
}

#[test]
fn mode_removes_speakers_from_channel() {
    let mut handler = dummy_client_handler();

    handler
        .database
        .add_client_to_channel("nickname", "#channel");

    handler.database.add_client(dummy_client("nick2"));
    handler.database.add_client_to_channel("nick2", "#channel");
    handler.database.add_speaker("#channel", "nick2");

    assert!(handler.database.is_channel_speaker("#channel", "nick2"));

    let parameters = vec![
        "#channel".to_string(),
        "-v".to_string(),
        "nick2".to_string(),
    ];
    handler.mode_command((None, parameters, None)).unwrap();

    assert_eq!("", handler.stream.read_wbuf_to_string());
    assert!(!handler.database.is_channel_speaker("#channel", "nick2"));
}

#[test]
fn mode_fails_with_no_speaker_parameter() {
    let mut handler = dummy_client_handler();

    handler
        .database
        .add_client_to_channel("nickname", "#channel");

    handler.database.add_client(dummy_client("nick2"));
    handler.database.add_client_to_channel("nick2", "#channel");

    assert!(!handler.database.is_channel_speaker("#channel", "nick2"));

    let parameters = vec!["#channel".to_string(), "+v".to_string()];
    handler.mode_command((None, parameters, None)).unwrap();

    assert_eq!(
        "461 MODE :Not enough parameters\r\n",
        handler.stream.read_wbuf_to_string()
    );
    assert!(!handler.database.is_channel_speaker("#channel", "nick2"));
}

#[test]
fn mode_fails_with_nonexistent_speaker() {
    let mut handler = dummy_client_handler();

    handler
        .database
        .add_client_to_channel("nickname", "#channel");

    let parameters = vec![
        "#channel".to_string(),
        "+v".to_string(),
        "nick2".to_string(),
    ];
    handler.mode_command((None, parameters, None)).unwrap();

    assert_eq!(
        "401 nick2 :No such nick/channel\r\n",
        handler.stream.read_wbuf_to_string()
    );
    assert!(!handler.database.is_channel_speaker("#channel", "nick2"));
}

#[test]
fn mode_speaker_fails_with_nick_not_on_channel() {
    let mut handler = dummy_client_handler();

    handler
        .database
        .add_client_to_channel("nickname", "#channel");

    handler.database.add_client(dummy_client("nick2"));

    let parameters = vec![
        "#channel".to_string(),
        "+v".to_string(),
        "nick2".to_string(),
    ];
    handler.mode_command((None, parameters, None)).unwrap();

    assert_eq!(
        "442 #channel :You're not on that channel\r\n",
        handler.stream.read_wbuf_to_string()
    );
    assert!(!handler.database.is_channel_speaker("#channel", "nick2"));
}

#[test]
fn mode_sets_channel_key() {
    let mut handler = dummy_client_handler();

    handler
        .database
        .add_client_to_channel("nickname", "#channel");

    assert!(handler.database.get_channel_key("#channel").is_none());

    let parameters = vec!["#channel".to_string(), "+k".to_string(), "key".to_string()];
    handler.mode_command((None, parameters, None)).unwrap();

    assert_eq!("", handler.stream.read_wbuf_to_string());
    assert_eq!(
        handler.database.get_channel_key("#channel"),
        Some("key".to_string())
    );
}

#[test]
fn mode_unsets_channel_key() {
    let mut handler = dummy_client_handler();

    handler
        .database
        .add_client_to_channel("nickname", "#channel");
    handler
        .database
        .set_channel_key("#channel", Some("key".to_string()));

    assert_eq!(
        handler.database.get_channel_key("#channel"),
        Some("key".to_string())
    );

    let parameters = vec!["#channel".to_string(), "-k".to_string()];
    handler.mode_command((None, parameters, None)).unwrap();

    assert_eq!("", handler.stream.read_wbuf_to_string());
    assert!(handler.database.get_channel_key("#channel").is_none());
}

#[test]
fn mode_fails_with_no_key_parameter() {
    let mut handler = dummy_client_handler();

    handler
        .database
        .add_client_to_channel("nickname", "#channel");

    assert!(handler.database.get_channel_key("#channel").is_none());

    let parameters = vec!["#channel".to_string(), "+k".to_string()];
    handler.mode_command((None, parameters, None)).unwrap();

    assert_eq!(
        "461 MODE :Not enough parameters\r\n",
        handler.stream.read_wbuf_to_string()
    );
    assert!(handler.database.get_channel_key("#channel").is_none());
}

#[test]
fn mode_fails_with_key_already_set() {
    let mut handler = dummy_client_handler();

    handler
        .database
        .add_client_to_channel("nickname", "#channel");
    handler
        .database
        .set_channel_key("#channel", Some("key".to_string()));

    assert_eq!(
        handler.database.get_channel_key("#channel"),
        Some("key".to_string())
    );

    let parameters = vec![
        "#channel".to_string(),
        "+k".to_string(),
        "new_key".to_string(),
    ];
    handler.mode_command((None, parameters, None)).unwrap();

    assert_eq!(
        "467 #channel :Channel key already set\r\n",
        handler.stream.read_wbuf_to_string()
    );
    assert_eq!(
        handler.database.get_channel_key("#channel"),
        Some("key".to_string())
    );
}

#[test]
fn mode_sets_and_unsets_private_flag() {
    let mut handler = dummy_client_handler();

    handler
        .database
        .add_client_to_channel("nickname", "#channel");

    assert!(!handler
        .database
        .channel_has_mode("#channel", &ChannelFlag::Private));

    let mut parameters = vec!["#channel".to_string(), "+p".to_string()];
    handler.mode_command((None, parameters, None)).unwrap();

    assert_eq!("", handler.stream.read_wbuf_to_string());
    assert!(handler
        .database
        .channel_has_mode("#channel", &ChannelFlag::Private));

    parameters = vec!["#channel".to_string(), "-p".to_string()];

    handler.mode_command((None, parameters, None)).unwrap();

    assert_eq!("", handler.stream.read_wbuf_to_string());
    assert!(!handler
        .database
        .channel_has_mode("#channel", &ChannelFlag::Private));
}

#[test]
fn mode_sets_and_unsets_secret_flag() {
    let mut handler = dummy_client_handler();

    handler
        .database
        .add_client_to_channel("nickname", "#channel");

    assert!(!handler
        .database
        .channel_has_mode("#channel", &ChannelFlag::Secret));

    let mut parameters = vec!["#channel".to_string(), "+s".to_string()];
    handler.mode_command((None, parameters, None)).unwrap();

    assert_eq!("", handler.stream.read_wbuf_to_string());
    assert!(handler
        .database
        .channel_has_mode("#channel", &ChannelFlag::Secret));

    parameters = vec!["#channel".to_string(), "-s".to_string()];

    handler.mode_command((None, parameters, None)).unwrap();

    assert_eq!("", handler.stream.read_wbuf_to_string());
    assert!(!handler
        .database
        .channel_has_mode("#channel", &ChannelFlag::Secret));
}

#[test]
fn mode_sets_and_unsets_invite_only_flag() {
    let mut handler = dummy_client_handler();

    handler
        .database
        .add_client_to_channel("nickname", "#channel");

    assert!(!handler
        .database
        .channel_has_mode("#channel", &ChannelFlag::InviteOnly));

    let mut parameters = vec!["#channel".to_string(), "+i".to_string()];
    handler.mode_command((None, parameters, None)).unwrap();

    assert_eq!("", handler.stream.read_wbuf_to_string());
    assert!(handler
        .database
        .channel_has_mode("#channel", &ChannelFlag::InviteOnly));

    parameters = vec!["#channel".to_string(), "-i".to_string()];

    handler.mode_command((None, parameters, None)).unwrap();

    assert_eq!("", handler.stream.read_wbuf_to_string());
    assert!(!handler
        .database
        .channel_has_mode("#channel", &ChannelFlag::InviteOnly));
}

#[test]
fn mode_sets_and_unsets_topic_flag() {
    let mut handler = dummy_client_handler();

    handler
        .database
        .add_client_to_channel("nickname", "#channel");

    assert!(!handler
        .database
        .channel_has_mode("#channel", &ChannelFlag::TopicByOperatorOnly));

    let mut parameters = vec!["#channel".to_string(), "+t".to_string()];
    handler.mode_command((None, parameters, None)).unwrap();

    assert_eq!("", handler.stream.read_wbuf_to_string());
    assert!(handler
        .database
        .channel_has_mode("#channel", &ChannelFlag::TopicByOperatorOnly));

    parameters = vec!["#channel".to_string(), "-t".to_string()];

    handler.mode_command((None, parameters, None)).unwrap();

    assert_eq!("", handler.stream.read_wbuf_to_string());
    assert!(!handler
        .database
        .channel_has_mode("#channel", &ChannelFlag::TopicByOperatorOnly));
}
#[test]
fn mode_sets_and_unsets_no_outside_messages_flag() {
    let mut handler = dummy_client_handler();

    handler
        .database
        .add_client_to_channel("nickname", "#channel");

    assert!(!handler
        .database
        .channel_has_mode("#channel", &ChannelFlag::NoOutsideMessages));

    let mut parameters = vec!["#channel".to_string(), "+n".to_string()];
    handler.mode_command((None, parameters, None)).unwrap();

    assert_eq!("", handler.stream.read_wbuf_to_string());
    assert!(handler
        .database
        .channel_has_mode("#channel", &ChannelFlag::NoOutsideMessages));

    parameters = vec!["#channel".to_string(), "-n".to_string()];

    handler.mode_command((None, parameters, None)).unwrap();

    assert_eq!("", handler.stream.read_wbuf_to_string());
    assert!(!handler
        .database
        .channel_has_mode("#channel", &ChannelFlag::NoOutsideMessages));
}

#[test]
fn mode_sets_and_unsets_moderated_flag() {
    let mut handler = dummy_client_handler();

    handler
        .database
        .add_client_to_channel("nickname", "#channel");

    assert!(!handler
        .database
        .channel_has_mode("#channel", &ChannelFlag::Moderated));

    let mut parameters = vec!["#channel".to_string(), "+m".to_string()];
    handler.mode_command((None, parameters, None)).unwrap();

    assert_eq!("", handler.stream.read_wbuf_to_string());
    assert!(handler
        .database
        .channel_has_mode("#channel", &ChannelFlag::Moderated));

    parameters = vec!["#channel".to_string(), "-m".to_string()];

    handler.mode_command((None, parameters, None)).unwrap();

    assert_eq!("", handler.stream.read_wbuf_to_string());
    assert!(!handler
        .database
        .channel_has_mode("#channel", &ChannelFlag::Moderated));
}

// #[test]
// fn mode_fails_when_setting_unknown_mode() {
//     let mut handler = dummy_client_handler();

//     handler
//         .database
//         .add_client_to_channel("nickname", "#channel");

//     assert!(!handler.database.channel_has_mode("#channel", 'w'));

//     let parameters = vec!["#channel".to_string(), "+w".to_string()];
//     handler.mode_command((None, parameters, None)).unwrap();

//     assert_eq!(
//         "472 w :Is unknown mode char to me\r\n",
//         handler.stream.read_wbuf_to_string()
//     );
//     assert!(!handler.database.channel_has_mode("#channel", 'w'));
// }

// #[test]
// fn mode_fails_when_unsetting_unknown_mode() {
//     let mut handler = dummy_client_handler();

//     handler
//         .database
//         .add_client_to_channel("nickname", "#channel");

//     assert!(!handler.database.channel_has_mode("#channel", 'w'));

//     let parameters = vec!["#channel".to_string(), "-w".to_string()];
//     handler.mode_command((None, parameters, None)).unwrap();

//     assert_eq!(
//         "472 w :Is unknown mode char to me\r\n",
//         handler.stream.read_wbuf_to_string()
//     );
//     assert!(!handler.database.channel_has_mode("#channel", 'w'));
// }

#[test]
fn mode_sets_multiple_flags() {
    let mut handler = dummy_client_handler();

    handler
        .database
        .add_client_to_channel("nickname", "#channel");

    handler.database.add_client(dummy_client("nick2"));
    handler.database.add_client_to_channel("nick2", "#channel");

    assert!(!handler
        .database
        .channel_has_mode("#channel", &ChannelFlag::Secret));
    assert!(!handler
        .database
        .channel_has_mode("#channel", &ChannelFlag::InviteOnly));
    assert!(!handler.database.is_channel_operator("#channel", "nick2"));

    let parameters = vec![
        "#channel".to_string(),
        "+ois".to_string(),
        "nick2".to_string(),
    ];
    handler.mode_command((None, parameters, None)).unwrap();

    assert_eq!("", handler.stream.read_wbuf_to_string());
    assert!(handler
        .database
        .channel_has_mode("#channel", &ChannelFlag::Secret));
    assert!(handler
        .database
        .channel_has_mode("#channel", &ChannelFlag::InviteOnly));
    assert!(handler.database.is_channel_operator("#channel", "nick2"));
}

#[test]
fn mode_unsets_multiple_flags() {
    let mut handler = dummy_client_handler();

    handler
        .database
        .add_client_to_channel("nickname", "#channel");

    handler.database.add_client(dummy_client("nick2"));
    handler.database.add_client_to_channel("nick2", "#channel");

    handler
        .database
        .set_channel_mode("#channel", ChannelFlag::Secret);
    handler
        .database
        .set_channel_mode("#channel", ChannelFlag::InviteOnly);
    handler.database.add_channop("#channel", "nick2");

    let parameters = vec![
        "#channel".to_string(),
        "-ois".to_string(),
        "nick2".to_string(),
    ];
    handler.mode_command((None, parameters, None)).unwrap();

    assert_eq!("", handler.stream.read_wbuf_to_string());
    assert!(!handler
        .database
        .channel_has_mode("#channel", &ChannelFlag::Secret));
    assert!(!handler
        .database
        .channel_has_mode("#channel", &ChannelFlag::InviteOnly));
    assert!(!handler.database.is_channel_operator("#channel", "nick2"));
}

#[test]
fn mode_sets_and_unsets_multiple_flags() {
    let mut handler = dummy_client_handler();

    handler
        .database
        .add_client_to_channel("nickname", "#channel");

    handler.database.add_client(dummy_client("nick2"));
    handler.database.add_client_to_channel("nick2", "#channel");

    handler
        .database
        .set_channel_mode("#channel", ChannelFlag::Secret);
    handler
        .database
        .set_channel_mode("#channel", ChannelFlag::Moderated);
    handler.database.add_channop("#channel", "nick2");

    let parameters = vec![
        "#channel".to_string(),
        "-os+p-m+i".to_string(),
        "nick2".to_string(),
    ];
    handler.mode_command((None, parameters, None)).unwrap();

    assert_eq!("", handler.stream.read_wbuf_to_string());
    assert!(!handler
        .database
        .channel_has_mode("#channel", &ChannelFlag::Secret));
    assert!(!handler.database.is_channel_operator("#channel", "nick2"));
    assert!(handler
        .database
        .channel_has_mode("#channel", &ChannelFlag::Private));
    assert!(!handler
        .database
        .channel_has_mode("#channel", &ChannelFlag::Moderated));
    assert!(handler
        .database
        .channel_has_mode("#channel", &ChannelFlag::InviteOnly));
}

#[test]
fn mode_sets_and_unsets_multiple_valid_flags() {
    let mut handler = dummy_client_handler();

    handler
        .database
        .add_client_to_channel("nickname", "#channel");

    handler
        .database
        .set_channel_mode("#channel", ChannelFlag::Secret);
    handler
        .database
        .set_channel_mode("#channel", ChannelFlag::Moderated);

    let parameters = vec![
        "#channel".to_string(),
        "-os+p-m+i".to_string(),
        "nick2".to_string(),
    ];
    handler.mode_command((None, parameters, None)).unwrap();

    assert_eq!(
        "401 nick2 :No such nick/channel\r\n",
        handler.stream.read_wbuf_to_string()
    );
    assert!(!handler
        .database
        .channel_has_mode("#channel", &ChannelFlag::Secret));
    assert!(handler
        .database
        .channel_has_mode("#channel", &ChannelFlag::Private));
    assert!(!handler
        .database
        .channel_has_mode("#channel", &ChannelFlag::Moderated));
    assert!(handler
        .database
        .channel_has_mode("#channel", &ChannelFlag::InviteOnly));
}
