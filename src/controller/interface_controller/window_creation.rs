use gtk4::{glib::Sender, Application, ApplicationWindow};

use std::net::SocketAddr;

use crate::{
    controller::controller_message::ControllerMessage,
    views::{
        add_views::{
            add_channel_view::AddChannelView, add_client_view::AddClientView,
            channel_members_view::ChannelMembersView, close_safe_conv_view::CloseSafeConvView,
            dcc_invitation_view::DccInvitationView, invite_view::InviteView,
            notifications_view::NotificationsView, safe_conversation_view::SafeConversationView,
            user_info_view::UserInfoView, warning_view::WarningView,
        },
        ip_view::IpView,
        main_view::MainView,
        register_view::RegisterView,
    },
};

/// Creates new [`AddChannelView`].
pub fn add_channel_view(sender: &Sender<ControllerMessage>) -> AddChannelView {
    AddChannelView::new(sender.clone())
}

/// Creates new [`AddClientView`] window.
pub fn add_client_window(
    app: &Application,
    clients: Vec<String>,
    sender: &Sender<ControllerMessage>,
) -> ApplicationWindow {
    AddClientView::new(clients, sender.clone()).get_view(app.clone())
}

/// Creates new [`ChannelMembersView`] window.
pub fn channel_members_window(
    app: &Application,
    channel: String,
    clients: Vec<String>,
    nickname: String,
    sender: &Sender<ControllerMessage>,
) -> ApplicationWindow {
    ChannelMembersView::new(channel, clients, nickname, sender.clone()).get_view(app.clone())
}

/// Creates new [`CloseSafeConvView`] window.
pub fn close_safe_conv_window(
    app: &Application,
    client: String,
    sender: &Sender<ControllerMessage>,
) -> ApplicationWindow {
    CloseSafeConvView::new(sender.clone()).get_view(app.clone(), client)
}

/// Creates new [`DccInvitationView`] window.
pub fn dcc_invitation_window(
    app: &Application,
    client: String,
    address: SocketAddr,
    sender: &Sender<ControllerMessage>,
) -> ApplicationWindow {
    DccInvitationView::new(sender.clone()).get_view(app.clone(), client, address)
}

/// Creates new [`InviteView`] window.
pub fn invite_window(
    app: &Application,
    channels: Vec<String>,
    sender: &Sender<ControllerMessage>,
) -> ApplicationWindow {
    InviteView::new(channels, sender.clone()).get_view(app.clone())
}

/// Creates new [`IpView`] window.
pub fn ip_window(app: &Application, sender: &Sender<ControllerMessage>) -> ApplicationWindow {
    IpView::new(sender.clone()).get_view(app.clone())
}

/// Creates new [`MainView`].
pub fn main_view(sender: &Sender<ControllerMessage>) -> MainView {
    MainView::new(sender.clone())
}

/// Creates new [`NotificationsView`] window.
pub fn notifications_window(app: &Application, notifications: Vec<String>) -> ApplicationWindow {
    NotificationsView::new(notifications).get_view(app.clone())
}

/// Creates new [`RegisterView`] window.
pub fn register_window(app: &Application, sender: &Sender<ControllerMessage>) -> ApplicationWindow {
    RegisterView::new(sender.clone()).get_view(app.clone())
}

/// Creates new [`SafeConversationView`].
pub fn safe_conversation_view(
    client: &str,
    nickname: String,
    sender: &Sender<ControllerMessage>,
) -> SafeConversationView {
    SafeConversationView::new(client, nickname, sender.clone())
}

/// Creates new [`UserInfoView`] window.
pub fn user_info_window(
    app: &Application,
    nickname: String,
    realname: String,
    servername: String,
    username: String,
) -> ApplicationWindow {
    UserInfoView::new(nickname, realname, servername, username).get_view(app.clone())
}

/// Creates new [`WarningView`] window.
pub fn warning_window(app: &Application, warning_text: String) -> ApplicationWindow {
    WarningView::new(warning_text).get_view(app.clone())
}
