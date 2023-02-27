use gtk4::{glib::Sender, Application, ApplicationWindow};

use std::net::SocketAddr;

use crate::{
    controller::controller_message::ControllerMessage,
    views::{
        add_views::{
            add_channel_view::AddChannelView, add_client_view::AddClientView,
            channel_members_view::ChannelMembersView, dcc_invitation_view::DccInvitationView,
            invite_view::InviteView, notifications_view::NotificationsView,
            safe_conversation_view::SafeConversationView, user_info_view::UserInfoView,
            warning_view::WarningView,
        },
        ip_view::IpView,
        main_view::MainView,
        register_view::RegisterView,
    },
};

pub fn add_channel_view(sender: &Sender<ControllerMessage>) -> AddChannelView {
    AddChannelView::new(sender.clone())
}

pub fn add_client_window(
    app: &Application,
    clients: Vec<String>,
    sender: &Sender<ControllerMessage>,
) -> ApplicationWindow {
    AddClientView::new(clients, sender.clone()).get_view(app.clone())
}

pub fn channel_members_window(
    app: &Application,
    channel: String,
    clients: Vec<String>,
    nickname: String,
    sender: &Sender<ControllerMessage>,
) -> ApplicationWindow {
    ChannelMembersView::new(channel, clients, nickname, sender.clone()).get_view(app.clone())
}

pub fn dcc_invitation_window(
    app: &Application,
    client: String,
    address: SocketAddr,
    sender: &Sender<ControllerMessage>,
) -> ApplicationWindow {
    DccInvitationView::new(sender.clone()).get_view(app.clone(), client, address)
}

pub fn invite_window(
    app: &Application,
    channels: Vec<String>,
    sender: &Sender<ControllerMessage>,
) -> ApplicationWindow {
    InviteView::new(channels, sender.clone()).get_view(app.clone())
}

pub fn ip_window(app: &Application, sender: &Sender<ControllerMessage>) -> ApplicationWindow {
    IpView::new(sender.clone()).get_view(app.clone())
}

pub fn main_view(sender: &Sender<ControllerMessage>) -> MainView {
    MainView::new(sender.clone())
}

pub fn notifications_window(app: &Application, notifications: Vec<String>) -> ApplicationWindow {
    NotificationsView::new(notifications).get_view(app.clone())
}

pub fn register_window(app: &Application, sender: &Sender<ControllerMessage>) -> ApplicationWindow {
    RegisterView::new(sender.clone()).get_view(app.clone())
}

pub fn safe_conversation_view(
    client: &str,
    nickname: String,
    sender: &Sender<ControllerMessage>,
) -> SafeConversationView {
    SafeConversationView::new(client, nickname, sender.clone())
}

pub fn user_info_window(
    app: &Application,
    nickname: String,
    realname: String,
    servername: String,
    username: String,
) -> ApplicationWindow {
    UserInfoView::new(nickname, realname, servername, username).get_view(app.clone())
}

pub fn warning_window(app: &Application, warning_text: String) -> ApplicationWindow {
    WarningView::new(warning_text).get_view(app.clone())
}
