use super::*;

#[test]
fn part_fails_with_empty_params() {
    let mut handler = dummy_client_handler();
    let parameters = vec![];
    let channels: Vec<String> = vec![];

    handler.part_command(parameters).unwrap();

    assert_eq!(
        "461 PART :Not enough parameters\r\n",
        handler.stream.read_wbuf_to_string()
    );
    assert_eq!(handler.database.get_all_channels(), channels);
}

#[test]
fn part_fails_with_invalid_channel_name() {
    let mut handler = dummy_client_handler();

    let parameters = vec!["hola,#ho'la,#hola".to_string()];

    handler.part_command(parameters).unwrap();

    let responses = handler.stream.get_responses();

    assert_eq!("403 hola :No such channel", responses[0]);
    assert_eq!("403 #ho'la :No such channel", responses[1]);
    assert_eq!("403 #hola :No such channel", responses[2]);
}

#[test]
fn part_fails_with_user_not_on_channel() {
    let mut handler = dummy_client_handler();

    handler.database.add_client(dummy_client("newnickname"));

    let parameters = vec!["#hola".to_string()];
    handler
        .database
        .add_client_to_channel("newnickname", "#hola");

    handler.part_command(parameters).unwrap();

    assert_eq!(
        "442 #hola :You're not on that channel\r\n",
        handler.stream.read_wbuf_to_string()
    )
}

#[test]
fn can_part_one_channel() {
    let mut handler = dummy_client_handler();

    let parameters = vec!["#hola".to_string()];
    handler.join_command(parameters.clone()).unwrap();

    handler.stream.clear();

    handler.part_command(parameters).unwrap();

    assert!(handler.database.get_all_channels().is_empty());
}

#[test]
fn can_part_existing_channels() {
    let mut handler = dummy_client_handler();

    let parameters = vec!["#hola,#chau".to_string()];

    handler.database.add_client(dummy_client("nick"));

    handler.database.add_client_to_channel("nick", "#hola");
    handler.database.add_client_to_channel("nick", "#chau");

    handler.join_command(parameters.clone()).unwrap();

    handler.stream.clear();

    handler.part_command(parameters).unwrap();

    assert!(!handler.database.get_all_channels().is_empty())
}

#[test]
fn part_notifies_users_in_channel() {
    let mut handler = dummy_client_handler();

    handler.database.add_client(dummy_client("nick2"));
    handler
        .database
        .add_client_to_channel("nickname", "#channel");
    handler.database.add_client_to_channel("nick2", "#channel");

    let parameters = vec!["#channel".to_string()];
    handler.part_command(parameters).unwrap();

    assert_eq!(
        ":nickname PART #channel\r\n",
        handler
            .database
            .get_stream("nick2")
            .unwrap()
            .read_wbuf_to_string()
    );

    assert_eq!(
        ":nickname PART #channel\r\n",
        handler
            .database
            .get_stream("nickname")
            .unwrap()
            .read_wbuf_to_string()
    );
}
