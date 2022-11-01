use super::*;

#[test]
fn names_fails_with_unregistered_client() {
    let mut handler = dummy_client_handler();

    let parameters = vec![];

    handler.names_command(parameters).unwrap();

    assert_eq!(
        "200 :unregistered\r\n",
        handler.stream.read_wbuf_to_string()
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
        handler.stream.read_wbuf_to_string()
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
        handler.stream.read_wbuf_to_string()
    );

    handler.database.add_client_to_channel("nick2", "#canal");
    handler.stream.clear();

    handler.names_command(parameters).unwrap();

    let responses = handler.stream.get_responses();

    assert_eq!("353 #canal :nick2", responses[0]);
    assert_eq!("353 #chau :nick", responses[1]);
    assert_eq!("353 #hola :nick", responses[2]);
    assert_eq!("366 :End of /NAMES list", responses[3]);
}

#[test]
fn names_with_parameters_prints_requested_channels() {
    let mut handler = dummy_client_handler();
    register_client(&mut handler, "nick");

    let parameters = vec!["#hola".to_string()];

    handler.database.add_client_to_channel("nick", "#hola");
    handler.database.add_client_to_channel("nick", "#chau");

    handler.names_command(parameters).unwrap();

    assert_eq!(
        "353 #hola :nick\r\n366 #hola :End of /NAMES list\r\n",
        handler.stream.read_wbuf_to_string()
    );

    handler.stream.clear();

    let parameters2 = vec!["#hola,#chau".to_string()];

    handler.names_command(parameters2).unwrap();

    let responses = handler.stream.get_responses();

    assert_eq!("353 #hola :nick", responses[0]);
    assert_eq!("366 #hola :End of /NAMES list", responses[1]);
    assert_eq!("353 #chau :nick", responses[2]);
    assert_eq!("366 #chau :End of /NAMES list", responses[3]);
}

#[test]
fn names_ignores_invalid_channels() {
    let mut handler = dummy_client_handler();
    register_client(&mut handler, "nick");

    let parameters = vec!["#hola,#invalido,#chau".to_string()];

    handler.database.add_client_to_channel("nick", "#hola");
    handler.database.add_client_to_channel("nick", "#chau");

    handler.names_command(parameters).unwrap();

    let responses = handler.stream.get_responses();

    assert_eq!("353 #hola :nick", responses[0]);
    assert_eq!("366 #hola :End of /NAMES list", responses[1]);
    assert_eq!("353 #chau :nick", responses[2]);
    assert_eq!("366 #chau :End of /NAMES list", responses[3]);
}
