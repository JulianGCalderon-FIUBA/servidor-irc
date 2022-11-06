use gtk4::glib::GString;

/// Possible messages or requests a Controller can receive.
pub enum ControllerMessage {
    Register {
        pass: GString,
        nickname: GString,
        username: GString,
        realname: GString
    },
    ChangeViewToMain {},
    SendPrivMessage {
        nickname: String,
        message: GString
    },
    RegularMessage {
        message: String
    },
}