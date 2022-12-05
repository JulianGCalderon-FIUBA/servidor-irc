use crate::server::{
    connection_handler::{server_handler::tests::dummy_server_handler, ConnectionHandlerCommands},
    testing::dummy_server,
};

#[test]
fn server_with_invalid_arguments_is_ignored() {
    let mut handler = dummy_server_handler();
    handler
        .database
        .add_immediate_server(dummy_server("servername2"));

    let parameters = vec!["servername4".to_string(), "2".to_string()];
    handler.server_command((None, parameters, None)).unwrap();

    let trail = Some("serverinfo".to_string());
    let parameters = vec!["servername4".to_string()];
    handler
        .server_command((None, parameters, trail.clone()))
        .unwrap();

    let parameters = vec![];
    handler
        .server_command((None, parameters, trail.clone()))
        .unwrap();

    let parameters = vec!["servername4".to_string(), "a".to_string()];
    handler.server_command((None, parameters, trail)).unwrap();

    assert_eq!(
        "",
        handler
            .database
            .get_server_stream("servername2")
            .unwrap()
            .read_wbuf_to_string()
    );
}

#[test]
fn server_adds_server_to_database() {
    let mut handler = dummy_server_handler();

    let parameters = vec!["servername2".to_string(), "1".to_string()];
    let trail = Some("serverinfo".to_string());
    handler.server_command((None, parameters, trail)).unwrap();

    assert!(handler.database.contains_server("servername2"));
}

#[test]
fn server_is_relayed_to_all_other_servers() {
    let mut handler = dummy_server_handler();
    handler
        .database
        .add_immediate_server(dummy_server("servername2"));
    handler
        .database
        .add_immediate_server(dummy_server("servername3"));

    let parameters = vec!["servername4".to_string(), "2".to_string()];
    let trail = Some("serverinfo".to_string());
    handler.server_command((None, parameters, trail)).unwrap();

    assert_eq!(
        "SERVER servername4 3 :serverinfo\r\n",
        handler
            .database
            .get_server_stream("servername2")
            .unwrap()
            .read_wbuf_to_string()
    );
    assert_eq!(
        "SERVER servername4 3 :serverinfo\r\n",
        handler
            .database
            .get_server_stream("servername3")
            .unwrap()
            .read_wbuf_to_string()
    );
}

#[test]
fn server_is_not_relayed_to_sending_server() {
    let mut handler = dummy_server_handler();

    let parameters = vec!["servername2".to_string(), "1".to_string()];
    let trail = Some("serverinfo".to_string());
    handler.server_command((None, parameters, trail)).unwrap();

    assert_eq!(
        "",
        handler
            .database
            .get_server_stream("servername1")
            .unwrap()
            .read_wbuf_to_string()
    );
}
