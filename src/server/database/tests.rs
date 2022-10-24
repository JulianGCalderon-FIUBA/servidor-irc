use std::{fs::File, os::unix::prelude::OwnedFd};

use super::*;

fn dummy_client_w_nick(nickname: &str) -> ClientInfo {
    ClientInfoBuilder::new_with(
        nickname.to_string(),
        "username".to_string(),
        "hostname".to_string(),
        "servername".to_string(),
        "real name".to_string(),
    )
    .build()
}

fn add_stream_to_clientinfo(client_info: &mut ClientInfo, stream: TcpStream) {
    let stream = Arc::new(Mutex::new(stream));
    client_info.stream = Some(stream);
}

fn tcp_stream_from_file(path: &str) -> TcpStream {
    let file = File::open(path).unwrap();
    let owned_fd = OwnedFd::from(file);
    TcpStream::from(owned_fd)
}

#[test]
fn add_client() {
    let database = Database::new();

    assert!(!database.contains_client("nickname"));
    database.add_client(dummy_client_w_nick("nickname"));
    assert!(database.contains_client("nickname"));

    assert!(!database.contains_client("nickname2"));
    database.add_client(dummy_client_w_nick("nickname2"));
    assert!(database.contains_client("nickname2"));
}

#[test]
fn server_operator() {
    let database = Database::new();

    database.add_client(dummy_client_w_nick("nickname"));

    assert!(!database._is_server_operator("nickname"));
    database.set_server_operator("nickname");
    assert!(database._is_server_operator("nickname"));
}

#[test]
fn get_stream() {
    let database = Database::new();

    let mut client = dummy_client_w_nick("nickname");
    let stream = tcp_stream_from_file("assets/dummy_stream");
    add_stream_to_clientinfo(&mut client, stream);

    let stream_ref_expected = client.get_stream().unwrap();

    database.add_client(client);
    let stream_ref_actual = database.get_stream("nickname").unwrap();

    assert!(Arc::ptr_eq(&stream_ref_expected, &stream_ref_actual));
}

#[test]
fn disconnect_client() {
    let database = Database::new();

    let mut client = dummy_client_w_nick("nickname");
    let stream = tcp_stream_from_file("assets/dummy_stream");

    add_stream_to_clientinfo(&mut client, stream);

    database.add_client(client);

    assert!(database.get_stream("nickname").is_some());
    database.disconnect_client("nickname");
    assert!(database.get_stream("nickname").is_none());
}

#[test]
fn add_client_to_channel() {
    let database = Database::new();

    let client = dummy_client_w_nick("nickname1");
    database.add_client(client);

    let client = dummy_client_w_nick("nickname2");
    database.add_client(client);

    assert!(!database.contains_channel("channel"));
    database.add_client_to_channel("nickname1", "channel");
    assert!(database.contains_channel("channel"));

    database.add_client_to_channel("nickname2", "channel");

    assert_eq!(
        database.get_clients("channel"),
        vec!["nickname1".to_string(), "nickname2".to_string()]
    )
}

#[test]
fn remove_client_from_channel() {
    let database = Database::new();

    let client = dummy_client_w_nick("nickname1");
    database.add_client(client);
    database.add_client_to_channel("nickname1", "channel");

    let client = dummy_client_w_nick("nickname2");
    database.add_client(client);
    database.add_client_to_channel("nickname2", "channel");

    database.remove_client_of_channel("nickname1", "channel");

    assert_eq!(
        database.get_clients("channel"),
        vec!["nickname2".to_string()]
    );

    database.remove_client_of_channel("nickname2", "channel");

    assert!(!database.contains_channel("channel"));
}

#[test]
fn is_client_in_channel() {
    let database = Database::new();

    let client = dummy_client_w_nick("nickname");
    database.add_client(client);
    database.add_client_to_channel("nickname", "channel");

    assert!(database.is_client_in_channel("nickname", "channel"));
}

#[test]
fn get_channels() {
    let database = Database::new();

    let client = dummy_client_w_nick("nickname");
    database.add_client(client);
    database.add_client_to_channel("nickname", "channel1");
    database.add_client_to_channel("nickname", "channel2");

    let mut channels_real = database.get_channels();
    let channels_expected = vec!["channel1".to_string(), "channel2".to_string()];

    channels_real.sort();

    assert_eq!(channels_real, channels_expected);
}

#[test]
fn get_channels_for_client() {
    let database = Database::new();

    let client = dummy_client_w_nick("nickname");
    database.add_client(client);
    database.add_client_to_channel("nickname", "channel1");
    database.add_client_to_channel("nickname", "channel2");

    let mut channels_real = database.get_channels_for_client("nickname");
    let channels_expected = vec!["channel1".to_string(), "channel2".to_string()];

    channels_real.sort();

    assert_eq!(channels_expected, channels_real);
}
