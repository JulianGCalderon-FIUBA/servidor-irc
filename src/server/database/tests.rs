use crate::server::testing_utils::{dummy_client, mock_stream::MockTcpStream};

use super::*;

#[test]
fn can_add_client() {
    let database = Database::new();

    assert!(!database.contains_client("nickname"));
    database.add_client(dummy_client("nickname"));
    assert!(database.contains_client("nickname"));

    assert!(!database.contains_client("nickname2"));
    database.add_client(dummy_client("nickname2"));
    assert!(database.contains_client("nickname2"));
}

#[test]
fn can_set_server_operator() {
    let database = Database::new();

    database.add_client(dummy_client("nickname"));

    assert!(!database.is_server_operator("nickname"));
    database.set_server_operator("nickname");
    assert!(database.is_server_operator("nickname"));
}

#[test]
fn can_get_client_stream() {
    let database = Database::new();

    let client = dummy_client("nickname");

    let stream_ref_expected = client.get_stream().unwrap();

    database.add_client(client);
    let stream_ref_actual = database.get_stream("nickname").unwrap();

    assert!(Arc::ptr_eq(&stream_ref_expected, &stream_ref_actual));
}

#[test]
fn can_disconnect_client() {
    let database = Database::new();

    let client = dummy_client("nickname");

    database.add_client(client);

    assert!(database.get_stream("nickname").is_some());
    database.disconnect_client("nickname");
    assert!(database.get_stream("nickname").is_none());
}

#[test]
fn can_add_client_to_channel() {
    let database = Database::new();

    let client = dummy_client("nickname");
    database.add_client(client);

    assert!(!database.contains_channel("channel"));
    database.add_client_to_channel("nickname", "channel");
    assert!(database.contains_channel("channel"));

    assert!(database.is_client_in_channel("nickname", "channel"));
}

#[test]
fn can_get_all_clients_from_channel() {
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
fn can_remove_client_from_channel() {
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
fn removing_last_client_from_channel_deletes_channel() {
    let database = Database::new();

    let client = dummy_client("nickname1");
    database.add_client(client);

    database.add_client_to_channel("nickname1", "channel");
    database.remove_client_of_channel("nickname1", "channel");

    assert!(!database.contains_channel("channel"));
}

#[test]
fn can_get_all_channels() {
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
fn can_get_all_channels_from_client() {
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
fn can_get_all_clients_for_mask() {
    let database = Database::new();

    let client = ClientBuilder::new()
        .nickname("nickAname".to_string())
        .username("userBname".to_string())
        .hostname("hostCname".to_string())
        .servername("serverDname".to_string())
        .realname("realEname".to_string())
        .stream(MockTcpStream::new())
        .build()
        .unwrap();

    let clientinfo = client.get_info();

    database.add_client(client);

    database.add_client(dummy_client("othernick"));

    let expected = vec![clientinfo];

    assert_eq!(database.get_clients_for_mask("*A*"), expected);
    assert_eq!(database.get_clients_for_mask("*B*"), expected);
    assert_eq!(database.get_clients_for_mask("*C*"), expected);
    assert_eq!(database.get_clients_for_mask("*D*"), expected);
    assert_eq!(database.get_clients_for_mask("*E*"), expected);
}

#[test]
fn can_get_all_clients() {
    let database = Database::new();

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
fn wildcard_pattern_works() {
    assert!(matches("hola_como_estas", "hola*estas"));
    assert!(matches("hola_como_estas", "hola*como*estas"));
    assert!(matches("hola_como_estas", "*ola*como*estas"));
    assert!(matches("hola_como_estas", "hola*como*esta*"));

    assert!(!matches("hola_como_estas", "Xola*como*estas"));
    assert!(!matches("hola_como_estas", "hola*Xomo*estas"));
    assert!(!matches("hola_como_estas", "hola*como*Xstas"));

    assert!(!matches("hola_como_estas", "ola*como*estas"));
    assert!(!matches("hola_como_estas", "hola*como*esta"));

    assert!(matches("hola_como_estas", "hola_?omo_estas"));
    assert!(matches("hola_como_estas", "hola_com?_estas"));
    assert!(matches("hola_como_estas", "hola_????_estas"));
}
