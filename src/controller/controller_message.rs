use std::collections::HashMap;

use gtk4::glib::GString;

/// Possible messages or requests a Controller can receive.
pub enum ControllerMessage {
    AddNewClient {
        new_client: GString,
    },
    ChangeConversation {
        nickname: String,
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
        realname: String,
        servername: String,
        nickname: String,
        username: String,
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
        nickname: String,
        channel: String,
    },
    ReceiveKick {
        kicked: String,
        channel: String,
    },
    ReceiveListChannels {
        channels: Vec<String>,
    },
    ReceiveNamesChannels {
        channels_and_clients: HashMap<String, Vec<String>>,
    },
    ReceivePrivMessage {
        sender_nickname: String,
        message: String,
        channel: Option<String>,
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
