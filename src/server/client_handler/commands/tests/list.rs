use super::*;

#[test]
fn list_fails_with_unregistered_client() {
    let mut handler = dummy_client_handler();

    let parameters = vec![];

    handler.list_command(parameters).unwrap();

    assert_eq!(
        "200 :unregistered\r\n",
        handler.stream_client_handler.read_wbuf_to_string()
    )
}

#[test]
fn list_with_no_parameters_returns_all_channels() {
    let mut handler = dummy_client_handler();
    register_client(&mut handler, "nick");

    let parameters = vec![];

    handler.list_command(parameters.clone()).unwrap();

    assert_eq!(
        "321 :Channel :Users Name\r\n322 :\r\n323 :End of /LIST\r\n",
        handler.stream_client_handler.read_wbuf_to_string()
    );

    handler.stream_client_handler.clear();

    handler.database.add_client_to_channel("nick", "#hola");
    handler.database.add_client_to_channel("nick", "#chau");

    handler.list_command(parameters).unwrap();

    assert_eq!(
        "321 :Channel :Users Name\r\n322 : #chau, #hola\r\n323 :End of /LIST\r\n",
        handler.stream_client_handler.read_wbuf_to_string()
    );
}
