/// Contains ip selection view.  
pub mod ip_view;

/// The application's main view.  
/// Containts conversations, chat and features.  
pub mod view_main;

/// Contains the registration view.  
pub mod view_register;

/// Contains multiples views that help use of the features.
pub mod views_add;

/// Contains multiple functions that create widgets for every view.
pub mod widgets_creation;

const APP_TITLE: &str = "Lemon Pie IRC";
const ERROR_TEXT: &str = "ERROR";
const NICKNAME_LABEL_TEXT: &str = "Nickname:";
const PASSWORD_LABEL_TEXT: &str = "Password:";
const REALNAME_LABEL_TEXT: &str = "Your name:";
const SERVERNAME_LABEL_TEXT: &str = "Servername:";
const USERNAME_LABEL_TEXT: &str = "Username:";

const MAIN_BOX_CSS: &str = "main_box";
const WARNING_TEXT_CSS: &str = "warning_text";
