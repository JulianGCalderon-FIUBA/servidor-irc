pub mod message_matcher;
pub mod names_message_intention;
pub mod utils;

use gtk4 as gtk;

use crate::{
    client::Client,
    views::{
        add_views::{
            add_channel_view::AddChannelView, add_client_view::AddClientView,
            invite_view::InviteView,
        },
        ip_view::IpView,
        main_view::MainView,
        register_view::RegisterView,
    },
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

use self::names_message_intention::NamesMessageIntention::{self, Undefined};

use super::controller_message::ControllerMessage;

pub struct InterfaceController {
    add_channel_window: ApplicationWindow,
    add_client_window: ApplicationWindow,
    app: Application,
    client: Client,
    current_conv: String,
    current_nickname: String,
    current_realname: String,
    current_servername: String,
    current_username: String,
    invite_window: ApplicationWindow,
    ip_window: ApplicationWindow,
    main_view: MainView,
    main_window: ApplicationWindow,
    register_window: ApplicationWindow,
    sender: Sender<ControllerMessage>,
    names_message_intention: NamesMessageIntention,
}

impl InterfaceController {
    /// Creates new [`Client`] connected to received address.
    pub fn new(app: Application, client: Client, sender: Sender<ControllerMessage>) -> Self {
        Self {
            add_channel_window: AddChannelView::new(sender.clone()).get_view(app.clone(), vec![]),
            add_client_window: AddClientView::new(sender.clone()).get_view(app.clone(), vec![]),
            app: app.clone(),
            client,
            current_conv: String::new(),
            current_nickname: String::new(),
            current_realname: String::new(),
            current_servername: String::new(),
            current_username: String::new(),
            invite_window: InviteView::new(sender.clone()).get_view(app.clone(), vec![]),
            ip_window: IpView::new(sender.clone()).get_view(app.clone()),
            main_view: MainView::new(sender.clone()),
            main_window: MainView::new(sender.clone()).get_view(app.clone(), String::new()),
            register_window: RegisterView::new(sender.clone()).get_view(app),
            sender,
            names_message_intention: Undefined
        }
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
                OpenMainView {
                    realname,
                    servername,
                    nickname,
                    username,
                } => {
                    self.open_main_view(realname, servername, nickname, username);
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
                Quit {} => {
                    self.quit();
                }
                QuitChannel {} => {
                    self.quit_channel();
                }
                ReceiveInvite { nickname, channel } => {
                    self.receive_invite(channel, nickname);
                }
                ReceiveKick { kicked, channel } => {
                    self.receive_kick(channel, kicked);
                }
                ReceiveListChannels { channels } => {
                    self.receive_list_channels(channels);
                }
                ReceiveNamesChannels {
                    channels_and_clients,
                } => {
                    self.receive_names_channels(channels_and_clients);
                }
                ReceivePrivMessage {
                    sender_nickname,
                    message,
                    channel,
                } => {
                    self.receive_priv_message(channel, message, sender_nickname);
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
