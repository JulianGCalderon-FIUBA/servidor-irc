use crate::server::{
    connection_handler::connection_handler_trait::ConnectionHandlerCommands, testing::dummy_client,
};

use super::dummy_server_handler;

#[test]

fn privmsg_is_relayed_to_client() {
    let mut handler = dummy_server_handler();
    handler.database.add_client(dummy_client("target"));

    let prefix = Some("sender".to_string());
    let params = vec!["target".to_string()];
    let trail = Some("message".to_string());
    handler.privmsg_command((prefix, params, trail)).unwrap();

    assert_eq!(
        ":sender PRIVMSG target :message\r\n",
        handler
            .database
            .get_stream("target")
            .unwrap()
            .unwrap()
            .read_wbuf_to_string()
    )
}

// fn privmsg_to_channel_is_relayed_to_all_clients_in_channel() {}

// fn privmsg_to_client_is_relayed_to_necesary_server() {}

// fn privmsg_to_channel_is_relayed_to_each_server_once() {}

// fn privmsg_fails_with_unexistent_client() {}
