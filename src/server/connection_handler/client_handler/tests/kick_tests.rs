use crate::server::testing::dummy_server;

use super::*;

#[test]
fn kick_fails_with_empty_params() {
    let mut handler = dummy_client_handler();

    handler.kick_command((None, vec![], None)).unwrap();

    assert_eq!(
        "461 KICK :Not enough parameters\r\n",
        handler.stream.read_wbuf_to_string()
    );

    let channels: Vec<String> = vec![];
    assert_eq!(handler.database.get_all_channels(), channels);
}

#[test]
fn kick_fails_when_not_on_channel() {
    let mut handler = dummy_client_handler();

    handler.database.add_local_client(dummy_client("nick1"));
    handler.database.add_client_to_channel("nick1", "#channel1");

    let parameters = vec!["#channel1".to_string(), "nick1".to_string()];
    handler.kick_command((None, parameters, None)).unwrap();

    assert_eq!(
        "442 #channel1 :You're not on that channel\r\n",
        handler.stream.read_wbuf_to_string()
    );
}

#[test]
fn kick_fails_when_not_operator() {
    let mut handler = dummy_client_handler();

    handler.database.add_local_client(dummy_client("nick2"));
    handler.database.add_client_to_channel("nick2", "#channel1");
    handler
        .database
        .add_client_to_channel("nickname", "#channel1");

    let parameters = vec!["#channel1".to_string(), "nickname".to_string()];
    handler.kick_command((None, parameters, None)).unwrap();

    assert_eq!(
        "482 #channel1 :You're not channel operator\r\n",
        handler.stream.read_wbuf_to_string()
    );
}

#[test]
fn kick_fails_when_channel_does_not_exist() {
    let mut handler = dummy_client_handler();

    let parameters = vec!["#channel1".to_string(), "nickname".to_string()];
    handler.kick_command((None, parameters, None)).unwrap();

    assert_eq!(
        "403 #channel1 :No such channel\r\n",
        handler.stream.read_wbuf_to_string()
    );
}

#[test]
fn can_kick_user_from_channel() {
    let mut handler = dummy_client_handler();

    handler.database.add_local_client(dummy_client("nick2"));
    handler
        .database
        .add_client_to_channel("nickname", "#channel");
    handler.database.add_client_to_channel("nick2", "#channel");

    let parameters = vec!["#channel".to_string(), "nick2".to_string()];
    handler.kick_command((None, parameters, None)).unwrap();

    assert!(!handler.database.is_client_in_channel("nick2", "#channel"));

    assert_eq!(
        ":nickname KICK #channel nick2\r\n",
        handler
            .database
            .get_local_stream("nick2")
            .unwrap()
            .read_wbuf_to_string()
    );
}

// #[test]
// fn can_kick_user_with_old_nickname() {
//     let mut handler = dummy_client_handler();

//     handler.database.add_local_client(dummy_client("nick2"));
//     handler
//         .database
//         .add_client_to_channel("nickname", "#channel");
//     handler.database.add_client_to_channel("nick2", "#channel");

//     handler.database.update_nickname("nick2", "nick3");

//     let parameters = vec!["#channel".to_string(), "nick2".to_string()];
//     handler.kick_command((None, parameters, None)).unwrap();

//     assert!(!handler.database.is_client_in_channel("nick3", "#channel"));
// }

#[test]
fn can_kick_user_from_channel_with_comment() {
    let mut handler = dummy_client_handler();

    handler.database.add_local_client(dummy_client("nick2"));
    handler
        .database
        .add_client_to_channel("nickname", "#channel");
    handler.database.add_client_to_channel("nick2", "#channel");

    let parameters = vec!["#channel".to_string(), "nick2".to_string()];
    let trailing = Some("no lollygagging".to_string());
    handler.kick_command((None, parameters, trailing)).unwrap();

    assert!(!handler.database.is_client_in_channel("nick2", "#channel"));

    assert_eq!(
        ":nickname KICK #channel nick2 :no lollygagging\r\n",
        handler
            .database
            .get_local_stream("nick2")
            .unwrap()
            .read_wbuf_to_string()
    );
}

#[test]
fn can_kick_multiple_user() {
    let mut handler = dummy_client_handler();

    handler.database.add_local_client(dummy_client("nick2"));
    handler.database.add_local_client(dummy_client("nick3"));
    handler
        .database
        .add_client_to_channel("nickname", "#channel");
    handler.database.add_client_to_channel("nick2", "#channel");
    handler.database.add_client_to_channel("nick3", "#channel");

    let parameters = vec!["#channel,#channel".to_string(), "nick2,nick3".to_string()];
    let trailing = Some("no lollygagging".to_string());
    handler.kick_command((None, parameters, trailing)).unwrap();

    assert!(!handler.database.is_client_in_channel("nick2", "#channel"));
    assert!(!handler.database.is_client_in_channel("nick3", "#channel"));
}

#[test]
fn kick_notifies_users_in_channel() {
    let mut handler = dummy_client_handler();

    handler.database.add_local_client(dummy_client("nick2"));
    handler.database.add_local_client(dummy_client("nick3"));
    handler
        .database
        .add_client_to_channel("nickname", "#channel");
    handler.database.add_client_to_channel("nick2", "#channel");
    handler.database.add_client_to_channel("nick3", "#channel");

    let parameters = vec!["#channel".to_string(), "nick2".to_string()];
    let trailing = Some("no lollygagging".to_string());
    handler.kick_command((None, parameters, trailing)).unwrap();

    let responses = handler
        .database
        .get_local_stream("nick3")
        .unwrap()
        .get_responses();

    assert_eq!(
        ":nickname KICK #channel nick2 :no lollygagging",
        responses[0]
    );
}

#[test]
fn on_distributed_channels_kick_is_relayed_to_all_servers() {
    let mut handler = dummy_client_handler();

    handler
        .database()
        .add_immediate_server(dummy_server("servername1"));
    handler
        .database()
        .add_immediate_server(dummy_server("servername2"));

    handler
        .database()
        .add_local_client(dummy_client("nickname1"));

    handler
        .database()
        .add_client_to_channel("nickname", "#channel");
    handler
        .database()
        .add_client_to_channel("nickname1", "#channel");

    let params = vec!["#channel".to_string(), "nickname1".to_string()];
    let trail = Some("message".to_string());
    handler.kick_command((None, params, trail)).unwrap();

    assert_eq!(
        ":nickname KICK #channel nickname1 :message\r\n",
        handler
            .database
            .get_server_stream("servername1")
            .unwrap()
            .read_wbuf_to_string()
    );
    assert_eq!(
        ":nickname KICK #channel nickname1 :message\r\n",
        handler
            .database
            .get_server_stream("servername2")
            .unwrap()
            .read_wbuf_to_string()
    );
}
