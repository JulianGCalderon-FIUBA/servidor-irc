use gtk4::glib::{self, Receiver, Sender};

use crate::controller::controller_message::ControllerMessage;

use crate::controller::controller_message::ControllerMessage::ReceiveSafeMessage;

use super::InterfaceController;

impl InterfaceController {
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
