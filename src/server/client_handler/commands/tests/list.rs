use super::*;

#[test]
fn list_fails_with_unregistered_client() {
    let mut handler = dummy_client_handler();

    let parameters = vec![];

    handler.list_command(parameters).unwrap();

    assert_eq!(
        "200 :Unregistered\r\n",
        handler.stream.read_wbuf_to_string()
    )
}

#[test]
fn list_with_no_channels_prints_start_and_end() {
    let mut handler = dummy_client_handler();
    register_client(&mut handler, "nick");

    let parameters = vec![];

    handler.list_command(parameters).unwrap();

    let responses = handler.stream.get_responses();

    assert_eq!("321 :Channel :Users Name", responses[0]);
    assert_eq!("323 :End of /LIST", responses[1]);
}

#[test]
fn list_with_no_parameters_prints_all_channels() {
    let mut handler = dummy_client_handler();
    register_client(&mut handler, "nick");

    let parameters = vec![];

    handler.database.add_client_to_channel("nick", "#hola");
    handler
        .database
        .set_channel_topic("#hola", "topic for #hola");
    handler.database.add_client_to_channel("nick", "#chau");

    handler.list_command(parameters.clone()).unwrap();

    let responses = handler.stream.get_responses();

    assert_eq!("321 :Channel :Users Name", responses[0]);
    assert_eq!("322 #chau :No topic set", responses[1]);
    assert_eq!("322 #hola :topic for #hola", responses[2]);
    assert_eq!("323 :End of /LIST", responses[3]);

    handler.database.add_client(dummy_client("nick2"));
    handler.database.add_client_to_channel("nick2", "#canal");
    handler.stream.clear();

    handler.list_command(parameters).unwrap();

    let responses = handler.stream.get_responses();

    assert_eq!("321 :Channel :Users Name", responses[0]);
    assert_eq!("322 #canal :No topic set", responses[1]);
    assert_eq!("322 #chau :No topic set", responses[2]);
    assert_eq!("322 #hola :topic for #hola", responses[3]);
    assert_eq!("323 :End of /LIST", responses[4]);
}

#[test]
fn list_with_parameters_prints_requested_channels() {
    let mut handler = dummy_client_handler();
    register_client(&mut handler, "nick");

    let parameters = vec!["#hola".to_string()];

    handler.database.add_client_to_channel("nick", "#hola");
    handler.database.add_client_to_channel("nick", "#chau");

    handler.list_command(parameters).unwrap();

    let responses = handler.stream.get_responses();

    assert_eq!("321 :Channel :Users Name", responses[0]);
    assert_eq!("322 #hola :No topic set", responses[1]);
    assert_eq!("323 :End of /LIST", responses[2]);

    handler.stream.clear();

    let parameters2 = vec!["#hola,#chau".to_string()];

    handler.list_command(parameters2).unwrap();

    let responses = handler.stream.get_responses();

    assert_eq!("321 :Channel :Users Name", responses[0]);
    assert_eq!("322 #hola :No topic set", responses[1]);
    assert_eq!("322 #chau :No topic set", responses[2]);
    assert_eq!("323 :End of /LIST", responses[3]);
}

#[test]
fn list_ignores_invalid_channels() {
    let mut handler = dummy_client_handler();
    register_client(&mut handler, "nick");

    let parameters = vec!["#hola,#invalido,#chau".to_string()];

    handler.database.add_client_to_channel("nick", "#hola");
    handler.database.add_client_to_channel("nick", "#chau");

    handler.list_command(parameters).unwrap();

    let responses = handler.stream.get_responses();

    assert_eq!("321 :Channel :Users Name", responses[0]);
    assert_eq!("322 #hola :No topic set", responses[1]);
    assert_eq!("322 #chau :No topic set", responses[2]);
    assert_eq!("323 :End of /LIST", responses[3]);
}

#[test]
fn list_ignores_secret_channels() {
    let mut handler = dummy_client_handler();
    register_client(&mut handler, "nick");

    handler.database.add_client_to_channel("nick", "#hola");
    handler.database.add_client_to_channel("nick", "#chau");

    handler.database.add_client(dummy_client("nick2"));
    handler.database.add_client_to_channel("nick2", "#secreto");

    handler.database.set_channel_mode("#secreto", 's');

    let parameters = vec![];
    handler.list_command(parameters).unwrap();

    let responses = handler.stream.get_responses();

    assert_eq!("321 :Channel :Users Name", responses[0]);
    assert_eq!("322 #chau :No topic set", responses[1]);
    assert_eq!("322 #hola :No topic set", responses[2]);
    assert_eq!("323 :End of /LIST", responses[3]);
}

#[test]
fn list_prints_priv_channels_as_priv() {
    let mut handler = dummy_client_handler();
    register_client(&mut handler, "nick");

    handler.database.add_client_to_channel("nick", "#hola");
    handler.database.add_client_to_channel("nick", "#chau");

    handler.database.add_client(dummy_client("nick2"));
    handler.database.add_client_to_channel("nick2", "#privado");

    handler.database.set_channel_mode("#privado", 'p');

    let parameters = vec!["#hola,#privado,#chau".to_string()];
    handler.list_command(parameters).unwrap();

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
    register_client(&mut handler, "nick");

    handler.database.add_client_to_channel("nick", "#hola");
    handler.database.add_client_to_channel("nick", "#chau");
    handler.database.add_client_to_channel("nick", "#secreto");

    handler.database.set_channel_mode("#secreto", 's');

    let parameters = vec![];
    handler.list_command(parameters).unwrap();

    let responses = handler.stream.get_responses();

    assert_eq!("321 :Channel :Users Name", responses[0]);
    assert_eq!("322 #chau :No topic set", responses[1]);
    assert_eq!("322 #hola :No topic set", responses[2]);
    assert_eq!("322 #secreto :No topic set", responses[3]);
    assert_eq!("323 :End of /LIST", responses[4]);
}

#[test]
fn list_prints_private_channel_if_client_is_in_it() {
    let mut handler = dummy_client_handler();
    register_client(&mut handler, "nick");

    handler.database.add_client_to_channel("nick", "#hola");
    handler.database.add_client_to_channel("nick", "#chau");
    handler.database.add_client_to_channel("nick", "#privado");

    handler.database.set_channel_mode("#privado", 'p');

    let parameters = vec!["#hola,#privado,#chau".to_string()];
    handler.list_command(parameters).unwrap();

    let responses = handler.stream.get_responses();

    assert_eq!("321 :Channel :Users Name", responses[0]);
    assert_eq!("322 #hola :No topic set", responses[1]);
    assert_eq!("322 #privado :No topic set", responses[2]);
    assert_eq!("322 #chau :No topic set", responses[3]);
    assert_eq!("323 :End of /LIST", responses[4]);
}
