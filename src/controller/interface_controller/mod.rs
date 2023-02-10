pub mod message_matcher;
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
    glib::{self, Sender},
    prelude::*,
    Application, ApplicationWindow,
};

use crate::controller::ControllerMessage::*;

use super::controller_message::ControllerMessage;

pub struct InterfaceController {
    app: Application,
    client: Client,
    sender: Sender<ControllerMessage>,
    ip_window: ApplicationWindow,
    register_window: ApplicationWindow,
    main_view: MainView,
    main_window: ApplicationWindow,
    add_channel_window: ApplicationWindow,
    add_client_window: ApplicationWindow,
    invite_window: ApplicationWindow,
    current_conv: String,
    current_nickname: String,
    current_realname: String,
    current_servername: String,
    current_username: String,
    trying_to_add_client: bool,
    trying_to_invite_client: bool,
}

impl InterfaceController {
    /// Creates new [`Client`] connected to received address.
    pub fn new(app: Application, client: Client) -> Self {
        let (sender, _) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
        Self {
            app: app.clone(),
            client,
            sender: sender.clone(),
            ip_window: IpView::new(sender.clone()).get_view(app.clone()),
            register_window: RegisterView::new(sender.clone()).get_view(app.clone()),
            main_view: MainView::new(sender.clone()),
            main_window: MainView::new(sender.clone()).get_view(app.clone(), "".to_string()),
            add_channel_window: AddChannelView::new(sender.clone()).get_view(app.clone(), vec![]),
            add_client_window: AddClientView::new(sender.clone()).get_view(app.clone(), vec![]),
            invite_window: InviteView::new(sender).get_view(app, vec![]),
            current_conv: "".to_string(),
            current_nickname: String::from(""),
            current_realname: String::from(""),
            current_servername: String::from(""),
            current_username: String::from(""),
            trying_to_add_client: false,
            trying_to_invite_client: false,
        }
    }

    pub fn initialize_values(&mut self) {
        self.ip_window = IpView::new(self.sender.clone()).get_view(self.app.clone());

        self.register_window = RegisterView::new(self.sender.clone()).get_view(self.app.clone());

        self.main_view = MainView::new(self.sender.clone());
        self.main_window =
            MainView::new(self.sender.clone()).get_view(self.app.clone(), "".to_string());

        self.add_channel_window =
            AddChannelView::new(self.sender.clone()).get_view(self.app.clone(), vec![]);

        self.add_client_window =
            AddClientView::new(self.sender.clone()).get_view(self.app.clone(), vec![]);

        self.invite_window =
            InviteView::new(self.sender.clone()).get_view(self.app.clone(), vec![]);
    }

    pub fn build(mut self) {
        let (sender, receiver) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
        self.sender = sender;

        self.initialize_values();

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
