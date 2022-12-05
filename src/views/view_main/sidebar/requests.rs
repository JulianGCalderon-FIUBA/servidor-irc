use gtk::glib::Sender;
use gtk4 as gtk;

use crate::{controller::controller_message::ControllerMessage, views::ERROR_TEXT};

/// Sends a names request to the controller.  
///
/// Receives nothing.
pub fn add_view_to_add_client_request(sender: Sender<ControllerMessage>) {
    sender
        .send(ControllerMessage::SendNamesMessageToAddClient {})
        .expect(ERROR_TEXT);
}

/// Sends a list request to the controller.  
///
/// Receives nothing.
pub fn send_list_request(sender: Sender<ControllerMessage>) {
    sender
        .send(ControllerMessage::SendListMessage {})
        .expect(ERROR_TEXT);
}

/// Sends an add notification view request to the controller.  
///
/// Receives nothing.
pub fn add_notifications_view_request(sender: Sender<ControllerMessage>) {
    sender
        .send(ControllerMessage::AddNotificationsView {})
        .expect(ERROR_TEXT);
}

pub fn add_user_info_view(sender: Sender<ControllerMessage>) {
    sender
        .send(ControllerMessage::AddUserInfoView {})
        .expect(ERROR_TEXT);
}
