use crate::server::{
    connection_handler::{client_handler::tests::dummy_client_handler, ConnectionHandlerCommands},
    consts::channel_flag::ChannelFlag,
    testing::{dummy_client, dummy_external_client, dummy_server},
};

#[test]
fn ctcp_fails_with_empty_params() {
    let mut handler = dummy_client_handler();
    let parameters = vec![];

    handler.ctcp_command((None, parameters, None)).unwrap();

    assert_eq!(
        "461 CTCP :Not enough parameters\r\n",
        handler.stream.read_wbuf_to_string()
    );
}

#[test]
fn ctcp_fails_with_invalid_target() {
    let mut handler = dummy_client_handler();

    let parameters = vec!["nick1".to_string()];
    let trailing = Some("message!".to_string());
    handler.ctcp_command((None, parameters, trailing)).unwrap();

    let responses = handler.stream.get_responses();

    assert_eq!("401 nick1 :No such nick/channel", responses[0]);
}

#[test]
fn ctcp_fails_with_invalid_channel_target() {
    let mut handler = dummy_client_handler();

    let parameters = vec!["#channel".to_string()];
    let trailing = Some("message!".to_string());
    handler.ctcp_command((None, parameters, trailing)).unwrap();

    let responses = handler.stream.get_responses();

    assert_eq!("401 #channel :No such nick/channel", responses[0]);
}

#[test]
fn ctcp_fails_with_not_on_channel_with_flag_n() {
    let mut handler = dummy_client_handler();

    handler.database.add_local_client(dummy_client("nick1"));
    handler.database.add_local_client(dummy_client("nick2"));
    handler.database.add_client_to_channel("#channel", "nick1");
    handler.database.add_client_to_channel("#channel", "nick2");

    handler
        .database
        .set_channel_flag("#channel", ChannelFlag::NoOutsideMessages);

    let parameters = vec!["#channel".to_string()];
    let trailing = Some("message!".to_string());
    handler.ctcp_command((None, parameters, trailing)).unwrap();

    let responses = handler.stream.get_responses();

    assert_eq!("404 #channel :Cannot send to channel", responses[0]);

    assert_eq!(
        "",
        handler
            .database
            .get_local_stream("nick1")
            .unwrap()
            .read_wbuf_to_string()
    );

    assert_eq!(
        "",
        handler
            .database
            .get_local_stream("nick2")
            .unwrap()
            .read_wbuf_to_string()
    );
}

#[test]
fn ctcp_fails_if_not_speaker_on_channel_with_flag_m() {
    let mut handler = dummy_client_handler();

    handler.database.add_local_client(dummy_client("nick1"));
    handler.database.add_client_to_channel("#channel", "nick1");
    handler
        .database
        .add_client_to_channel("#channel", "nickname");

    handler
        .database
        .set_channel_flag("#channel", ChannelFlag::Moderated);

    let parameters = vec!["#channel".to_string()];
    let trailing = Some("message!".to_string());
    handler.ctcp_command((None, parameters, trailing)).unwrap();

    let responses = handler.stream.get_responses();

    assert_eq!("404 #channel :Cannot send to channel", responses[0]);

    assert_eq!(
        "",
        handler
            .database
            .get_local_stream("nick1")
            .unwrap()
            .read_wbuf_to_string()
    );
}

#[test]
fn ctcp_fails_with_no_text() {
    let mut handler = dummy_client_handler();
    handler.database.add_local_client(dummy_client("nick1"));

    let parameters = vec!["nick1".to_string()];
    let trailing = None;
    handler.ctcp_command((None, parameters, trailing)).unwrap();

    assert_eq!(
        "412 :No text to send\r\n",
        handler.stream.read_wbuf_to_string()
    );
}

#[test]
fn ctcp_works_with_valid_target_client() {
    let mut handler = dummy_client_handler();

    handler.database.add_local_client(dummy_client("nick1"));

    let parameters = vec!["nick1".to_string()];
    let trailing = Some("message!".to_string());
    handler.ctcp_command((None, parameters, trailing)).unwrap();

    assert_eq!(
        ":nickname PRIVMSG nick1 :message!\r\n",
        handler
            .database
            .get_local_stream("nick1")
            .unwrap()
            .read_wbuf_to_string()
    );
}

#[test]
fn ctcp_works_with_valid_target_channel() {
    let mut handler = dummy_client_handler();

    handler.database.add_local_client(dummy_client("nick1"));
    handler.database.add_local_client(dummy_client("nick2"));
    handler.database.add_client_to_channel("#channel", "nick1");
    handler.database.add_client_to_channel("#channel", "nick2");
    handler
        .database
        .add_client_to_channel("#channel", "nickname");

    let parameters = vec!["#channel".to_string()];
    let trailing = Some("message!".to_string());
    handler.ctcp_command((None, parameters, trailing)).unwrap();

    let responses = handler.stream.get_responses();

    assert_eq!(":nickname PRIVMSG #channel :message!", responses[0]);

    assert_eq!(
        ":nickname PRIVMSG #channel :message!\r\n",
        handler
            .database
            .get_local_stream("nick1")
            .unwrap()
            .read_wbuf_to_string()
    );

    assert_eq!(
        ":nickname PRIVMSG #channel :message!\r\n",
        handler
            .database
            .get_local_stream("nick2")
            .unwrap()
            .read_wbuf_to_string()
    );
}

#[test]
fn privmsg_with_away_client_returns_away_message() {
    let mut handler = dummy_client_handler();

    handler.database.add_local_client(dummy_client("nick1"));
    handler
        .database
        .set_away_message("nick1", Some("away message!".to_string()));

    let parameters = vec!["nick1".to_string()];
    let trailing = Some("message!".to_string());
    handler.ctcp_command((None, parameters, trailing)).unwrap();

    assert_eq!(
        "301 nick1 :away message!\r\n",
        handler.stream.read_wbuf_to_string(),
    );
}

#[test]
fn privmsg_works_on_channel_with_flag_n() {
    let mut handler = dummy_client_handler();

    handler.database.add_local_client(dummy_client("nick1"));
    handler.database.add_local_client(dummy_client("nick2"));
    handler
        .database
        .add_client_to_channel("#channel", "nickname");
    handler.database.add_client_to_channel("#channel", "nick1");
    handler.database.add_client_to_channel("#channel", "nick2");

    handler
        .database
        .set_channel_flag("#channel", ChannelFlag::NoOutsideMessages);

    let parameters = vec!["#channel".to_string()];
    let trailing = Some("message!".to_string());
    handler.ctcp_command((None, parameters, trailing)).unwrap();

    let responses = handler.stream.get_responses();

    assert_eq!(":nickname PRIVMSG #channel :message!", responses[0]);

    assert_eq!(
        ":nickname PRIVMSG #channel :message!\r\n",
        handler
            .database
            .get_local_stream("nick1")
            .unwrap()
            .read_wbuf_to_string()
    );

    assert_eq!(
        ":nickname PRIVMSG #channel :message!\r\n",
        handler
            .database
            .get_local_stream("nick2")
            .unwrap()
            .read_wbuf_to_string()
    );
}

#[test]
fn privmsg_works_on_channel_with_flag_m() {
    let mut handler = dummy_client_handler();

    handler.database.add_local_client(dummy_client("nick1"));
    handler.database.add_client_to_channel("#channel", "nick1");
    handler
        .database
        .add_client_to_channel("#channel", "nickname");

    handler
        .database
        .set_channel_flag("#channel", ChannelFlag::Moderated);
    handler.database.add_channel_speaker("#channel", "nickname");

    let parameters = vec!["#channel".to_string()];
    let trailing = Some("message!".to_string());
    handler.ctcp_command((None, parameters, trailing)).unwrap();

    let responses = handler.stream.get_responses();

    assert_eq!(":nickname PRIVMSG #channel :message!", responses[0]);

    assert_eq!(
        ":nickname PRIVMSG #channel :message!\r\n",
        handler
            .database
            .get_local_stream("nick1")
            .unwrap()
            .read_wbuf_to_string()
    );
}

#[test]
fn privmsg_to_distributed_channel_is_only_relayed_to_each_neccesary_server_once() {
    let mut handler = dummy_client_handler();

    handler
        .database
        .add_immediate_server(dummy_server("servername1"));
    handler
        .database
        .add_immediate_server(dummy_server("servername2"));

    handler
        .database
        .add_external_client(dummy_external_client("nickname1", "servername1"));
    handler
        .database
        .add_external_client(dummy_external_client("nickname2", "servername1"));

    handler
        .database
        .add_client_to_channel("#channel", "nickname1");
    handler
        .database
        .add_client_to_channel("#channel", "nickname2");

    let parameters = vec!["#channel".to_string()];
    let trailing = Some("message!".to_string());
    handler.ctcp_command((None, parameters, trailing)).unwrap();

    assert_eq!(
        ":nickname PRIVMSG #channel :message!\r\n",
        handler
            .database
            .get_server_stream("servername1")
            .unwrap()
            .read_wbuf_to_string()
    );
    assert_eq!(
        "",
        handler
            .database
            .get_server_stream("servername2")
            .unwrap()
            .read_wbuf_to_string()
    );
}
