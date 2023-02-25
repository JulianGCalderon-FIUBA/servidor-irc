use gtk4::glib::{self, Receiver, Sender};

use crate::{controller::controller_message::ControllerMessage, client::async_reader::AsyncReader};

use crate::controller::controller_message::ControllerMessage::ReceiveSafeMessage;

use super::InterfaceController;

use std::{net::TcpStream, thread};

impl InterfaceController {
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
            glib::Continue(true)
        });
    }
}
