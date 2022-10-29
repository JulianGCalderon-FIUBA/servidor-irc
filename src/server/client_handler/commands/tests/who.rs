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
        "352 :nick1\r\n352 :nick2\r\n315 ***** :End of /WHO list\r\n",
        handler.stream.read_wbuf_to_string()
    )
}

#[test]
fn who_with_mask_returns_all_public_clients_matching_mask() {
    let mut handler = dummy_client_handler();
    register_client(&mut handler, "nick");

    handler.database.add_client(dummy_client("ni_AA_ck1"));
    handler.database.add_client(dummy_client("ni_AA_ck2"));
    handler.database.add_client(dummy_client("nick3"));

    let parameters = vec!["*AA*".to_string()];

    handler.who_command(parameters).unwrap();

    let read = handler.stream.read_wbuf_to_string();
    let mut responses = read.split("\r\n");

    assert_eq!("352 :ni_AA_ck1", responses.next().unwrap());

    assert_eq!("352 :ni_AA_ck2", responses.next().unwrap());

    assert_eq!("315 ***** :End of /WHO list", responses.next().unwrap());
}
