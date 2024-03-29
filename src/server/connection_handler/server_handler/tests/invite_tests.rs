use crate::server::{
    connection_handler::ConnectionHandlerCommands,
    testing::{dummy_client, dummy_external_client, dummy_server},
};

use super::dummy_server_handler;

#[test]
fn invite_with_invalid_arguments_is_ignored() {
    let mut handler = dummy_server_handler();
    handler
        .database
        .add_immediate_server(dummy_server("servername2"));
    handler
        .database
        .add_external_client(dummy_external_client("sender", "servername1"));

    handler.database.add_local_client(dummy_client("target"));

    let params = vec!["target".to_string(), "#channel".to_string()];
    handler.invite_command((None, params, None)).unwrap();
    let prefix = Some("sender".to_string());
    let params = vec!["target".to_string()];
    handler.invite_command((prefix, params, None)).unwrap();
    let prefix = Some("sender".to_string());
    handler.invite_command((prefix, vec![], None)).unwrap();

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
fn invite_with_no_inviting_client_in_database_is_ignored() {
    let mut handler = dummy_server_handler();
    handler
        .database
        .add_immediate_server(dummy_server("servername2"));
    handler.database.add_local_client(dummy_client("target"));

    let prefix = Some("sender".to_string());
    let params = vec!["target".to_string(), "#channel".to_string()];
    handler.invite_command((prefix, params, None)).unwrap();

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
fn invite_with_no_invited_client_in_database_is_ignored() {
    let mut handler = dummy_server_handler();
    handler
        .database
        .add_immediate_server(dummy_server("servername2"));
    handler
        .database
        .add_external_client(dummy_external_client("sender", "servername1"));

    let prefix = Some("sender".to_string());
    let params = vec!["target".to_string(), "#channel".to_string()];
    handler.invite_command((prefix, params, None)).unwrap();

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
fn invite_is_sent_to_local_client() {
    let mut handler = dummy_server_handler();
    handler
        .database
        .add_external_client(dummy_external_client("sender", "servername1"));
    handler.database.add_local_client(dummy_client("target"));

    let prefix = Some("sender".to_string());
    let params = vec!["target".to_string(), "#channel".to_string()];
    handler.invite_command((prefix, params, None)).unwrap();

    assert_eq!(
        ":sender INVITE target #channel\r\n",
        handler
            .database
            .get_local_stream("target")
            .unwrap()
            .read_wbuf_to_string()
    )
}

#[test]
fn invite_is_relayed_to_all_other_servers() {
    let mut handler = dummy_server_handler();
    handler
        .database
        .add_immediate_server(dummy_server("servername2"));
    handler
        .database
        .add_immediate_server(dummy_server("servername3"));
    handler
        .database
        .add_external_client(dummy_external_client("target", "servername3"));
    handler.database.add_local_client(dummy_client("sender"));

    let prefix = Some("sender".to_string());
    let params = vec!["target".to_string(), "#channel".to_string()];
    handler.invite_command((prefix, params, None)).unwrap();

    assert_eq!(
        ":sender INVITE target #channel\r\n",
        handler
            .database
            .get_server_stream("servername2")
            .unwrap()
            .read_wbuf_to_string()
    );

    assert_eq!(
        ":sender INVITE target #channel\r\n",
        handler
            .database
            .get_server_stream("servername3")
            .unwrap()
            .read_wbuf_to_string()
    );
}
