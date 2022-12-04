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

    let parameters = vec!["nickname".to_string()];

    let parameters_oper = vec!["admin".to_string(), "admin".to_string()];

    handler.oper_command((None, parameters_oper, None)).unwrap();

    handler.stream.clear();

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

    handler.database.add_client_to_channel("nickname", "#hola");

    handler.whois_command((None, parameters, None)).unwrap();

    let responses = handler.stream.get_responses();

    assert_eq!("311 nickname username 127.0.0.1 *: realname", responses[0]);
    assert_eq!("312 nickname servername :serverinfo", responses[1]);
    assert_eq!("319 nickname : @#hola", responses[2]);
    assert_eq!("318 nickname :End of /WHOIS list", responses[3]);
}

#[test]
fn whois_returns_complete_nick_info() {
    let mut handler = dummy_client_handler();

    let parameters = vec!["nickname".to_string()];

    handler.database.add_client_to_channel("nickname", "#hola");

    let parameters_oper = vec!["admin".to_string(), "admin".to_string()];

    handler.oper_command((None, parameters_oper, None)).unwrap();

    handler.stream.clear();

    handler.whois_command((None, parameters, None)).unwrap();

    let responses = handler.stream.get_responses();

    assert_eq!("311 nickname username 127.0.0.1 *: realname", responses[0]);
    assert_eq!("312 nickname servername :serverinfo", responses[1]);
    assert_eq!("313 nickname :Is an IRC operator", responses[2]);
    assert_eq!("319 nickname : @#hola", responses[3]);
    assert_eq!("318 nickname :End of /WHOIS list", responses[4]);
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
