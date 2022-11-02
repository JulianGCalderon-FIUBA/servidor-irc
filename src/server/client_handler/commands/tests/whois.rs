use super::*;

#[test]
fn whois_fails_with_unregistered_client() {
    let mut handler = dummy_client_handler();

    let parameters = vec!["server".to_string(), "nick".to_string()];

    handler.whois_command(parameters).unwrap();

    assert_eq!(
        "200 :unregistered\r\n",
        handler.stream.read_wbuf_to_string()
    )
}

#[test]
fn whois_fails_with_empty_params() {
    let mut handler = dummy_client_handler();
    let parameters = vec![];

    handler.whois_command(parameters).unwrap();

    assert_eq!(
        "431 :no nickname given\r\n",
        handler.stream.read_wbuf_to_string()
    )
}

#[test]
fn whois_fails_with_nonexistent_nickname() {
    let mut handler = dummy_client_handler();
    register_client(&mut handler, "nick");

    let parameters: Vec<String> = vec!["nick2".to_string()];

    handler.whois_command(parameters).unwrap();

    assert_eq!(
        "401 nick2 :No such nick/channel\r\n",
        handler.stream.read_wbuf_to_string()
    )
}

#[test]
fn whois_returns_nick_info() {
    let mut handler = dummy_client_handler();
    register_client(&mut handler, "nick");

    let parameters = vec!["nick".to_string()];

    handler.whois_command(parameters).unwrap();

    let responses = handler.stream.get_responses();

    assert_eq!("311 nick username hostname *: realname", responses[0]);
    assert_eq!("318 nick :End of /WHOIS list", responses[1]);
}

#[test]
fn whois_returns_nick_info_if_oper() {
    let mut handler = dummy_client_handler();
    register_client(&mut handler, "nick");

    let parameters = vec!["nick".to_string()];

    let parameters_oper = vec!["admin".to_string(), "admin".to_string()];

    handler.oper_command(parameters_oper).unwrap();

    handler.stream.clear();

    handler.whois_command(parameters).unwrap();

    let responses = handler.stream.get_responses();

    assert_eq!("311 nick username hostname *: realname", responses[0]);
    assert_eq!("313 nick :is an IRC operator", responses[1]);
    assert_eq!("318 nick :End of /WHOIS list", responses[2]);
}

#[test]
fn whois_returns_nick_info_with_channels() {
    let mut handler = dummy_client_handler();
    register_client(&mut handler, "nick");

    let parameters = vec!["nick".to_string()];

    handler.database.add_client_to_channel("nick", "#hola");

    handler.whois_command(parameters).unwrap();

    let responses = handler.stream.get_responses();

    assert_eq!("311 nick username hostname *: realname", responses[0]);
    assert_eq!("319 nick : #hola", responses[1]);
    assert_eq!("318 nick :End of /WHOIS list", responses[2]);
}

#[test]
fn whois_returns_complete_nick_info() {
    let mut handler = dummy_client_handler();
    register_client(&mut handler, "nick");

    let parameters = vec!["nick".to_string()];

    handler.database.add_client_to_channel("nick", "#hola");

    let parameters_oper = vec!["admin".to_string(), "admin".to_string()];

    handler.oper_command(parameters_oper).unwrap();

    handler.stream.clear();

    handler.whois_command(parameters).unwrap();

    let responses = handler.stream.get_responses();

    assert_eq!("311 nick username hostname *: realname", responses[0]);
    assert_eq!("313 nick :is an IRC operator", responses[1]);
    assert_eq!("319 nick : #hola", responses[2]);
    assert_eq!("318 nick :End of /WHOIS list", responses[3]);
}

#[test]
fn whois_works_with_nickmask() {
    let mut handler = dummy_client_handler();
    register_client(&mut handler, "nick");

    let parameters = vec!["nic*".to_string()];

    handler.database.add_client(dummy_client("nick2"));
    handler.database.add_client(dummy_client("nick3"));

    handler.whois_command(parameters).unwrap();

    let responses = handler.stream.get_responses();

    assert_eq!("311 nick username hostname *: realname", responses[0]);
    assert_eq!("318 nick :End of /WHOIS list", responses[1]);
    assert_eq!("311 nick2 username hostname *: realname", responses[2]);
    assert_eq!("318 nick2 :End of /WHOIS list", responses[3]);
    assert_eq!("311 nick3 username hostname *: realname", responses[4]);
    assert_eq!("318 nick3 :End of /WHOIS list", responses[5]);
}
