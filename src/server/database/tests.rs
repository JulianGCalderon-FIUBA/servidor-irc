use crate::server::testing_utils::{dummy_client, mock_stream::MockTcpStream};

use super::*;

#[test]
fn after_adding_client_database_contains_client() {
    let database = Database::new();

    assert!(!database.contains_client("nickname"));
    database.add_client(dummy_client("nickname"));
    assert!(database.contains_client("nickname"));

    assert!(!database.contains_client("nickname2"));
    database.add_client(dummy_client("nickname2"));
    assert!(database.contains_client("nickname2"));
}

#[test]
fn after_setting_server_operator_client_is_server_operator() {
    let database = Database::new();

    database.add_client(dummy_client("nickname"));

    assert!(!database._is_server_operator("nickname"));
    database.set_server_operator("nickname");
    assert!(database._is_server_operator("nickname"));
}

#[test]
fn get_stream_returns_reference_to_client_stream() {
    let database = Database::new();

    let client = dummy_client("nickname");

    let stream_ref_expected = client.get_stream().unwrap();

    database.add_client(client);
    let stream_ref_actual = database.get_stream("nickname").unwrap();

    assert!(Arc::ptr_eq(&stream_ref_expected, &stream_ref_actual));
}

#[test]
fn disconnect_client_sets_stream_to_none() {
    let database = Database::new();

    let client = dummy_client("nickname");

    database.add_client(client);

    assert!(database.get_stream("nickname").is_some());
    database.disconnect_client("nickname");
    assert!(database.get_stream("nickname").is_none());
}

#[test]
fn after_adding_client_to_channel_it_exists() {
    let database = Database::new();

    let client = dummy_client("nickname1");
    database.add_client(client);

    assert!(!database.contains_channel("channel"));
    database.add_client_to_channel("nickname1", "channel");
    assert!(database.contains_channel("channel"));
}

#[test]
fn after_adding_client_to_channel_it_contains_client() {
    let database = Database::new();

    let client = dummy_client("nickname");
    database.add_client(client);
    database.add_client_to_channel("nickname", "channel");

    assert!(database.is_client_in_channel("nickname", "channel"));
}

#[test]
fn get_clients_returns_all_clients_from_channel() {
    let database = Database::new();

    let client = dummy_client("nickname1");
    database.add_client(client);

    let client = dummy_client("nickname2");
    database.add_client(client);

    database.add_client_to_channel("nickname1", "channel");
    database.add_client_to_channel("nickname2", "channel");

    let mut value = database.get_clients("channel");
    let expected = vec!["nickname1".to_string(), "nickname2".to_string()];
    value.sort();

    assert_eq!(value, expected)
}

#[test]
fn after_removing_client_from_channel_it_no_longer_contains_client() {
    let database = Database::new();

    let client = dummy_client("nickname1");
    database.add_client(client);

    let client = dummy_client("nickname2");
    database.add_client(client);

    database.add_client_to_channel("nickname1", "channel");
    database.add_client_to_channel("nickname2", "channel");
    database.remove_client_of_channel("nickname1", "channel");

    let value = database.get_clients("channel");
    let expected = vec!["nickname2".to_string()];

    assert_eq!(value, expected);
}

#[test]
fn after_removing_last_client_from_channel_it_no_longer_exists() {
    let database = Database::new();

    let client = dummy_client("nickname1");
    database.add_client(client);

    database.add_client_to_channel("nickname1", "channel");
    database.remove_client_of_channel("nickname1", "channel");

    assert!(!database.contains_channel("channel"));
}

#[test]
fn get_channels_returns_all_channels() {
    let database = Database::new();

    let client = dummy_client("nickname");
    database.add_client(client);
    database.add_client_to_channel("nickname", "channel1");
    database.add_client_to_channel("nickname", "channel2");

    let mut channels_real = database.get_channels();
    let channels_expected = vec!["channel1".to_string(), "channel2".to_string()];

    channels_real.sort();

    assert_eq!(channels_real, channels_expected);
}

#[test]
fn get_channels_for_client_returns_all_channels_for_client() {
    let database = Database::new();

    let client = dummy_client("nickname");
    database.add_client(client);
    database.add_client_to_channel("nickname", "channel1");
    database.add_client_to_channel("nickname", "channel2");

    let mut channels_real = database.get_channels_for_client("nickname");
    let channels_expected = vec!["channel1".to_string(), "channel2".to_string()];

    channels_real.sort();

    assert_eq!(channels_expected, channels_real);
}

#[test]
fn get_clients_for_query_returns_all_matching_clients() {
    let database = Database::new();

    database.add_client(
        ClientBuilder::new()
            .nickname("nickAname".to_string())
            .username("userBname".to_string())
            .hostname("hostCname".to_string())
            .servername("serverDname".to_string())
            .realname("realEname".to_string())
            .stream(MockTcpStream::new())
            .build()
            .unwrap(),
    );

    database.add_client(dummy_client("othernick"));

    let expected = vec!["nickAname".to_string()];

    assert_eq!(database.get_clients_for_query("*A*"), expected);
    assert_eq!(database.get_clients_for_query("*B*"), expected);
    assert_eq!(database.get_clients_for_query("*C*"), expected);
    assert_eq!(database.get_clients_for_query("*D*"), expected);
    assert_eq!(database.get_clients_for_query("*E*"), expected);
}

#[test]
fn get_all_clients_returns_all_clients() {
    let database = Database::new();

    database.add_client(dummy_client("nick1"));
    database.add_client(dummy_client("nick2"));

    let expected = vec!["nick1".to_string(), "nick2".to_string()];
    let mut real = database.get_all_clients();
    real.sort();

    assert_eq!(real, expected);
}

#[test]
fn matches_works_for_inner_wilcard() {
    let stack = "hola_como_estas";

    assert!(matches(stack, "hola*estas"));
    assert!(matches(stack, "hola*como*estas"));
    assert!(matches(stack, "*ola*como*estas"));
    assert!(matches(stack, "hola*como*esta*"));
    assert!(!matches(stack, "Xola*como*estas"));
    assert!(!matches(stack, "hola*Xomo*estas"));
    assert!(!matches(stack, "hola*como*Xstas"));
    assert!(!matches(stack, "ola*como*estas"));
    assert!(!matches(stack, "hola*como*esta"));
}
