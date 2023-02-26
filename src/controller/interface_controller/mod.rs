pub mod dcc_controller;
pub mod decode_message;
mod download;
pub mod message_matcher;
pub mod names_message_intention;
mod resume_utils;
pub mod utils;
pub mod window_creation;

use std::{
    collections::HashMap,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    thread,
};

use gtk4 as gtk;

use crate::{
    client::{async_reader::AsyncReader, Client},
    ctcp::dcc_send::dcc_resume_sender::DccResumeSender,
    ctcp::{
        dcc_chat::{dcc_chat_receiver::DccChatReceiver, dcc_chat_sender::DccChatSender, DccChat},
        dcc_send::{dcc_send_receiver::DccSendReceiver, dcc_send_sender::DccSendSender},
    },
    views::{
        add_views::add_channel_view::AddChannelView,
        add_views::safe_conversation_view::SafeConversationView, main_view::MainView,
    },
};
use gtk::{
    glib::{self, Receiver, Sender},
    traits::WidgetExt,
    Application, ApplicationWindow, MessageDialog,
};

use crate::controller::ControllerMessage::*;

use self::{
    download::Download,
    names_message_intention::NamesMessageIntention::{self, Undefined},
    window_creation::{
        add_channel_view, add_client_window, dcc_invitation_window, invite_window, ip_window,
        main_view, register_window, safe_conversation_view,
    },
};

use super::{controller_handler::to_controller_message, controller_message::ControllerMessage};

const FAILED_TO_READ_MESSAGE_ERROR_TEXT: &str = "Failed to read message";

pub struct InterfaceController {
    accumulated_channels_from_list: Vec<String>,
    accumulated_channels_from_names: Vec<String>,
    accumulated_clients_from_names: Vec<Vec<String>>,
    add_channel_view: AddChannelView,
    add_channel_window: ApplicationWindow,
    add_client_window: ApplicationWindow,
    app: Application,
    cancel_dialogs: HashMap<String, MessageDialog>,
    client: Client,
    current_conv: String,
    dcc_chats: HashMap<String, DccChat>,
    dcc_invitation_window: ApplicationWindow,
    dcc_receivers: HashMap<String, DccChatReceiver>,
    dcc_resume_senders: HashMap<String, DccResumeSender>,
    dcc_send_receivers: HashMap<String, DccSendReceiver>,
    dcc_send_senders: HashMap<String, DccSendSender>,
    dcc_senders: HashMap<String, DccChatSender>,
    downloads: Vec<Download>,
    invite_window: ApplicationWindow,
    ip_window: ApplicationWindow,
    main_view: MainView,
    main_window: ApplicationWindow,
    names_message_intention: NamesMessageIntention,
    nickname: String,
    realname: String,
    register_window: ApplicationWindow,
    safe_conversation_view: SafeConversationView,
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
            dcc_invitation_window: dcc_invitation_window(
                &app,
                String::new(),
                SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080),
                &sender,
            ),
            dcc_receivers: HashMap::new(),
            dcc_senders: HashMap::new(),
            dcc_chats: HashMap::new(),
            invite_window: invite_window(&app, vec![], &sender),
            ip_window: ip_window(&app, &sender),
            main_view: main_view(&sender),
            main_window: main_view(&sender).get_view(app.clone(), String::new()),
            names_message_intention: Undefined,
            nickname: String::new(),
            realname: String::new(),
            register_window: register_window(&app, &sender),
            safe_conversation_view: safe_conversation_view("".to_string(), &sender),
            sender,
            servername: String::new(),
            username: String::new(),
            dcc_send_senders: HashMap::new(),
            dcc_send_receivers: HashMap::new(),
            cancel_dialogs: HashMap::new(),
            downloads: Vec::new(),
            dcc_resume_senders: HashMap::new(),
        }
    }

    pub fn start_listening(&mut self) {
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
                AcceptDccChat { client, address } => {
                    self.accept_dcc_chat(client, address);
                }
                AddNewClient { new_client } => {
                    self.add_new_client(new_client);
                }
                ChangeConversation { nickname } => {
                    self.change_conversation(nickname);
                }
                DeclineDccChat { client } => {
                    self.decline_dcc_chat(client);
                }
                DownloadFile { sender, path } => {
                    self.download_file(sender, path);
                }
                ErrorWhenAddingChannel { message } => {
                    self.error_when_adding_channel(message);
                }
                IgnoreFile { sender } => {
                    self.ignore_file(sender);
                }
                JoinChannel { channel } => {
                    self.join_channel(channel);
                }
                KickMember { channel, member } => {
                    self.kick_member(channel, member);
                }
                OpenAddChannelView {} => {
                    self.open_add_channel_view();
                }
                OpenAddClientView {
                    channels_and_clients,
                } => self.open_add_client_view(channels_and_clients),
                OpenDccInvitationView { client, message } => {
                    self.open_dcc_invitation_view(client, message);
                }
                OpenFileDialogChooserView {} => {
                    self.open_file_chooser_dialog_view();
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
                OpenRegisterView { address } => {
                    self.open_register_view(address);
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
                ReceiveJoinNotification { message } => {
                    self.receive_join_notification(message);
                }
                ReceiveKick { message } => {
                    self.receive_kick(message);
                }
                ReceiveListLine { message } => {
                    self.receive_list_line(message);
                }
                ReceiveNamesEnd {} => {
                    self.receive_names_end();
                }
                ReceiveNamesLine { message } => {
                    self.receive_names_line(message);
                }
                ReceivePrivMessage { message } => {
                    self.receive_priv_message(message);
                }
                ReceiveResult {
                    sender,
                    name,
                    result,
                } => {
                    self.receive_result(sender, name, result);
                }
                ReceiveSafeMessage { client, message } => {
                    self.receive_safe_message(client, message);
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
                SendDccSend { path, target } => {
                    self.send_dcc_send(target, path);
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
                SendResult { sender, result } => {
                    self.send_result(sender, result);
                }
                SendSafeConversationRequest {} => {
                    self.send_safe_conversation_request();
                }
                SendSafeMessage { client, message } => {
                    self.send_safe_message(client, message);
                }
            }
            // Returning false here would close the receiver
            // and have senders fail
            glib::Continue(true)
        });
    }
}
