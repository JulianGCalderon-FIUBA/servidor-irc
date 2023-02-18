// mod utils;
// use std::{thread, time::Duration};

// use internet_relay_chat::client::Client;

// use crate::utils::{create_lemonpie_server, create_server};

// #[test]
// fn can_create_server_that_listens_to_address() {
//     let mut server = create_server("lemonpie", "IRC server");
//     let address = "127.0.0.1:9000".to_string();
//     assert!(server.listen_to(address).is_ok());
// }

// #[test]
// fn server_fails_when_listening_to_occupied_address() {
//     let address = "127.0.0.1:9001".to_string();

//     let _server = create_lemonpie_server(address.clone());

//     let mut new_server = create_server("chocotorta", "New IRC server");

//     assert!(new_server.listen_to(address).is_err());
// }

// #[test]
// fn client_fails_if_address_has_no_server_behind() {
//     let address = "127.0.0.1:9002".to_string();

//     let client = Client::new(address);
//     assert!(client.is_err());
// }

// #[test]
// fn can_connect_client_to_server() {
//     let address = "127.0.0.1:9003".to_string();

//     let _server = create_lemonpie_server(address.clone());

//     let client = Client::new(address);

//     assert!(client.is_ok());
// }

// #[test]
// fn client_can_send_and_receive_message_from_server() {
//     let address = "127.0.0.1:9005".to_string();
//     let _server = create_lemonpie_server(address.clone());

//     let mut client = Client::new(address).unwrap();

//     let message = "HOLA";
//     assert!(client.send_raw(message).is_ok());

//     let response = client.sync_read().unwrap();
//     assert_eq!("421 HOLA :Unknown command", response.to_string());
// }

// #[test]
// fn client_can_send_and_receive_multiple_messages_from_server() {
//     let address = "127.0.0.1:9006".to_string();

//     let _server = create_lemonpie_server(address.clone());

//     let mut client = Client::new(address).unwrap();

//     let message1 = "HOLA";
//     client.send_raw(message1).unwrap();
//     let message2 = "PROBANDO";
//     client.send_raw(message2).unwrap();

//     let response1 = client.sync_read().unwrap();
//     assert_eq!("421 HOLA :Unknown command", response1.to_string());
//     let response2 = client.sync_read().unwrap();
//     assert_eq!("421 PROBANDO :Unknown command", response2.to_string());
// }

// #[test]
// fn can_connect_multiple_clients_to_server() {
//     let address = "127.0.0.1:9004".to_string();
//     let _server = create_lemonpie_server(address.clone());

//     let client1 = Client::new(address.clone());
//     let client2 = Client::new(address);
//     assert!(client1.is_ok());
//     assert!(client2.is_ok());
// }

// #[test]
// fn client_can_register_in_server() {
//     let address = "127.0.0.1:9007".to_string();
//     let _server = create_lemonpie_server(address.clone());

//     let mut client = Client::new(address).unwrap();

//     let message1 = "NICK nickname";
//     client.send_raw(message1).unwrap();
//     let message2 = "USER username :realname";
//     client.send_raw(message2).unwrap();

//     let response = client.sync_read().unwrap();
//     assert_eq!(
//         "001 realname :Welcome to lemonpie Network, nickname !username @127.0.0.1",
//         response.to_string()
//     );

//     let message3 = "JOIN #channel";
//     client.send_raw(message3).unwrap();
//     let response = client.sync_read().unwrap();
//     assert_eq!("331 #channel :No topic is set", response.to_string());
//     let response2 = client.sync_read().unwrap();
//     assert_eq!("353 #channel :nickname", response2.to_string());
// }

// //Corre pero está comentado para no hacer lentos los demás tests.

// // #[test]
// // fn client_connection_closes_after_timeout() {
// //     let address = "127.0.0.1:9008".to_string();
// //     let _server = create_lemonpie_server(address.clone());

// //     let client_thread = thread::spawn(|| {
// //         let mut client = Client::new(address).unwrap();
// //         sleep(Duration::from_millis(101));

// //         let response = client.sync_read().unwrap();
// //         assert_eq!("Registration timeout", response.to_string());
// //     });

// //     client_thread.join().unwrap();
// // }

// #[test]
// fn can_shutdown_server() {
//     let address = "127.0.0.1:9009".to_string();
//     let server = create_lemonpie_server(address.clone());

//     let mut client = Client::new(address).unwrap();

//     server.quit();
//     assert!(client.sync_read().is_err());
// }

// #[test]
// fn can_connect_two_servers() {
//     let address1 = "127.0.0.1:9010".to_string();
//     let _server1 = create_lemonpie_server(address1.clone());

//     let mut client1 = Client::new(address1.clone()).unwrap();

//     let nick1 = "NICK nickname1";
//     let user1 = "USER username1 :realname1";
//     client1.send_raw(nick1).unwrap();
//     client1.send_raw(user1).unwrap();

//     let response1 = client1.sync_read().unwrap();
//     assert_eq!(
//         "001 realname1 :Welcome to lemonpie Network, nickname1 !username1 @127.0.0.1",
//         response1.to_string()
//     );

//     let address2 = "127.0.0.1:9011".to_string();
//     let mut server2 = create_server("chocotorta", "New IRC server");
//     server2.listen_to(address2.clone()).unwrap();

//     let mut client2 = Client::new(address2).unwrap();

//     let nick2 = "NICK nickname2";
//     let user2 = "USER username2 :realname2";
//     client2.send_raw(nick2).unwrap();
//     client2.send_raw(user2).unwrap();

//     let response2 = client2.sync_read().unwrap();
//     assert_eq!(
//         "001 realname2 :Welcome to chocotorta Network, nickname2 !username2 @127.0.0.1",
//         response2.to_string()
//     );

//     server2.connect_to(&address1);
//     thread::sleep(Duration::from_millis(10));

//     let thread1 = thread::spawn(move || {
//         let privmsg = "PRIVMSG nickname2 :holaa";
//         client1.send_raw(privmsg).unwrap();
//     });
//     let thread2 = thread::spawn(move || {
//         let client2_response = client2.sync_read().unwrap();
//         assert_eq!(
//             ":nickname1 PRIVMSG nickname2 :holaa",
//             client2_response.to_string()
//         );
//     });

//     thread1.join().unwrap();
//     thread2.join().unwrap();
// }
