use gtk4::glib::GString;

/// Possible messages or requests a Controller can receive.
pub enum ControllerMessage {
    Register {
        pass: GString,
        nickname: GString,
        username: GString,
        realname: GString,
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
    AddNewChannel {
        channel: GString,
    },
    AddNewClient {
        client: GString,
    },
    QuitChannel {},
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
    RegularMessage {
        message: String,
    },
}