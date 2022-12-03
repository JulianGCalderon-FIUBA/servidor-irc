use crate::server::consts::user_flag::UserFlag;
use crate::server::testing::dummy_external_client;

use super::*;

#[test]
fn who_with_no_parameters_returns_all_public_clients_with_no_common_channels() {
    let mut handler = dummy_client_handler();

    handler.database.add_local_client(dummy_client("nick1"));
    handler.database.add_local_client(dummy_client("nick2"));
    handler.database.add_local_client(dummy_client("nick3"));
    handler
        .database
        .add_client_to_channel("#channel", "nickname");
    handler.database.add_client_to_channel("#channel", "nick3");
    handler.database.add_client_to_channel("#channel2", "nick1");

    let parameters = vec![];

    handler.who_command((None, parameters, None)).unwrap();

    let mut responses = handler.stream.get_responses();

    let mut channels: Vec<String> = responses.drain(0..=1).collect();

    channels.sort();

    assert_eq!(
        "352 #channel2 username 127.0.0.1 servername nick1 :0 realname",
        channels[0]
    );
    assert_eq!(
        "352 * username 127.0.0.1 servername nick2 :0 realname",
        channels[1]
    );
    assert_eq!("315 :End of /WHO list", responses[0]);
}

#[test]
fn who_with_mask_returns_all_public_clients_matching_mask() {
    let mut handler = dummy_client_handler();

    handler.database.add_local_client(dummy_client("nick1name"));

    let parameters = vec!["*k1*".to_string()];

    handler.who_command((None, parameters, None)).unwrap();

    let responses = handler.stream.get_responses();

    assert_eq!(
        "352 * username 127.0.0.1 servername nick1name :0 realname",
        responses[0]
    );
    assert_eq!("315 *k1* :End of /WHO list", responses[1]);
}

#[test]
fn who_with_external_clients_returns_client_correct_hopcount() {
    let mut handler = dummy_client_handler();

    handler
        .database
        .add_external_client(dummy_external_client("nick1name", "servername1"));

    let parameters = vec!["*k1*".to_string()];

    handler.who_command((None, parameters, None)).unwrap();

    let responses = handler.stream.get_responses();

    assert_eq!(
        "352 * username 127.0.0.1 servername1 nick1name :1 realname",
        responses[0]
    );
    assert_eq!("315 *k1* :End of /WHO list", responses[1]);
}

#[test]
fn who_returns_client_masks() {
    let mut handler = dummy_client_handler();

    handler.database.add_local_client(dummy_client("nick1name"));

    let parameters = vec!["*k1*".to_string()];

    handler
        .database
        .set_user_flag("nick1name", UserFlag::Invisible);
    handler
        .database
        .set_user_flag("nick1name", UserFlag::Operator);

    handler.who_command((None, parameters, None)).unwrap();

    let responses = handler.stream.get_responses();

    let mut first_response: Vec<String> = responses[0].split(' ').map(String::from).collect();
    let mut flags = first_response[6].chars().collect::<Vec<char>>();
    flags.sort();
    first_response[6] = flags.iter().collect();
    let first_response = first_response.join(" ");

    assert_eq!(
        "352 * username 127.0.0.1 servername nick1name io :0 realname",
        first_response
    );
    assert_eq!("315 *k1* :End of /WHO list", responses[1]);
}
