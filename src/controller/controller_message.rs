use std::collections::HashMap;

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
    ErrorWhenAddingChannel{
        message: String,
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
    OpenSafeConversationView {},
    OpenUserInfoView {},
    OpenWarningView {
        message: String,
    },
    Quit {},
    QuitChannel {},
    ReceiveInvite {
        message: Message,
    },
    ReceiveJoin {message: Message},
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
    SendListMessage {},
    SendNamesMessageToAddClient {},
    SendNamesMessageToInviteClient {},
    SendNamesMessageToKnowMembers {},
    SendPrivMessage {
        message: GString,
    },
    ToRegister {
        address: String,
    },
}
