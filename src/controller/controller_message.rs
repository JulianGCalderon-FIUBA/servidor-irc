use std::collections::HashMap;

use gtk4::glib::GString;

/// Possible messages or requests a Controller can receive.
pub enum ControllerMessage {
    AddInviteView {},
    AddNewClient {
        new_client: GString,
    },
    SendNamesMessageToAddClient {},
    AddViewToAddClient {
        channels_and_clients: HashMap<String, Vec<String>>,
    },
    ChangeConversation {
        nickname: String,
    },
    ChangeViewToMain {
        nickname: String,
    },
    JoinChannel {
        channel: String,
    },
    QuitChannel {},
    RecieveInvite {
        nickname: String,
        channel: String,
    },
    ReceivePrivMessage {
        sender_nickname: String,
        message: String,
        channel: Option<String>,
    },
    RemoveConversation {},
    Register {
        pass: GString,
        nickname: GString,
        username: GString,
        realname: GString,
    },
    RegularMessage {
        message: String,
    },
    SendInviteMessage {
        channel: GString,
    },
    ToRegister {
        address: String,
    },
    ReceiveListChannels {
        channels: Vec<String>,
    },
    ReceiveNamesChannels {
        channels_and_clients: HashMap<String, Vec<String>>,
    },
    SendListMessage {},
    SendNamesMessageToKnowMembers {},
    SendPrivMessage {
        message: GString,
    },
    KickMember {
        channel: String,
        member: String,
    },
    ReceiveKick {
        kicked: String,
        channel: String,
    }
}
