use super::*;

#[test]
fn names_fails_with_unregistered_client() {
    let mut handler = dummy_client_handler();

    let parameters = vec![];

    handler.names_command(parameters).unwrap();

    assert_eq!(
        "200 :unregistered\r\n",
        handler.stream_client_handler.read_wbuf_to_string()
    )
}

#[test]
fn names_with_no_channels_prints_end_of_names() {
    let mut handler = dummy_client_handler();
    register_client(&mut handler, "nick");

    let parameters = vec![];

    handler.names_command(parameters).unwrap();

    assert_eq!(
        "366 :End of /NAMES list\r\n",
        handler.stream_client_handler.read_wbuf_to_string()
    )
}

#[test]
fn names_with_no_parameters_prints_all_channels() {
    let mut handler = dummy_client_handler();
    register_client(&mut handler, "nick");

    let parameters = vec![];

    handler.database.add_client_to_channel("nick", "#hola");
    handler.database.add_client_to_channel("nick", "#chau");

    handler.names_command(parameters.clone()).unwrap();

    assert_eq!(
        "353 #chau :nick\r\n353 #hola :nick\r\n366 :End of /NAMES list\r\n",
        handler.stream_client_handler.read_wbuf_to_string()
    );

    handler.database.add_client_to_channel("nick2", "#canal");
    handler.stream_client_handler.clear();

    handler.list_command(parameters).unwrap();

    assert_eq!(
        "321 :Channel :Users Name\r\n322 : #canal #chau #hola\r\n323 :End of /LIST\r\n",
        handler.stream_client_handler.read_wbuf_to_string()
    );
}
