use std::{collections::HashMap, io, net::SocketAddr, path::PathBuf};

use crate::message::Message;

/// Possible messages or requests a Controller can receive.
pub enum ControllerMessage {
    AcceptDccChat {
        client: String,
        address: SocketAddr,
    },
    AddNewClient {
        new_client: String,
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
    ReceiveJoinNotification {
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
        pass: String,
        nickname: String,
        username: String,
        realname: String,
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
        channel: String,
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
        message: String,
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
