use super::*;

#[test]
fn kick_fails_with_unregistered_client() {
    let mut handler = dummy_client_handler();

    let parameters = vec!["#canal1".to_string(), "user1".to_string()];
    handler.kick_command(parameters, None).unwrap();

    assert_eq!(
        "200 :Unregistered\r\n",
        handler.stream.read_wbuf_to_string()
    )
}

// #[test]
// fn join_fails_with_empty_params() {
//     let mut handler = dummy_client_handler();
//     let parameters = vec![];
//     register_client(&mut handler, "nick");

//     let channels: Vec<String> = vec![];
//     handler.join_command(parameters).unwrap();

//     assert_eq!(
//         "461 JOIN :Not enough parameters\r\n",
//         handler.stream.read_wbuf_to_string()
//     );
//     assert_eq!(handler.database.get_all_channels(), channels);
// }
