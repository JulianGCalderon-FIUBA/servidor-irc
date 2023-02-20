pub mod decode_message;
pub mod message_matcher;
pub mod names_message_intention;
pub mod utils;
pub mod window_creation;

use std::thread;

use gtk4 as gtk;

use crate::{
    client::{Client, async_reader::AsyncReader},
    views::{add_views::add_channel_view::AddChannelView, main_view::MainView},
};
use gtk::{
    glib::{self, Receiver, Sender},
    prelude::*,
    Application, ApplicationWindow,
};

use crate::controller::controller_message::ControllerMessage::{
    OpenAddClientView, OpenInviteClientView, OpenWarningView,
};

use crate::controller::ControllerMessage::*;

use self::{
    names_message_intention::NamesMessageIntention::{self, Undefined},
    window_creation::{
        add_channel_view, add_client_window, invite_window, ip_window, main_view, register_window,
    },
};

use super::{controller_message::ControllerMessage, controller_handler::to_controller_message};

const FAILED_TO_READ_MESSAGE_ERROR_TEXT: &str = "Failed to read message";

pub struct InterfaceController {
    accumulated_channels_from_list: Vec<String>,
    accumulated_channels_from_names: Vec<String>,
    accumulated_clients_from_names: Vec<Vec<String>>,
    add_channel_view: AddChannelView,
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
            add_channel_view: add_channel_view(&sender),
            add_channel_window: add_channel_view(&sender).get_view(app.clone(), vec![]),
            add_client_window: add_client_window(&app, vec![], &sender),
            app: app.clone(),
            client,
            current_conv: String::new(),
            invite_window: invite_window(&app, vec![], &sender),
            ip_window: ip_window(&app, &sender),
            main_view: main_view(&sender),
            main_window: main_view(&sender).get_view(app.clone(), String::new()),
            names_message_intention: Undefined,
            nickname: String::new(),
            realname: String::new(),
            register_window: register_window(&app, &sender),
            sender,
            servername: String::new(),
            username: String::new(),
        }
    }

    pub fn start_listening(&mut self){
        let sender_clone = self.sender.clone();
        let (_async_reader, message_receiver) =
            AsyncReader::spawn(self.client.get_stream().expect("error"));
        thread::spawn(move || {
            while let Ok(message_received) = message_receiver.recv() {
                match message_received {
                    Ok(message) => {
                        let controller_message = to_controller_message(message);
                        sender_clone.send(controller_message).unwrap();
                    }
                    Err(error) => eprintln!("{FAILED_TO_READ_MESSAGE_ERROR_TEXT}: {error}"),
                }
            }
        });
    }

    pub fn build(mut self, receiver: Receiver<ControllerMessage>) {
        self.ip_window.show();

        receiver.attach(None, move |msg| {
            match msg {
                AddNewClient { new_client } => {
                    self.add_new_client(new_client);
                }
                ChangeConversation { nickname } => {
                    self.change_conversation(nickname);
                }
                ErrorWhenAddingChannel { message } => {
                    self.error_when_adding_channel(message);
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
                OpenSafeConversationView {} => {
                    self.open_safe_conversation_view();
                }
                OpenUserInfoView {} => {
                    self.open_user_info_view();
                }
                OpenWarningView { message } => {
                    self.open_warning_view(message);
                }
                ReceiveInvite { message } => {
                    self.receive_invite(message);
                }
                ReceiveJoin { message } => {
                    self.receive_join(message);
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
                SendInviteMessage { channel } => {
                    self.send_invite_message(channel);
                }
                SendJoinMessage { channel } => {
                    self.send_join_message(channel);
                }
                SendKickMessage { channel, member } => {
                    self.send_kick_message(channel, member);
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
                SendPartMessage {} => {
                    self.send_part_message();
                }
                SendPrivMessage { message } => {
                    self.send_priv_message(message);
                }
                SendQuitMessage {} => {
                    self.send_quit_message();
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
