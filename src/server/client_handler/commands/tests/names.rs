use super::*;

#[test]
fn names_fails_with_unregistered_client() {
    let mut handler = dummy_client_handler();

    let parameters = vec![];

    handler.names_command(parameters).unwrap();

    assert_eq!(
        "200 :Unregistered\r\n",
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

    handler.database.add_client(dummy_client("nick2"));

    handler.database.add_client_to_channel("nick", "#hola");
    handler.database.add_client_to_channel("nick", "#chau");
    handler.database.add_client_to_channel("nick2", "#canal");

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

    handler.database.add_client_to_channel("nick", "#hola");
    handler.database.add_client_to_channel("nick", "#chau");

    let parameters = vec!["#hola,#chau".to_string()];
    handler.names_command(parameters).unwrap();

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

#[test]
fn name_ignores_secret_channels() {
    let mut handler = dummy_client_handler();
    register_client(&mut handler, "nick");

    handler.database.add_client_to_channel("nick", "#hola");
    handler.database.add_client_to_channel("nick", "#chau");

    handler.database.add_client(dummy_client("nick2"));
    handler.database.add_client_to_channel("nick2", "#secreto");

    handler.database.set_channel_mode("#secreto", 's');

    let parameters = vec!["#hola,#secreto,#chau".to_string()];
    handler.names_command(parameters).unwrap();

    let responses = handler.stream.get_responses();

    assert_eq!("353 #hola :nick", responses[0]);
    assert_eq!("366 #hola :End of /NAMES list", responses[1]);
    assert_eq!("353 #chau :nick", responses[2]);
    assert_eq!("366 #chau :End of /NAMES list", responses[3]);
}

#[test]
fn name_ignores_private_channels() {
    let mut handler = dummy_client_handler();
    register_client(&mut handler, "nick");

    handler.database.add_client_to_channel("nick", "#hola");
    handler.database.add_client_to_channel("nick", "#chau");

    handler.database.add_client(dummy_client("nick2"));
    handler.database.add_client_to_channel("nick2", "#privado");

    handler.database.set_channel_mode("#privado", 'p');

    let parameters = vec!["#hola,#privado,#chau".to_string()];
    handler.names_command(parameters).unwrap();

    let responses = handler.stream.get_responses();

    assert_eq!("353 #hola :nick", responses[0]);
    assert_eq!("366 #hola :End of /NAMES list", responses[1]);
    assert_eq!("353 #chau :nick", responses[2]);
    assert_eq!("366 #chau :End of /NAMES list", responses[3]);
}

#[test]
fn name_prints_secret_channel_if_client_is_in_it() {
    let mut handler = dummy_client_handler();
    register_client(&mut handler, "nick");

    handler.database.add_client_to_channel("nick", "#hola");
    handler.database.add_client_to_channel("nick", "#chau");
    handler.database.add_client_to_channel("nick", "#secreto");

    handler.database.set_channel_mode("#secreto", 's');

    let parameters = vec!["#hola,#secreto,#chau".to_string()];
    handler.names_command(parameters).unwrap();

    let responses = handler.stream.get_responses();

    assert_eq!("353 #hola :nick", responses[0]);
    assert_eq!("366 #hola :End of /NAMES list", responses[1]);
    assert_eq!("353 #secreto :nick", responses[2]);
    assert_eq!("366 #secreto :End of /NAMES list", responses[3]);
    assert_eq!("353 #chau :nick", responses[4]);
    assert_eq!("366 #chau :End of /NAMES list", responses[5]);
}

#[test]
fn name_prints_private_channel_if_client_is_in_it() {
    let mut handler = dummy_client_handler();
    register_client(&mut handler, "nick");

    handler.database.add_client_to_channel("nick", "#hola");
    handler.database.add_client_to_channel("nick", "#chau");
    handler.database.add_client_to_channel("nick", "#privado");

    handler.database.set_channel_mode("#privado", 'p');

    let parameters = vec!["#hola,#privado,#chau".to_string()];
    handler.names_command(parameters).unwrap();

    let responses = handler.stream.get_responses();

    assert_eq!("353 #hola :nick", responses[0]);
    assert_eq!("366 #hola :End of /NAMES list", responses[1]);
    assert_eq!("353 #privado :nick", responses[2]);
    assert_eq!("366 #privado :End of /NAMES list", responses[3]);
    assert_eq!("353 #chau :nick", responses[4]);
    assert_eq!("366 #chau :End of /NAMES list", responses[5]);
}
