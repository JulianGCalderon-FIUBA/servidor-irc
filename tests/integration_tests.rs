use std::{io::Write, net::TcpStream, thread};

use internet_relay_chat::{client::Client, server::Server};

#[test]
fn can_create_server_that_listens_to_address() -> std::io::Result<()> {
    let servername = "lemonpie".to_string();
    let serverinfo = "IRC server".to_string();
    let address = "127.0.0.1:9000".to_string();

    let server_thread = thread::spawn(|| {
        let mut server = Server::start(servername, serverinfo);
        server.listen_to(address).unwrap();
    });

    let mut stream = TcpStream::connect("127.0.0.1:9000")?;
    let quit = "QUIT\n".as_bytes();
    let bytes_written = stream.write(quit)?;
    assert_eq!(quit.len(), bytes_written);
    server_thread.join().unwrap();
    Ok(())
}

#[test]
fn server_fails_when_listening_to_occupied_address() {
    let mut servername = "lemonpie".to_string();
    let mut serverinfo = "IRC server".to_string();
    let address = "127.0.0.1:9001".to_string();

    let mut server = Server::start(servername, serverinfo);
    server.listen_to(address.clone()).unwrap();

    servername = "chocotorta".to_string();
    serverinfo = "New IRC server".to_string();

    let mut new_server = Server::start(servername, serverinfo);

    assert!(new_server.listen_to(address).is_err());

    drop(server);
    drop(new_server);
}

#[test]
fn client_fails_if_address_has_no_server_behind() {
    let address = "127.0.0.1:9002".to_string();

    let client = Client::new(address);
    assert!(client.is_err());
}

#[test]
fn can_connect_client_to_server() {
    let servername = "lemonpie".to_string();
    let serverinfo = "IRC server".to_string();
    let address = "127.0.0.1:9003".to_string();

    let mut server = Server::start(servername, serverinfo);

    server.listen_to(address.clone()).unwrap();

    let client = Client::new(address);

    assert!(client.is_ok());
}

#[test]
fn client_can_send_and_receive_messages_from_server() {
    let servername = "lemonpie".to_string();
    let serverinfo = "IRC server".to_string();
    let address = "127.0.0.1:9005".to_string();

    let mut server = Server::start(servername, serverinfo);

    server.listen_to(address.clone()).unwrap();

    let client = Client::new(address);

    assert!(client.is_ok());
}

#[test]
fn can_connect_multiple_clients_to_server() {
    let servername = "lemonpie".to_string();
    let serverinfo = "IRC server".to_string();
    let address = "127.0.0.1:9004".to_string();

    let mut server = Server::start(servername, serverinfo);

    server.listen_to(address.clone()).unwrap();

    let client1 = Client::new(address.clone());
    let client2 = Client::new(address);
    assert!(client1.is_ok());
    assert!(client2.is_ok());
}
