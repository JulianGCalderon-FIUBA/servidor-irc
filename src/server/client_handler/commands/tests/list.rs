use super::*;

#[test]
fn list_fails_with_unregistered_client() {
    let mut handler = dummy_client_handler();

    let parameters = vec![];

    handler.list_command(parameters).unwrap();

    assert_eq!(
        "200 :unregistered\r\n",
        handler.stream.read_wbuf_to_string()
    )
}

#[test]
fn list_with_no_channels_prints_start_and_end() {
    let mut handler = dummy_client_handler();
    register_client(&mut handler, "nick");

    let parameters = vec![];

    handler.list_command(parameters).unwrap();

    assert_eq!(
        "321 :Channel :Users Name\r\n323 :End of /LIST\r\n",
        handler.stream.read_wbuf_to_string()
    )
}

#[test]
fn list_with_no_parameters_prints_all_channels() {
    let mut handler = dummy_client_handler();
    register_client(&mut handler, "nick");

    let parameters = vec![];

    handler.database.add_client_to_channel("nick", "#hola");
    handler.database.add_client_to_channel("nick", "#chau");

    handler.list_command(parameters.clone()).unwrap();

    assert_eq!(
        "321 :Channel :Users Name\r\n322 : #chau #hola\r\n323 :End of /LIST\r\n",
        handler.stream.read_wbuf_to_string()
    );

    handler.database.add_client_to_channel("nick2", "#canal");
    handler.stream.clear();

    handler.list_command(parameters).unwrap();

    assert_eq!(
        "321 :Channel :Users Name\r\n322 : #canal #chau #hola\r\n323 :End of /LIST\r\n",
        handler.stream.read_wbuf_to_string()
    );
}

#[test]
fn list_with_parameters_prints_requested_channels() {
    let mut handler = dummy_client_handler();
    register_client(&mut handler, "nick");

    let parameters = vec!["#hola".to_string()];

    handler.database.add_client_to_channel("nick", "#hola");
    handler.database.add_client_to_channel("nick", "#chau");

    handler.list_command(parameters).unwrap();

    assert_eq!(
        "321 :Channel :Users Name\r\n322 : #hola\r\n323 :End of /LIST\r\n",
        handler.stream.read_wbuf_to_string()
    );

    handler.stream.clear();

    let parameters2 = vec!["#hola,#chau".to_string()];

    handler.list_command(parameters2).unwrap();

    assert_eq!(
        "321 :Channel :Users Name\r\n322 : #hola #chau\r\n323 :End of /LIST\r\n",
        handler.stream.read_wbuf_to_string()
    );
}

#[test]
fn list_ignores_invalid_channels() {
    let mut handler = dummy_client_handler();
    register_client(&mut handler, "nick");

    let parameters = vec!["#hola,#invalido,#chau".to_string()];

    handler.database.add_client_to_channel("nick", "#hola");
    handler.database.add_client_to_channel("nick", "#chau");

    handler.list_command(parameters).unwrap();

    assert_eq!(
        "321 :Channel :Users Name\r\n322 : #hola #chau\r\n323 :End of /LIST\r\n",
        handler.stream.read_wbuf_to_string()
    );
}
