use super::*;

#[test]
fn list_fails_with_unregistered_client() {
    let mut handler = dummy_client_handler();

    let parameters = vec![];

    handler.list_command(parameters).unwrap();

    assert_eq!(
        "200 :unregistered\r\n",
        handler.stream_client_handler.read_wbuf_to_string()
    )
}
