use std::collections::HashMap;

use crate::message::Message;

/// Possible messages or requests a Controller can receive.
pub enum ControllerMessage {
    AddNewClient {
        new_client: String,
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
        pass: String,
        nickname: String,
        username: String,
        realname: String,
    },
    RegularMessage {
        message: String,
    },
    RemoveConversation {},
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
    ToRegister {
        address: String,
    },
}
