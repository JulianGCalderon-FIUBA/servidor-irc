use gtk4::traits::WidgetExt;
use std::{thread, net::SocketAddr};

use crate::{controller::utils::get_sender_and_receiver, ctcp::dcc_chat::dcc_chat_receiver::DccChatReceiver};

use super::{InterfaceController, window_creation::{safe_conversation_view, dcc_invitation_window}};

impl InterfaceController {
    pub fn dcc_receive_accept(&mut self, client: String) {
        let mut dcc_chat = self.dcc_senders.remove(&client).unwrap().accept().unwrap();
        let dcc_std_receiver = dcc_chat.async_read_message();
        self.dcc_chats.insert(client.clone(), dcc_chat);

        let (dcc_sender, dcc_receiver) = get_sender_and_receiver();

        thread::spawn(move || {
            while let Ok(message_received) = dcc_std_receiver.recv() {
                dcc_sender.send(message_received).expect("error");
            }
        });

        self.receiver_attach(client.clone(), dcc_receiver, self.sender.clone());

        self.main_view.disable_safe_conversation_button();

        let mut safe_conversation = safe_conversation_view(self.nickname.clone(), &self.sender);
        safe_conversation
            .get_view(&client, self.app.clone())
            .show();
        self.safe_conversation_view.insert(client, safe_conversation);
    }

    pub fn dcc_receive_decline(&mut self, client: String) {
        self.dcc_senders.remove(&client).unwrap().close();
    }

    pub fn open_dcc_invitation_view(&mut self, client: String, message: SocketAddr) {
        let stream = self.get_stream();
        let dcc_receiver = DccChatReceiver::new(stream, client.clone());
        self.dcc_receivers.insert(client.clone(), dcc_receiver);

        self.dcc_invitation_window =
            dcc_invitation_window(&self.app, client, message, &self.sender);
        self.dcc_invitation_window.show();
    }

    pub fn receive_dcc_close(&mut self, _client: String) {

    }
}