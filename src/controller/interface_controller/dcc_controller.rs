use gtk4::glib::{Sender, Receiver, self};

use crate::{client::async_reader::AsyncReader, controller::controller_message::ControllerMessage};

use super::InterfaceController;

use std::thread;

impl InterfaceController {
    pub fn start_listening_dcc(&mut self, sender: Sender<String>) {
        let sender_clone = self.sender.clone();
        let (_async_reader, message_receiver) =
            AsyncReader::spawn(self.client.get_stream().expect("error"));
        thread::spawn(move || {
            while let Ok(message_received) = message_receiver.recv() {
                match message_received {
                    Ok(message) => {
                        // let controller_message = to_controller_message(message);
                        // sender_clone.send(controller_message).unwrap();
                    }
                    Err(error) => eprintln!("{error}"),
                }
            }
        });
    }

    pub fn receiver_attach(&mut self, client: String, dcc_receiver: Receiver<String>, sender: Sender<ControllerMessage>) {
        dcc_receiver.attach(None, move |msg| {
            sender.send(ReceiveSafeMessage { client, msg });
            glib::Continue(true)
        });
    }
}