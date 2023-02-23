use gtk4::glib::{Sender, Receiver, self};

use crate::{client::async_reader::AsyncReader, controller::controller_message::ControllerMessage};

use crate::controller::controller_message::ControllerMessage::ReceiveSafeMessage;

use super::InterfaceController;

use std::{thread, net::TcpStream};

impl InterfaceController {
    pub fn start_listening_dcc(&mut self, dcc_chat: TcpStream, sender: Sender<String>) {
        let (_async_reader, message_receiver) =
            AsyncReader::spawn(dcc_chat);
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

    pub fn receiver_attach(&mut self, client: String, dcc_receiver: Receiver<String>, sender: Sender<ControllerMessage>) {
        dcc_receiver.attach(None, move |message| {
            sender.send(ReceiveSafeMessage { client: client.clone(), message }).expect("error");
            glib::Continue(true)
        });
    }
}