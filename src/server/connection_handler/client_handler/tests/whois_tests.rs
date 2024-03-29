use crate::server::testing::{dummy_external_client, dummy_server};

use super::*;

#[test]
fn whois_fails_with_empty_params() {
    let mut handler = dummy_client_handler();
    let parameters = vec![];

    handler.whois_command((None, parameters, None)).unwrap();

    assert_eq!(
        "431 :No nickname given\r\n",
        handler.stream.read_wbuf_to_string()
    )
}

#[test]
fn whois_fails_with_nonexistent_nickname() {
    let mut handler = dummy_client_handler();

    let parameters: Vec<String> = vec!["nick2".to_string()];

    handler.whois_command((None, parameters, None)).unwrap();

    assert_eq!(
        "401 nick2 :No such nick/channel\r\n",
        handler.stream.read_wbuf_to_string()
    )
}

#[test]
fn whois_returns_nick_info() {
    let mut handler = dummy_client_handler();

    let parameters = vec!["nickname".to_string()];

    handler.whois_command((None, parameters, None)).unwrap();

    let responses = handler.stream.get_responses();

    assert_eq!("311 nickname username 127.0.0.1 *: realname", responses[0]);
    assert_eq!("312 nickname servername :serverinfo", responses[1]);
    assert_eq!("318 nickname :End of /WHOIS list", responses[2]);
}

#[test]
fn whois_returns_nick_info_if_oper() {
    let mut handler = dummy_client_handler();

    handler.database.set_server_operator("nickname");

    let parameters = vec!["nickname".to_string()];

    handler.whois_command((None, parameters, None)).unwrap();

    let responses = handler.stream.get_responses();

    assert_eq!("311 nickname username 127.0.0.1 *: realname", responses[0]);
    assert_eq!("312 nickname servername :serverinfo", responses[1]);
    assert_eq!("313 nickname :Is an IRC operator", responses[2]);
    assert_eq!("318 nickname :End of /WHOIS list", responses[3]);
}

#[test]
fn whois_returns_nick_info_with_channels() {
    let mut handler = dummy_client_handler();

    let parameters = vec!["nickname".to_string()];

    handler.database.add_client_to_channel("#hola", "nickname");

    handler.whois_command((None, parameters, None)).unwrap();

    let responses = handler.stream.get_responses();

    assert_eq!("311 nickname username 127.0.0.1 *: realname", responses[0]);
    assert_eq!("312 nickname servername :serverinfo", responses[1]);
    assert_eq!("319 nickname : #hola", responses[2]);
    assert_eq!("318 nickname :End of /WHOIS list", responses[3]);
}

#[test]
fn whois_returns_nick_away_info() {
    let mut handler = dummy_client_handler();

    handler
        .database
        .set_away_message("nickname", Some("away".to_string()));

    let parameters = vec!["nickname".to_string()];

    handler.whois_command((None, parameters, None)).unwrap();

    let responses = handler.stream.get_responses();

    assert_eq!("311 nickname username 127.0.0.1 *: realname", responses[0]);
    assert_eq!("312 nickname servername :serverinfo", responses[1]);
    assert_eq!("301 nickname :away", responses[2]);
    assert_eq!("318 nickname :End of /WHOIS list", responses[3]);
}

#[test]
fn whois_returns_complete_nick_info() {
    let mut handler = dummy_client_handler();

    let parameters = vec!["nickname".to_string()];

    handler.database.add_client_to_channel("#hola", "nickname");
    handler.database.set_server_operator("nickname");
    handler
        .database
        .set_away_message("nickname", Some("away message".to_string()));

    handler.whois_command((None, parameters, None)).unwrap();

    let responses = handler.stream.get_responses();

    assert_eq!("311 nickname username 127.0.0.1 *: realname", responses[0]);
    assert_eq!("312 nickname servername :serverinfo", responses[1]);
    assert_eq!("313 nickname :Is an IRC operator", responses[2]);
    assert_eq!("319 nickname : #hola", responses[3]);
    assert_eq!("301 nickname :away message", responses[4]);
    assert_eq!("318 nickname :End of /WHOIS list", responses[5]);
}

#[test]
fn whois_works_with_nickmask() {
    let mut handler = dummy_client_handler();

    let parameters = vec!["nic*".to_string()];

    handler.database.add_local_client(dummy_client("nickname2"));
    handler.database.add_local_client(dummy_client("nickname3"));

    handler.whois_command((None, parameters, None)).unwrap();

    let mut responses = handler.stream.get_responses();

    let mut sorted_responses: Vec<Vec<String>> = vec![];

    for _ in 0..3 {
        sorted_responses.push(responses.drain(0..=2).collect())
    }

    sorted_responses.sort();

    assert_eq!(
        "311 nickname username 127.0.0.1 *: realname",
        sorted_responses[0][0]
    );
    assert_eq!(
        "312 nickname servername :serverinfo",
        sorted_responses[0][1]
    );
    assert_eq!("318 nickname :End of /WHOIS list", sorted_responses[0][2]);

    assert_eq!(
        "311 nickname2 username 127.0.0.1 *: realname",
        sorted_responses[1][0]
    );
    assert_eq!(
        "312 nickname2 servername :serverinfo",
        sorted_responses[1][1]
    );
    assert_eq!("318 nickname2 :End of /WHOIS list", sorted_responses[1][2]);

    assert_eq!(
        "311 nickname3 username 127.0.0.1 *: realname",
        sorted_responses[2][0]
    );
    assert_eq!(
        "312 nickname3 servername :serverinfo",
        sorted_responses[2][1]
    );
    assert_eq!("318 nickname3 :End of /WHOIS list", sorted_responses[2][2]);
}

#[test]
fn whois_fails_with_unknown_server() {
    let mut handler = dummy_client_handler();
    let parameters = vec!["server1".to_string(), "nickname".to_string()];

    handler.whois_command((None, parameters, None)).unwrap();

    assert_eq!(
        "402 server1 :No such server\r\n",
        handler.stream.read_wbuf_to_string()
    )
}

#[test]
fn whois_with_server_param_only_sends_response_for_clients_in_server() {
    let mut handler = dummy_client_handler();

    handler
        .database()
        .add_immediate_server(dummy_server("servername2"));
    let client = dummy_external_client("nickname2", "servername2");
    handler.database.add_external_client(client);

    let parameters = vec!["servername2".to_string(), "nickna*".to_string()];

    handler.whois_command((None, parameters, None)).unwrap();

    let responses = handler.stream.get_responses();

    assert_eq!("311 nickname2 username 127.0.0.1 *: realname", responses[0]);
    assert_eq!("312 nickname2 servername2 :serverinfo", responses[1]);
    assert_eq!("318 nickname2 :End of /WHOIS list", responses[2]);
}
