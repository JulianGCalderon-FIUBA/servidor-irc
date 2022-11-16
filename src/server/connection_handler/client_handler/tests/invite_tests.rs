use super::*;

#[test]
fn invite_fails_with_less_than_two_parameters() {
    let mut handler = dummy_client_handler();

    let parameters = vec![];

    handler.invite_command(parameters).unwrap();

    assert_eq!(
        "461 INVITE :Not enough parameters\r\n",
        handler.stream.read_wbuf_to_string()
    );
    handler.stream.clear();

    let parameters2 = vec!["nick2".to_string()];

    handler.invite_command(parameters2).unwrap();

    assert_eq!(
        "461 INVITE :Not enough parameters\r\n",
        handler.stream.read_wbuf_to_string()
    )
}

#[test]
fn invite_fails_with_invalid_nickname() {
    let mut handler = dummy_client_handler();

    let parameters = vec!["nick2".to_string(), "#hola".to_string()];

    handler.invite_command(parameters).unwrap();

    assert_eq!(
        "401 nick2 :No such nick/channel\r\n",
        handler.stream.read_wbuf_to_string()
    )
}

#[test]
fn invite_fails_with_user_already_on_channel() {
    let mut handler = dummy_client_handler();

    handler.database.add_client(dummy_client("nick2"));
    handler.database.add_client_to_channel("nick2", "#hola");
    handler.database.add_client_to_channel("nick", "#hola");

    let parameters = vec!["nick2".to_string(), "#hola".to_string()];

    handler.invite_command(parameters).unwrap();

    assert_eq!(
        "443 nick2 #hola :Is already on channel\r\n",
        handler.stream.read_wbuf_to_string()
    )
}

#[test]
fn invite_fails_with_sending_user_not_on_channel() {
    let mut handler = dummy_client_handler();

    handler.database.add_client(dummy_client("nick2"));
    handler.database.add_client_to_channel("nick2", "#hola");

    let parameters = vec!["nick2".to_string(), "#hola".to_string()];

    handler.invite_command(parameters).unwrap();

    assert_eq!(
        "442 #hola :You're not on that channel\r\n",
        handler.stream.read_wbuf_to_string()
    )
}

#[test]
fn can_invite_one_user() {
    let mut handler = dummy_client_handler();

    handler.database.add_client(dummy_client("nick2"));
    handler.database.add_client_to_channel("nick", "#hola");

    let parameters = vec!["nick2".to_string(), "#hola".to_string()];

    handler.invite_command(parameters).unwrap();

    assert_eq!("341 #hola nickname\r\n", handler.stream.read_wbuf_to_string());

    assert_eq!(
        ":nick INVITE nick2 #hola\r\n",
        handler
            .database
            .get_stream("nick2")
            .unwrap()
            .read_wbuf_to_string()
    );
}

#[test]
fn invite_fails_with_not_channop_on_moderated_channel() {
    let mut handler = dummy_client_handler();

    handler.database.add_client(dummy_client("nick2"));
    handler.database.add_client_to_channel("nick", "#hola");

    handler.database.set_channel_mode("#hola", 'i');
    handler.database.remove_channop("#hola", "nick");

    let parameters = vec!["nick2".to_string(), "#hola".to_string()];

    handler.invite_command(parameters).unwrap();

    assert_eq!(
        "482 #hola :You're not channel operator\r\n",
        handler.stream.read_wbuf_to_string()
    );

    assert_eq!(
        "",
        handler
            .database
            .get_stream("nick2")
            .unwrap()
            .read_wbuf_to_string()
    );
}

#[test]
fn can_invite_user_in_moderated_channel_if_channop() {
    let mut handler = dummy_client_handler();

    handler.database.add_client(dummy_client("nick2"));
    handler.database.add_client_to_channel("nick", "#hola");

    handler.database.set_channel_mode("#hola", 'i');

    let parameters = vec!["nick2".to_string(), "#hola".to_string()];

    handler.invite_command(parameters).unwrap();

    assert_eq!("341 #hola nickname\r\n", handler.stream.read_wbuf_to_string());

    assert_eq!(
        ":nick INVITE nick2 #hola\r\n",
        handler
            .database
            .get_stream("nick2")
            .unwrap()
            .read_wbuf_to_string()
    );
}
