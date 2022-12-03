use std::collections::HashMap;

use gtk4::glib::GString;

/// Possible messages or requests a Controller can receive.
pub enum ControllerMessage {
    Register {
        pass: GString,
        nickname: GString,
        username: GString,
        realname: GString,
        address: String,
    },
    ChangeViewToMain {
        nickname: GString,
    },
    SendPrivMessage {
        message: GString,
    },
    ReceivePrivMessage {
        nickname: String,
        message: String,
    },
    JoinChannel {
        channel: GString,
    },
    AddViewToAddClient {},
    AddNewClient {
        client: GString,
    },
    QuitChannel {},
    RemoveConversation {},
    ChangeConversation {
        nickname: String,
    },
    AddInviteView {},
    SendInviteMessage {
        channel: GString,
    },
    RecieveInvite {
        nickname: String,
        channel: String,
    },
    SendListMessage {},
    ReceiveListChannels {
        channels: Vec<String>,
    },
    SendNamesMessage {},
    ReceiveNamesChannels {
        channels_and_clients: HashMap<String, Vec<String>>,
    },
    RegularMessage {
        message: String,
    },
}
