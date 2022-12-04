// Necesario para correr el test comentado

// use std::{
//     thread::{self, sleep},
//     time::Duration,
// };

use internet_relay_chat::{client::Client, server::Server};

#[test]
fn can_create_server_that_listens_to_address() {
    let servername = "lemonpie".to_string();
    let serverinfo = "IRC server".to_string();
    let address = "127.0.0.1:9000".to_string();

    let mut server = Server::start(servername, serverinfo);
    assert!(server.listen_to(address).is_ok());
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
fn client_can_send_and_receive_message_from_server() {
    let servername = "lemonpie".to_string();
    let serverinfo = "IRC server".to_string();
    let address = "127.0.0.1:9005".to_string();

    let mut server = Server::start(servername, serverinfo);
    server.listen_to(address.clone()).unwrap();

    let mut client = Client::new(address).unwrap();

    let message = "HOLA";
    assert!(client.send_raw(message).is_ok());

    let response = client.sync_read().unwrap();
    assert_eq!("421 HOLA :Unknown command", response.to_string());
}

#[test]
fn client_can_send_and_receive_multiple_messages_from_server() {
    let servername = "lemonpie".to_string();
    let serverinfo = "IRC server".to_string();
    let address = "127.0.0.1:9006".to_string();

    let mut server = Server::start(servername, serverinfo);

    server.listen_to(address.clone()).unwrap();

    let mut client = Client::new(address).unwrap();

    let message1 = "HOLA";
    client.send_raw(message1).unwrap();
    let message2 = "PROBANDO";
    client.send_raw(message2).unwrap();

    let response1 = client.sync_read().unwrap();
    assert_eq!("421 HOLA :Unknown command", response1.to_string());
    let response2 = client.sync_read().unwrap();
    assert_eq!("421 PROBANDO :Unknown command", response2.to_string());
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

#[test]
fn client_can_register_in_server() {
    let servername = "lemonpie".to_string();
    let serverinfo = "IRC server".to_string();
    let address = "127.0.0.1:9007".to_string();

    let mut server = Server::start(servername, serverinfo);
    server.listen_to(address.clone()).unwrap();

    let mut client = Client::new(address).unwrap();

    let message1 = "NICK nickname";
    client.send_raw(message1).unwrap();
    let message2 = "USER username :realname";
    client.send_raw(message2).unwrap();

    let response = client.sync_read().unwrap();
    assert_eq!(
        "001 nickname :Welcome to server lemonpie, username",
        response.to_string()
    );

    let message3 = "NICK nickname";
    client.send_raw(message3).unwrap();
    let response = client.sync_read().unwrap();
    assert_eq!(
        "433 nickname :Nickname is already in use",
        response.to_string()
    );
}

// Corre pero está comentado para no hacer lentos los demás tests.

// #[test]
// fn client_connection_closes_after_timeout() {
//     let servername = "lemonpie".to_string();
//     let serverinfo = "IRC server".to_string();
//     let address = "127.0.0.1:9008".to_string();

//     let mut server = Server::start(servername, serverinfo);

//     server.listen_to(address.clone()).unwrap();

//     let client_thread = thread::spawn(|| {
//         let mut client = Client::new(address).unwrap();
//         sleep(Duration::from_millis(101));

//         let response = client.sync_read().unwrap();
//         assert_eq!("Registration timeout", response.to_string());
//     });

//     client_thread.join().unwrap();
// }

// #[test]
// fn can_send_messages_between_clients_through_server() {
//     let servername = "lemonpie".to_string();
//     let serverinfo = "IRC server".to_string();
//     let address = "127.0.0.1:9009".to_string();

//     let mut server = Server::start(servername, serverinfo);

//     server.listen_to(address.clone()).unwrap();

//     let mut client1 = Client::new(address.clone()).unwrap();
//     let mut client2 = Client::new(address).unwrap();

//     let nick1 = "NICK nickname1";
//     let user1 = "USER username1 :realname1";
//     let nick2 = "NICK nickname2";
//     let user2 = "USER username2 :realname2";

//     client1.send_raw(nick1).unwrap();
//     client1.send_raw(user1).unwrap();
//     client2.send_raw(nick2).unwrap();
//     client2.send_raw(user2).unwrap();

//     let response1 = client1.sync_read().unwrap();
//     assert_eq!(
//         "001 nickname1 :Welcome to server lemonpie, username1",
//         response1.to_string()
//     );

//     let response2 = client2.sync_read().unwrap();
//     assert_eq!(
//         "001 nickname2 :Welcome to server lemonpie, username2",
//         response2.to_string()
//     );

//     // let privmsg = "PRIVMSG nickname2 :holaaa";
//     // client1.send_raw(privmsg).unwrap();

//     // let response1 = client1.sync_read().unwrap();
//     // assert_eq!("", response1.to_string());

//     // let response2 = client2.sync_read().unwrap();
//     // assert_eq!("", response2.to_string());
// }
