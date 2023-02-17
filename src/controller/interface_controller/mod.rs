pub mod decode_message;
pub mod message_matcher;
pub mod names_message_intention;
pub mod utils;
pub mod window_creation;

use gtk4 as gtk;

use crate::{client::client::Client, views::main_view::MainView, ctcp::dcc_chat_sender::DccChatSender};
use gtk::{
    glib::{self, Receiver, Sender},
    prelude::*,
    Application, ApplicationWindow,
};
use std::collections::HashMap;

use crate::controller::controller_message::ControllerMessage::{
    OpenAddClientView, OpenInviteClientView, OpenWarningView,
};

use crate::controller::ControllerMessage::*;

use self::{
    names_message_intention::NamesMessageIntention::{self, Undefined},
    window_creation::{
        add_channel_window, add_client_window, invite_window, ip_window, main_view, main_window,
        register_window,
    },
};

use super::controller_message::ControllerMessage;

pub struct InterfaceController {
    accumulated_channels_from_list: Vec<String>,
    accumulated_channels_from_names: Vec<String>,
    accumulated_clients_from_names: Vec<Vec<String>>,
    add_channel_window: ApplicationWindow,
    add_client_window: ApplicationWindow,
    app: Application,
    client: Client,
    current_conv: String,
    invite_window: ApplicationWindow,
    ip_window: ApplicationWindow,
    main_view: MainView,
    main_window: ApplicationWindow,
    names_message_intention: NamesMessageIntention,
    nickname: String,
    realname: String,
    register_window: ApplicationWindow,
    safe_conversations: HashMap<String, DccChatSender>,
    sender: Sender<ControllerMessage>,
    servername: String,
    username: String,
}

impl InterfaceController {
    /// Creates new [`Client`] connected to received address.
    pub fn new(app: Application, client: Client, sender: Sender<ControllerMessage>) -> Self {
        Self {
            accumulated_channels_from_list: vec![],
            accumulated_channels_from_names: vec![],
            accumulated_clients_from_names: vec![],
            add_channel_window: add_channel_window(&app, vec![], &sender),
            add_client_window: add_client_window(&app, vec![], &sender),
            app: app.clone(),
            client,
            current_conv: String::new(),
            invite_window: invite_window(&app, vec![], &sender),
            ip_window: ip_window(&app, &sender),
            main_view: main_view(&sender),
            main_window: main_window(&app, String::new(), &sender),
            names_message_intention: Undefined,
            nickname: String::new(),
            realname: String::new(),
            register_window: register_window(&app, &sender),
            safe_conversations: HashMap::new(),
            sender,
            servername: String::new(),
            username: String::new(),
        }
    }

    pub fn build(mut self, receiver: Receiver<ControllerMessage>) {
        self.ip_window.show();

        receiver.attach(None, move |msg| {
            match msg {
                AcceptDccChat { client, address } => {
                    println!("DCC CHAT ACCEPTED");
                }
                AddNewClient { new_client } => {
                    self.add_new_client(new_client);
                }
                ChangeConversation { nickname } => {
                    self.change_conversation(nickname);
                }
                DccInvitation { client, message } => {
                    self.dcc_invitation(client, message);
                }
                JoinChannel { channel } => {
                    self.join_channel(channel);
                }
                KickMember { channel, member } => {
                    self.kick_member(channel, member);
                }
                OpenAddClientView {
                    channels_and_clients,
                } => {
                    self.open_add_client_view(channels_and_clients);
                }
                OpenInviteClientView {
                    channels_and_clients,
                } => {
                    self.open_invite_client_view(channels_and_clients);
                }
                OpenMainView { message } => {
                    self.open_main_view(message);
                }
                OpenNotificationsView {} => {
                    self.open_notifications_view();
                }
                OpenUserInfoView {} => {
                    self.open_user_info_view();
                }
                OpenWarningView { message } => {
                    self.open_warning_view(message);
                }
                Quit {} => {
                    self.quit();
                }
                QuitChannel {} => {
                    self.quit_channel();
                }
                ReceiveInvite { message } => {
                    self.receive_invite(message);
                }
                ReceiveKick { message } => {
                    self.receive_kick(message);
                }
                ReceiveListEnd {} => {
                    self.receive_list_end();
                }
                ReceiveListLine { message } => {
                    self.receive_list_line(message);
                }
                ReceiveNamesLine { message } => {
                    self.receive_names_line(message);
                }
                ReceiveNamesEnd {} => {
                    self.receive_names_end();
                }
                ReceivePrivMessage { message } => {
                    self.receive_priv_message(message);
                }
                Register {
                    pass,
                    nickname,
                    username,
                    realname,
                } => {
                    self.register(pass, nickname, username, realname);
                }
                RegularMessage { message } => {
                    self.regular_message(message);
                }
                RemoveConversation {} => {
                    self.remove_conversation();
                }
                SafeConversationRequest { } => {
                    self.send_safe_conversation_request();
                }
                SendInviteMessage { channel } => {
                    self.send_invite_message(channel);
                }
                SendListMessage {} => {
                    self.send_list_message();
                }
                SendNamesMessageToAddClient {} => {
                    self.send_names_message_to_add_client();
                }
                SendNamesMessageToInviteClient {} => {
                    self.send_names_message_to_invite_client();
                }
                SendNamesMessageToKnowMembers {} => {
                    self.send_names_message_to_know_members();
                }
                SendPrivMessage { message } => {
                    self.send_priv_message(message);
                }
                ToRegister { address } => {
                    self.to_register(address);
                }
            }
            // Returning false here would close the receiver
            // and have senders fail
            glib::Continue(true)
        });
    }
}
