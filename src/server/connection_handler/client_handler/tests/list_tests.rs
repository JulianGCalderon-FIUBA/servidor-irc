use super::*;

#[test]
fn list_with_no_channels_prints_start_and_end() {
    let mut handler = dummy_client_handler();

    let parameters = vec![];

    handler.list_command((None, parameters, None)).unwrap();

    let responses = handler.stream.get_responses();

    assert_eq!("321 :Channel :Users Name", responses[0]);
    assert_eq!("323 :End of /LIST", responses[1]);
}

#[test]
fn list_with_no_parameters_prints_all_channels() {
    let mut handler = dummy_client_handler();

    let parameters = vec![];

    handler.database.add_client_to_channel("#hola", "nickname");
    handler
        .database
        .set_channel_topic("#hola", "topic for #hola");
    handler.database.add_client_to_channel("#chau", "nickname");
    handler.database.add_local_client(dummy_client("nick2"));
    handler.database.add_client_to_channel("nick2", "#canal");

    handler.list_command((None, parameters, None)).unwrap();

    let mut responses = handler.stream.get_responses();

    let mut channels: Vec<String> = responses.drain(1..=3).collect();

    channels.sort();

    assert_eq!("321 :Channel :Users Name", responses[0]);
    assert_eq!("322 #canal :No topic set", channels[0]);
    assert_eq!("322 #chau :No topic set", channels[1]);
    assert_eq!("322 #hola :topic for #hola", channels[2]);
    assert_eq!("323 :End of /LIST", responses[1]);
}

#[test]
fn list_with_parameters_prints_requested_channels() {
    let mut handler = dummy_client_handler();

    let parameters = vec!["#hola,#chau".to_string()];

    handler.database.add_client_to_channel("#hola", "nickname");
    handler.database.add_client_to_channel("#chau", "nickname");

    handler.list_command((None, parameters, None)).unwrap();

    let responses = handler.stream.get_responses();

    assert_eq!("321 :Channel :Users Name", responses[0]);
    assert_eq!("322 #hola :No topic set", responses[1]);
    assert_eq!("322 #chau :No topic set", responses[2]);
    assert_eq!("323 :End of /LIST", responses[3]);
}

#[test]
fn list_ignores_invalid_channels() {
    let mut handler = dummy_client_handler();

    let parameters = vec!["#hola,#invalido,#chau".to_string()];

    handler.database.add_client_to_channel("#hola", "nickname");
    handler.database.add_client_to_channel("#chau", "nickname");

    handler.list_command((None, parameters, None)).unwrap();

    let responses = handler.stream.get_responses();

    assert_eq!("321 :Channel :Users Name", responses[0]);
    assert_eq!("322 #hola :No topic set", responses[1]);
    assert_eq!("322 #chau :No topic set", responses[2]);
    assert_eq!("323 :End of /LIST", responses[3]);
}

#[test]
fn list_ignores_secret_channels() {
    let mut handler = dummy_client_handler();

    handler.database.add_client_to_channel("#hola", "nickname");
    handler.database.add_client_to_channel("#chau", "nickname");

    handler.database.add_local_client(dummy_client("nick2"));
    handler.database.add_client_to_channel("nick2", "#secreto");

    handler
        .database
        .set_channel_flag("#secreto", ChannelFlag::Secret);

    let parameters = vec![];
    handler.list_command((None, parameters, None)).unwrap();

    let mut responses = handler.stream.get_responses();

    let mut channels: Vec<String> = responses.drain(1..=2).collect();

    channels.sort();

    assert_eq!("321 :Channel :Users Name", responses[0]);
    assert_eq!("322 #chau :No topic set", channels[0]);
    assert_eq!("322 #hola :No topic set", channels[1]);
    assert_eq!("323 :End of /LIST", responses[1]);
}

#[test]
fn list_prints_priv_channels_as_priv() {
    let mut handler = dummy_client_handler();

    handler.database.add_client_to_channel("#hola", "nickname");
    handler.database.add_client_to_channel("#chau", "nickname");

    handler.database.add_local_client(dummy_client("nick2"));
    handler.database.add_client_to_channel("nick2", "#privado");

    handler
        .database
        .set_channel_flag("#privado", ChannelFlag::Private);

    let parameters = vec!["#hola,#privado,#chau".to_string()];
    handler.list_command((None, parameters, None)).unwrap();

    let responses = handler.stream.get_responses();

    assert_eq!("321 :Channel :Users Name", responses[0]);
    assert_eq!("322 #hola :No topic set", responses[1]);
    assert_eq!("322 #privado Prv", responses[2]);
    assert_eq!("322 #chau :No topic set", responses[3]);
    assert_eq!("323 :End of /LIST", responses[4]);
}

#[test]
fn list_prints_secret_channel_if_client_is_in_it() {
    let mut handler = dummy_client_handler();

    handler.database.add_client_to_channel("#hola", "nickname");
    handler.database.add_client_to_channel("#chau", "nickname");
    handler
        .database
        .add_client_to_channel("nickname", "#secreto");

    handler
        .database
        .set_channel_flag("#secreto", ChannelFlag::Secret);

    let parameters = vec![];
    handler.list_command((None, parameters, None)).unwrap();

    let mut responses = handler.stream.get_responses();

    let mut channels: Vec<String> = responses.drain(1..=3).collect();

    channels.sort();

    assert_eq!("321 :Channel :Users Name", responses[0]);
    assert_eq!("322 #chau :No topic set", channels[0]);
    assert_eq!("322 #hola :No topic set", channels[1]);
    assert_eq!("322 #secreto :No topic set", channels[2]);
    assert_eq!("323 :End of /LIST", responses[1]);
}

#[test]
fn list_prints_private_channel_if_client_is_in_it() {
    let mut handler = dummy_client_handler();

    handler.database.add_client_to_channel("#hola", "nickname");
    handler.database.add_client_to_channel("#chau", "nickname");
    handler
        .database
        .add_client_to_channel("nickname", "#privado");

    handler
        .database
        .set_channel_flag("#privado", ChannelFlag::Private);

    let parameters = vec!["#hola,#privado,#chau".to_string()];
    handler.list_command((None, parameters, None)).unwrap();

    let responses = handler.stream.get_responses();

    assert_eq!("321 :Channel :Users Name", responses[0]);
    assert_eq!("322 #hola :No topic set", responses[1]);
    assert_eq!("322 #privado :No topic set", responses[2]);
    assert_eq!("322 #chau :No topic set", responses[3]);
    assert_eq!("323 :End of /LIST", responses[4]);
}
