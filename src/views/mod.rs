/// Contains ip selection view.  
pub mod ip_view;

/// The application's main view.  
/// Containts conversations, chat and features.  
pub mod main_view;

/// Contains the registration view.  
pub mod register_view;

/// Contains multiples views that help use of the features.
pub mod add_views;

/// Contains useful functions.
pub mod utils;
/// Contains multiple functions that create widgets for every view.
pub mod widgets_creation;

const APP_TITLE: &str = "Lemon Pie IRC";
const EMPTY_MESSAGE_ERROR: &str = "¡Message is empty!";
const ENTRY_PLACEHOLDER: &str = "Message...";
const ERROR_TEXT: &str = "ERROR";
const MESSAGE_MAX_CHARACTERS_ERROR: &str = "¡Message too long!";
const MESSAGE_MAX_CHARACTERS: usize = 510;
const MESSAGE_MAX_LINE_CHARACTERS: usize = 60;
const NICKNAME_LABEL_TEXT: &str = "Nickname:";
const PASSWORD_LABEL_TEXT: &str = "Password:";
const REALNAME_LABEL_TEXT: &str = "Your name:";
const SEND_BUTTON_TEXT: &str = "➤";
const SERVERNAME_LABEL_TEXT: &str = "Servername:";
const USERNAME_LABEL_TEXT: &str = "Username:";

const CHAT_CSS: &str = "chat";
const MAIN_BOX_CSS: &str = "main_box";
const MESSAGE_BOX_CSS: &str = "message_box";
const RECEIVED_MESSAGE_CSS: &str = "received_message";
const SEND_MESSAGE_CSS: &str = "send_message";
const WARNING_TEXT_CSS: &str = "warning_text";
