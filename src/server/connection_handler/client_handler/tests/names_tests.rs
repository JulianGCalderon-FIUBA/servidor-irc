use super::*;

#[test]
fn names_with_no_valid_channels_does_not_return() {
    let mut handler = dummy_client_handler();

    let parameters = vec!["no_channel".to_string()];

    handler.names_command((None, parameters, None)).unwrap();

    assert_eq!("", handler.stream.read_wbuf_to_string())
}

#[test]
fn names_with_no_parameters_prints_all_channels() {
    let mut handler = dummy_client_handler();

    handler.database.add_local_client(dummy_client("nick2"));

    handler.database.add_client_to_channel("#hola", "nickname");
    handler.database.add_client_to_channel("#chau", "nickname");
    handler.database.add_client_to_channel("#canal", "nick2");

    let parameters = vec![];
    handler.names_command((None, parameters, None)).unwrap();

    let mut responses = handler.stream.get_responses();

    let mut channels: Vec<String> = responses.drain(0..=2).collect();

    channels.sort();

    assert_eq!("353 #canal :nick2", channels[0]);
    assert_eq!("353 #chau :nickname", channels[1]);
    assert_eq!("353 #hola :nickname", channels[2]);
    assert_eq!("366 :End of /NAMES list", responses[0]);
}

#[test]
fn names_with_parameters_prints_requested_channels() {
    let mut handler = dummy_client_handler();

    handler.database.add_client_to_channel("#hola", "nickname");
    handler.database.add_client_to_channel("#chau", "nickname");

    let parameters = vec!["#hola,#chau".to_string()];
    handler.names_command((None, parameters, None)).unwrap();

    let responses = handler.stream.get_responses();

    assert_eq!("353 #hola :nickname", responses[0]);
    assert_eq!("366 #hola :End of /NAMES list", responses[1]);
    assert_eq!("353 #chau :nickname", responses[2]);
    assert_eq!("366 #chau :End of /NAMES list", responses[3]);
}

#[test]
fn names_ignores_invalid_channels() {
    let mut handler = dummy_client_handler();

    let parameters = vec!["#hola,#invalido,#chau".to_string()];

    handler.database.add_client_to_channel("#hola", "nickname");
    handler.database.add_client_to_channel("#chau", "nickname");

    handler.names_command((None, parameters, None)).unwrap();

    let responses = handler.stream.get_responses();

    assert_eq!("353 #hola :nickname", responses[0]);
    assert_eq!("366 #hola :End of /NAMES list", responses[1]);
    assert_eq!("353 #chau :nickname", responses[2]);
    assert_eq!("366 #chau :End of /NAMES list", responses[3]);
}

#[test]
fn name_ignores_secret_channels() {
    let mut handler = dummy_client_handler();

    handler.database.add_client_to_channel("#hola", "nickname");
    handler.database.add_client_to_channel("#chau", "nickname");

    handler.database.add_local_client(dummy_client("nick2"));
    handler.database.add_client_to_channel("#secreto", "nick2");

    handler
        .database
        .set_channel_flag("#secreto", ChannelFlag::Secret);

    let parameters = vec!["#hola,#secreto,#chau".to_string()];
    handler.names_command((None, parameters, None)).unwrap();

    let responses = handler.stream.get_responses();

    assert_eq!("353 #hola :nickname", responses[0]);
    assert_eq!("366 #hola :End of /NAMES list", responses[1]);
    assert_eq!("353 #chau :nickname", responses[2]);
    assert_eq!("366 #chau :End of /NAMES list", responses[3]);
}

#[test]
fn name_ignores_private_channels() {
    let mut handler = dummy_client_handler();

    handler.database.add_client_to_channel("#hola", "nickname");
    handler.database.add_channel_operator("#hola", "nickname");
    handler.database.add_client_to_channel("#chau", "nickname");
    handler.database.add_channel_operator("#chau", "nickname");

    handler.database.add_local_client(dummy_client("nick2"));
    handler.database.add_client_to_channel("#privado", "nick2");

    handler
        .database
        .set_channel_flag("#privado", ChannelFlag::Private);

    let parameters = vec!["#hola,#privado,#chau".to_string()];
    handler.names_command((None, parameters, None)).unwrap();

    let responses = handler.stream.get_responses();

    assert_eq!("353 #hola :@nickname", responses[0]);
    assert_eq!("366 #hola :End of /NAMES list", responses[1]);
    assert_eq!("353 #chau :@nickname", responses[2]);
    assert_eq!("366 #chau :End of /NAMES list", responses[3]);
}

#[test]
fn name_prints_secret_channel_if_client_is_in_it() {
    let mut handler = dummy_client_handler();

    handler.database.add_client_to_channel("#hola", "nickname");
    handler.database.add_client_to_channel("#chau", "nickname");
    handler
        .database
        .add_client_to_channel("#secreto", "nickname");

    handler
        .database
        .set_channel_flag("#secreto", ChannelFlag::Secret);

    let parameters = vec!["#hola,#secreto,#chau".to_string()];
    handler.names_command((None, parameters, None)).unwrap();

    let responses = handler.stream.get_responses();

    assert_eq!("353 #hola :nickname", responses[0]);
    assert_eq!("366 #hola :End of /NAMES list", responses[1]);
    assert_eq!("353 #secreto :nickname", responses[2]);
    assert_eq!("366 #secreto :End of /NAMES list", responses[3]);
    assert_eq!("353 #chau :nickname", responses[4]);
    assert_eq!("366 #chau :End of /NAMES list", responses[5]);
}

#[test]
fn name_prints_private_channel_if_client_is_in_it() {
    let mut handler = dummy_client_handler();

    handler.database.add_client_to_channel("#hola", "nickname");
    handler.database.add_client_to_channel("#chau", "nickname");
    handler
        .database
        .add_client_to_channel("#privado", "nickname");

    handler.database.add_channel_operator("#hola", "nickname");

    handler
        .database
        .set_channel_flag("#privado", ChannelFlag::Private);

    let parameters = vec!["#hola,#privado,#chau".to_string()];
    handler.names_command((None, parameters, None)).unwrap();

    let responses = handler.stream.get_responses();

    assert_eq!("353 #hola :@nickname", responses[0]);
    assert_eq!("366 #hola :End of /NAMES list", responses[1]);
    assert_eq!("353 #privado :nickname", responses[2]);
    assert_eq!("366 #privado :End of /NAMES list", responses[3]);
    assert_eq!("353 #chau :nickname", responses[4]);
    assert_eq!("366 #chau :End of /NAMES list", responses[5]);
}

#[test]
fn names_returns_clients_in_no_channels_as_being_on_wildcard_channel() {
    let mut handler = dummy_client_handler();

    handler.database.add_local_client(dummy_client("nick2"));
    handler.database.add_client_to_channel("#hola", "nickname");

    handler.names_command((None, vec![], None)).unwrap();

    let responses = handler.stream.get_responses();

    assert_eq!("353 #hola :nickname", responses[0]);
    assert_eq!("353 * :nick2", responses[1]);
    assert_eq!("366 :End of /NAMES list", responses[2]);
}

#[test]
fn names_returns_clients_in_not_visible_private_channel_as_being_on_wildcard_channel() {
    let mut handler = dummy_client_handler();

    handler.database.add_local_client(dummy_client("nick2"));
    handler.database.add_client_to_channel("#hola", "nickname");

    handler.database.add_client_to_channel("#hola2", "nick2");
    handler
        .database
        .set_channel_flag("#hola2", ChannelFlag::Private);

    handler.names_command((None, vec![], None)).unwrap();

    let responses = handler.stream.get_responses();

    assert_eq!("353 #hola :nickname", responses[0]);
    assert_eq!("353 * :nick2", responses[1]);
    assert_eq!("366 :End of /NAMES list", responses[2]);
}

#[test]
fn names_returns_clients_in_not_visible_secret_channel_as_being_on_wildcard_channel() {
    let mut handler = dummy_client_handler();

    handler.database.add_local_client(dummy_client("nick2"));
    handler.database.add_client_to_channel("#hola", "nickname");

    handler.database.add_client_to_channel("#hola2", "nick2");
    handler
        .database
        .set_channel_flag("#hola2", ChannelFlag::Secret);

    handler.names_command((None, vec![], None)).unwrap();

    let responses = handler.stream.get_responses();

    assert_eq!("353 #hola :nickname", responses[0]);
    assert_eq!("353 * :nick2", responses[1]);
    assert_eq!("366 :End of /NAMES list", responses[2]);
}

#[test]
fn names_returns_all_channel_clients() {
    let mut handler = dummy_client_handler();

    handler.database.add_local_client(dummy_client("nick2"));
    handler.database.add_client_to_channel("#hola", "nickname");
    handler.database.add_client_to_channel("#hola", "nick2");

    handler
        .names_command((None, vec!["#hola".to_string()], None))
        .unwrap();

    let responses = handler.stream.get_responses();

    assert_eq!("353 #hola :nickname nick2", responses[0]);
    assert_eq!("366 #hola :End of /NAMES list", responses[1]);
}

#[test]
fn names_returns_client_role() {
    let mut handler = dummy_client_handler();

    handler.database.add_local_client(dummy_client("nick2"));
    handler.database.add_local_client(dummy_client("nick3"));
    handler.database.add_client_to_channel("#hola", "nickname");
    handler.database.add_client_to_channel("#hola", "nick2");
    handler.database.add_client_to_channel("#hola", "nick3");

    handler.database.add_channel_operator("#hola", "nickname");

    handler
        .database
        .set_channel_flag("#hola", ChannelFlag::Moderated);
    handler.database.add_channel_speaker("#hola", "nick2");

    handler
        .names_command((None, vec!["#hola".to_string()], None))
        .unwrap();

    let responses = handler.stream.get_responses();

    assert_eq!("353 #hola :@nickname +nick2 nick3", responses[0]);
    assert_eq!("366 #hola :End of /NAMES list", responses[1]);
}
