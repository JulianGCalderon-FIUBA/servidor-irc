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
    AddViewToAddChannel {
    },
    AddNewChannel {
        channel: GString,
    },
    ChangeConversation {
        nickname: String,
    },
    RegularMessage {
        message: String,
    },
}
