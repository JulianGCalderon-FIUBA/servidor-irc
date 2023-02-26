use std::{ collections::HashMap, io, net::SocketAddr, path::PathBuf };

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
    DeclineDccChat {
        client: String,
    },
    DownloadFile {
        sender: String,
        path: PathBuf,
    },
    ErrorWhenAddingChannel {
        message: String,
    },
    IgnoreFile {
        sender: String,
    },
    JoinChannel {
        channel: String,
    },
    KickMember {
        channel: String,
        member: String,
    },
    OpenAddChannelView {},
    OpenAddClientView {
        channels_and_clients: HashMap<String, Vec<String>>,
    },
    OpenDccInvitationView {
        client: String,
        message: SocketAddr,
    },
    OpenFileDialogChooserView {},
    OpenInviteClientView {
        channels_and_clients: HashMap<String, Vec<String>>,
    },
    OpenMainView {
        message: Message,
    },
    OpenNotificationsView {},
    OpenRegisterView {
        address: String,
    },
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
    ReceiveListLine {
        message: Message,
    },
    ReceiveNamesEnd {},
    ReceiveNamesLine {
        message: Message,
    },
    ReceivePrivMessage {
        message: Message,
    },
    ReceiveResult {
        sender: String,
        name: String,
        result: Result<(), std::io::Error>,
    },
    ReceiveSafeMessage {
        client: String,
        message: String,
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
    SendDccSend {
        target: String,
        path: PathBuf,
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
    SendResult {
        sender: String,
        result: io::Result<()>,
    },
    SendSafeConversationRequest {},
    SendSafeMessage {
        client: String,
        message: String,
    },
}