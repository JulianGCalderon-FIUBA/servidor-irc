use gtk4::glib::{Receiver, Sender};
use gtk4::prelude::Continue;

use crate::{client::async_reader::AsyncReader, controller::controller_message::ControllerMessage};

use crate::controller::controller_message::ControllerMessage::ReceiveSafeMessage;

use super::InterfaceController;

use std::{net::TcpStream, thread};

impl InterfaceController {
    /// Starts listening a dcc chat
    ///
    /// Receives a stream and a string sender, returns nothing.
    pub fn start_listening_dcc(&mut self, dcc_chat: TcpStream, sender: Sender<String>) {
        let (_async_reader, message_receiver) = AsyncReader::spawn(dcc_chat);
        thread::spawn(move || {
            while let Ok(message_received) = message_receiver.recv() {
                match message_received {
                    Ok(message) => {
                        sender.send(message.to_string()).expect("error");
                    }
                    Err(error) => eprintln!("{error}"),
                }
            }
        });
    }

    /// Function to start sending whatever the receiver receives through the sender.
    ///
    /// Receives a string, string receiver and a controller message sender, returns nothing.
    pub fn receiver_attach(
        &mut self,
        client: String,
        dcc_receiver: Receiver<String>,
        sender: Sender<ControllerMessage>,
    ) {
        dcc_receiver.attach(None, move |message| {
            sender
                .send(ReceiveSafeMessage {
                    client: client.clone(),
                    message,
                })
                .expect("error");
            Continue(true)
        });
    }
}
