use crate::server::database::ClientBuilder;

use super::*;

#[test]
fn who_fails_with_unregistered_client() {
    let mut handler = dummy_client_handler();

    let parameters = vec![];

    handler.who_command(parameters).unwrap();

    assert_eq!(
        "200 :unregistered\r\n",
        handler.stream.read_wbuf_to_string()
    )
}

#[test]
fn who_with_no_parameters_returns_all_public_clients_with_no_common_channels() {
    let mut handler = dummy_client_handler();
    register_client(&mut handler, "nick");

    handler.database.add_client(dummy_client("nick1"));
    handler.database.add_client(dummy_client("nick2"));
    handler.database.add_client(dummy_client("nick3"));
    handler.database.add_client_to_channel("nick", "#channel");
    handler.database.add_client_to_channel("nick3", "#channel");

    let parameters = vec![];

    handler.who_command(parameters).unwrap();

    assert_eq!(
        "352 :nick1\r\n352 :nick2\r\n315 :End of /WHO list\r\n",
        handler.stream.read_wbuf_to_string()
    )
}

#[test]
fn who_with_mask_returns_all_public_clients_matching_mask() {
    let mut handler = dummy_client_handler();
    register_client(&mut handler, "local");

    handler.database.add_client(
        ClientBuilder::new()
            .nickname("nickname".to_string())
            .username("username".to_string())
            .hostname("hostname".to_string())
            .servername("servername".to_string())
            .realname("real name".to_string())
            .stream(MockTcpStream::new())
            .build()
            .unwrap(),
    );

    handler.database.add_client(
        ClientBuilder::new()
            .nickname("nickname1".to_string())
            .username("username".to_string())
            .hostname("hostname".to_string())
            .servername("servername".to_string())
            .realname("real name".to_string())
            .stream(MockTcpStream::new())
            .build()
            .unwrap(),
    );

    handler.database.add_client(
        ClientBuilder::new()
            .nickname("nickname2".to_string())
            .username("user1name".to_string())
            .hostname("hostname".to_string())
            .servername("servername".to_string())
            .realname("real name".to_string())
            .stream(MockTcpStream::new())
            .build()
            .unwrap(),
    );

    handler.database.add_client(
        ClientBuilder::new()
            .nickname("nickname3".to_string())
            .username("username".to_string())
            .hostname("host1name".to_string())
            .servername("servername".to_string())
            .realname("real name".to_string())
            .stream(MockTcpStream::new())
            .build()
            .unwrap(),
    );

    handler.database.add_client(
        ClientBuilder::new()
            .nickname("nickname4".to_string())
            .username("username".to_string())
            .hostname("hostname".to_string())
            .servername("server1name".to_string())
            .realname("real name".to_string())
            .stream(MockTcpStream::new())
            .build()
            .unwrap(),
    );

    handler.database.add_client(
        ClientBuilder::new()
            .nickname("nickname5".to_string())
            .username("username".to_string())
            .hostname("hostname".to_string())
            .servername("servername".to_string())
            .realname("real1name".to_string())
            .stream(MockTcpStream::new())
            .build()
            .unwrap(),
    );

    let parameters = vec!["*1*".to_string()];

    handler.who_command(parameters).unwrap();

    let read = handler.stream.read_wbuf_to_string();
    let mut responses = read.split("\r\n");

    assert_eq!("352 :nickname1", responses.next().unwrap());
    assert_eq!("352 :nickname2", responses.next().unwrap());
    assert_eq!("352 :nickname3", responses.next().unwrap());
    assert_eq!("352 :nickname4", responses.next().unwrap());
    assert_eq!("352 :nickname5", responses.next().unwrap());

    assert_eq!("315 *1* :End of /WHO list", responses.next().unwrap());
}
