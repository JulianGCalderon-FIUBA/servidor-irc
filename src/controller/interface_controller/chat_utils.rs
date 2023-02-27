use std::{thread, net::SocketAddr};
use gtk4::{
    traits::{GtkWindowExt, WidgetExt},
};

use crate::{controller::utils::get_sender_and_receiver, ctcp::dcc_chat::dcc_chat_receiver::DccChatReceiver};

use super::{InterfaceController, window_creation::{safe_conversation_view, dcc_invitation_window, close_safe_conv_window}};

impl InterfaceController {
    /// Handles a dcc accept reception.
    /// Creates new safe conversation view and starts listening from that stream.
    /// 
    /// Receives a string, returns nothing.
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

        let mut safe_conversation = safe_conversation_view(&client, self.nickname.clone(), &self.sender);
        let safe_view = safe_conversation.get_view(self.app.clone());
        safe_view.show();
        
        self.safe_conversation_view.insert(client.clone(), safe_conversation);
        self.safe_conversation_window.insert(client, safe_view);
    }

    /// Handles a dcc decline reception.
    /// 
    /// Receives a string, returns nothing.
    pub fn dcc_receive_decline(&mut self, client: String) {
        self.dcc_senders.remove(&client).unwrap().close();
    }

    /// Handles a new dcc invitation.
    /// Opens the dcc invitation view.
    /// 
    /// Receives a string and an address, returns nothing.
    pub fn open_dcc_invitation_view(&mut self, client: String, message: SocketAddr) {
        let stream = self.get_stream();
        let dcc_receiver = DccChatReceiver::new(stream, client.clone());
        self.dcc_receivers.insert(client.clone(), dcc_receiver);

        self.dcc_invitation_window =
            dcc_invitation_window(&self.app, client, message, &self.sender);
        self.dcc_invitation_window.show();
    }

    /// Handles a dcc close reception.
    /// Shows dcc close pop up.
    /// 
    /// Receives a string, returns nothing.
    pub fn receive_dcc_close(&mut self, client: String) {
        close_safe_conv_window(&self.app, client, &self.sender).show()
    }

    /// Closes dcc chat.
    /// 
    /// Receives a string, returns nothing.
    pub fn dcc_close(&mut self, client: String) {
        self.dcc_chats.remove(&client);
        self.safe_conversation_view.remove(&client);
        let safe_conversation = self.safe_conversation_window.remove(&client).unwrap();
        safe_conversation.close();

        self.main_view.update_safe_conversation_button(&client, &self.dcc_chats);
    }
}