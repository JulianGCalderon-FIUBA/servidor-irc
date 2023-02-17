use std::collections::HashMap;

use gtk4::glib::GString;

use crate::{message::Message, ctcp::dcc_message::DccMessage};

/// Possible messages or requests a Controller can receive.
pub enum ControllerMessage {
    AcceptDccChat {
        client: String,
        address: String,
    },
    AddNewClient {
        new_client: GString,
    },
    ChangeConversation {
        nickname: String,
    },
    DccInvitation {
        client: String,
        message: DccMessage,
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
    SafeConversationRequest {},
    OpenUserInfoView {},
    OpenWarningView {
        message: String,
    },
    Quit {},
    QuitChannel {},
    ReceiveInvite {
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
