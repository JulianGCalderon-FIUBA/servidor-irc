/// Contains ip selection view.  
pub mod ip_view;

/// The application's main view.  
/// Containts conversations, chat and features.  
pub mod main_view;

/// Contains the registration view.  
pub mod register_view;

/// Contains multiples views that help use of the features.
pub mod add_views;

/// Contains multiple functions that create widgets for every view.
pub mod widgets_creation;

const APP_TITLE: &str = "Lemon Pie IRC";
const ERROR_TEXT: &str = "ERROR";
const NICKNAME_LABEL_TEXT: &str = "Nickname:";
const PASSWORD_LABEL_TEXT: &str = "Password:";
const REALNAME_LABEL_TEXT: &str = "Your name:";
const SERVERNAME_LABEL_TEXT: &str = "Servername:";
const USERNAME_LABEL_TEXT: &str = "Username:";
const ENTRY_PLACEHOLDER: &str = "Message...";
const SEND_BUTTON_TEXT: &str = "Send"; // ➤ 

const CHAT_CSS: &str = "chat";
const MAIN_BOX_CSS: &str = "main_box";
const MESSAGE_BOX_CSS: &str = "message_box";
const WARNING_TEXT_CSS: &str = "warning_text";
