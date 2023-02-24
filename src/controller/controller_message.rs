use std::{collections::HashMap, io, path::PathBuf};

use gtk4::glib::GString;

use crate::message::Message;

/// Possible messages or requests a Controller can receive.
pub enum ControllerMessage {
    AddNewClient {
        new_client: GString,
    },
    ChangeConversation {
        nickname: String,
    },
    ErrorWhenAddingChannel {
        message: String,
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
    OpenSafeConversationView {},
    OpenFileDialogChooserView {},
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
    SendResult {
        sender: String,
        result: io::Result<()>,
    },
    ReceiveResult {
        sender: String,
        name: String,
        result: Result<(), std::io::Error>,
    },
}
