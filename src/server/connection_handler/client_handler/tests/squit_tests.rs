use crate::server::{
    connection_handler::connection_handler_trait::ConnectionHandlerCommands,
    testing::{dummy_client, dummy_distant_server, dummy_external_client, dummy_server},
};

use super::dummy_client_handler;

#[test]
fn squit_fails_without_oper() {
    let mut handler = dummy_client_handler();
    handler
        .database
        .add_immediate_server(dummy_server("servername2"));

    let params = vec!["servername2".to_string()];
    handler.squit_command((None, params, None)).unwrap();

    assert_eq!(
        "481 :Permission Denied- You're not an IRC operator\r\n",
        handler.stream.read_wbuf_to_string()
    );
}

#[test]
fn squit_fails_without_valid_server() {
    let mut handler = dummy_client_handler();
    handler.database.set_server_operator("nickname");

    let params = vec!["servername2".to_string()];
    handler.squit_command((None, params, None)).unwrap();

    assert_eq!(
        "402 servername2 :No such server\r\n",
        handler.stream.read_wbuf_to_string()
    );
}

#[test]
fn squit_is_relayed_to_all_servers() {
    let mut handler = dummy_client_handler();
    handler.database.set_server_operator("nickname");

    handler
        .database
        .add_immediate_server(dummy_server("servername2"));
    handler
        .database
        .add_immediate_server(dummy_server("servername3"));
    handler
        .database
        .add_distant_server(dummy_distant_server("servername5"));

    let params = vec!["servername5".to_string()];
    handler.squit_command((None, params, None)).unwrap();

    assert_eq!(
        ":nickname SQUIT servername5\r\n",
        handler
            .database
            .get_server_stream("servername2")
            .unwrap()
            .read_wbuf_to_string()
    );
    assert_eq!(
        ":nickname SQUIT servername5\r\n",
        handler
            .database
            .get_server_stream("servername3")
            .unwrap()
            .read_wbuf_to_string()
    );
}

#[test]
fn squit_to_immediate_server_removes_it_from_database() {
    let mut handler = dummy_client_handler();
    handler.database.set_server_operator("nickname");
    handler
        .database
        .add_immediate_server(dummy_server("servername2"));

    let params = vec!["servername2".to_string()];
    handler.squit_command((None, params, None)).unwrap();

    assert!(!handler.database.contains_server("servername2"));
}

// #[test]
// fn squit_to_immediate_server_disconnects_its_clients() {
//     let mut handler = dummy_client_handler();
//     handler.database.set_server_operator("nickname");
//     handler
//         .database
//         .add_immediate_server(dummy_server("servername2"));
//     handler
//         .database
//         .add_external_client(dummy_external_client("nickname1", "servername2"));

//     let params = vec!["servername2".to_string()];
//     handler.squit_command((None, params, None)).unwrap();

//     assert!(!handler.database.client_is_online("nickname1"));
// }

#[test]
fn squit_to_immediate_server_notifies_quit_for_every_client() {
    let mut handler = dummy_client_handler();
    handler.database.set_server_operator("nickname");
    handler
        .database
        .add_immediate_server(dummy_server("servername2"));
    handler
        .database
        .add_immediate_server(dummy_server("servername3"));

    handler
        .database
        .add_external_client(dummy_external_client("nickname1", "servername2"));
    handler.database.add_local_client(dummy_client("nickname2"));

    handler
        .database
        .add_client_to_channel("nickname2", "#channel");
    handler
        .database
        .add_client_to_channel("nickname1", "#channel");

    let params = vec!["servername2".to_string()];
    handler.squit_command((None, params, None)).unwrap();

    assert_eq!(
        ":nickname1 QUIT :Net split\r\n",
        handler
            .database
            .get_local_stream("nickname2")
            .unwrap()
            .read_wbuf_to_string()
    );
    assert_eq!(
        ":nickname SQUIT servername2\r\n:nickname1 QUIT :Net split\r\n",
        handler
            .database
            .get_server_stream("servername3")
            .unwrap()
            .read_wbuf_to_string()
    );
}
