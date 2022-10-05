use std::io;

use std::net::{TcpListener, TcpStream};

use crate::message::{CreationError, Message, ParsingError};

pub struct Server {
    // id
    // base de datos
    listener: TcpListener,
}

impl Server {
    pub fn new(address: String) -> io::Result<Self> {
        let listener = TcpListener::bind(address)?;

        Ok(Self { listener })
    }

    pub fn listen(&mut self) -> io::Result<()> {
        for client in self.listener.incoming() {
            if let Err(err) = self.handle_client(client?) {
                eprintln!("Conection ended: {err}");
            }
        }

        Ok(())
    }

    fn handle_client(&self, mut client: TcpStream) -> io::Result<()> {
        loop {
            let message = match Message::read_from(&mut client) {
                Ok(message) => message,
                Err(error) => match error {
                    CreationError::IoError(error) => return Err(error),
                    CreationError::ParsingError(error) => {
                        send_response_for_parsing_error(error, &mut client)?;
                        continue;
                    }
                },
            };

            println!("Received: {}", message);
            message.send_to(&mut client)?;
        }
    }
}

fn send_response_for_parsing_error(error: ParsingError, client: &mut TcpStream) -> io::Result<()> {
    let error_string = format!("PARSINGERROR :{}", error);
    let error_message = Message::new(&error_string).expect("Is always valid");
    error_message.send_to(client)
}
