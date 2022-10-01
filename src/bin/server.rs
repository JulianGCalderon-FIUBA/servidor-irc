use std::io;
use std::net::{TcpListener, TcpStream};

use internet_relay_chat::message::Message;
use internet_relay_chat::ADDRESS;

fn main() {
    let listener = match TcpListener::bind(ADDRESS) {
        Ok(listener) => listener,
        Err(error) => return eprintln!("Error: Binding to address: {:?}", error),
    };

    for stream in listener.incoming() {
        let stream = match stream {
            Ok(stream) => stream,
            Err(error) => {
                eprintln!("Error: Connecting to client: {:?}", error);
                continue;
            }
        };

        match handle_client(stream) {
            Ok(_) => println!("Closing conection with client"),
            Err(error) => eprintln!("Error: Handling client: {:?}", error),
        }
    }
}

fn handle_client(mut stream: TcpStream) -> io::Result<()>
where
{
    let mut response_stream = stream.try_clone()?;

    while let Ok(Some(message)) = Message::read_from(&mut stream) {
        println!("Received: {}", message);
        message.send_to(&mut response_stream)?;
    }

    Ok(())
}
