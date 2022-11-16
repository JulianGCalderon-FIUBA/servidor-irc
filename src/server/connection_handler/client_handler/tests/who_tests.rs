use super::*;

#[test]
fn who_with_no_parameters_returns_all_public_clients_with_no_common_channels() {
    let mut handler = dummy_client_handler();

    handler.database.add_client(dummy_client("nick1"));
    handler.database.add_client(dummy_client("nick2"));
    handler.database.add_client(dummy_client("nick3"));
    handler
        .database
        .add_client_to_channel("nickname", "#channel");
    handler.database.add_client_to_channel("nick3", "#channel");
    handler.database.add_client_to_channel("nick1", "#channel2");

    let parameters = vec![];

    handler.who_command(parameters).unwrap();

    let responses = handler.stream.get_responses();

    assert_eq!(
        "352 #channel2 username 127.0.0.1 servername nick1 \\MODOS :HOPCOUNT realname",
        responses[0]
    );
    assert_eq!(
        "352 * username 127.0.0.1 servername nick2 \\MODOS :HOPCOUNT realname",
        responses[1]
    );
    assert_eq!("315 :End of /WHO list", responses[2]);
}

#[test]
fn who_with_mask_returns_all_public_clients_matching_mask() {
    let mut handler = dummy_client_handler();

    handler.database.add_client(dummy_client("nick1name"));

    let parameters = vec!["*k1*".to_string()];

    handler.who_command(parameters).unwrap();

    let responses = handler.stream.get_responses();

    assert_eq!(
        "352 * username 127.0.0.1 servername nick1name \\MODOS :HOPCOUNT realname",
        responses[0]
    );
    assert_eq!("315 *k1* :End of /WHO list", responses[1]);
}
