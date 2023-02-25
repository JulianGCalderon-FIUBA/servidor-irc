use std::{collections::HashMap, path::PathBuf, net::SocketAddr};

use gtk4::glib::GString;

use crate::message::Message;

/// Possible messages or requests a Controller can receive.
pub enum ControllerMessage {
    AcceptDccChat {
        client: String,
        address: SocketAddr,
    },
    AddNewClient {
        new_client: GString,
    },
    ChangeConversation {
        nickname: String,
    },
    ErrorWhenAddingChannel {
        message: String,
    },
    OpenDccInvitationView {
        client: String,
        message: SocketAddr,
    },
    DccreceiveAccept {
        client: String,
    },
    DccreceiveDecline {
        client: String,
    },
    DeclineDccChat {
        client: String,
    },
    JoinChannel {
        channel: String,
    },
    KickMember {
        channel: String,
        member: String,
    },
    OpenAddClientView {
        channels_and_clients: HashMap<String, Vec<String>>,
    },
    OpenInviteClientView {
        channels_and_clients: HashMap<String, Vec<String>>,
    },
    OpenMainView {
        message: Message,
    },
    OpenNotificationsView {},
    OpenFileDialogChooserView {},
    SendSafeConversationRequest {},
    OpenUserInfoView {},
    OpenWarningView {
        message: String,
    },
    ReceiveInvite {
        message: Message,
    },
    ReceiveJoin {
        message: Message,
    },
    ReceiveKick {
        message: Message,
    },
    ReceiveListEnd {},
    ReceiveNamesEnd {},
    ReceivePrivMessage {
        message: Message,
    },
    ReceiveListLine {
        message: Message,
    },
    ReceiveNamesLine {
        message: Message,
    },
    Register {
        pass: GString,
        nickname: GString,
        username: GString,
        realname: GString,
    },
    RegularMessage {
        message: String,
    },
    RemoveConversation {},
    ReceiveSafeMessage {
        client: String,
        message: String,
    },
    SendInviteMessage {
        channel: GString,
    },
    SendJoinMessage {
        channel: String,
    },
    SendKickMessage {
        channel: String,
        member: String,
    },
    SendListMessage {},
    SendNamesMessageToAddClient {},
    SendNamesMessageToInviteClient {},
    SendNamesMessageToKnowMembers {},
    SendPartMessage {},
    SendPrivMessage {
        message: GString,
    },
    SendQuitMessage {},
    SendSafeMessage {
        client: String, 
        message: String,
    },
    ToRegister {
        address: String,
    },
    SendFile {
        target: String,
        path: PathBuf,
    },
    DownloadFile {
        sender: String,
        path: PathBuf,
    },
    IgnoreFile {
        sender: String,
    },
}
