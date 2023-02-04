/// Contains definition of used requests.
pub mod requests;

/// Contains multiple functions that create widgets for the view.
pub mod widgets_creation;

use gtk::{glib::Sender, prelude::*, Align::Start, Application, ApplicationWindow, Button, Label};
use gtk4 as gtk;

use crate::{
    controller::controller_message::ControllerMessage,
    views::{
        add_views::channel_members_view::widgets_creation::create_kick_label_box,
        widgets_creation::{build_application_window, create_center_button, create_label},
    },
};

use self::{
    requests::kick_request,
    widgets_creation::{create_kick_button, create_kick_label},
};

use super::{
    widgets_creation::{create_main_box_add_view, create_title},
    CONTINUE_BUTTON_TEXT,
};

const OPERATOR_FIRST_CHARACTER: &str = "@";
const TITLE: &str = "Members";

/// Shows channel members view.  
/// Contains an exit button.  
/// Uses sender to communicate with controller.
pub struct ChannelMembersView {
    button: Button,
}

impl Default for ChannelMembersView {
    fn default() -> Self {
        Self::new()
    }
}

impl ChannelMembersView {
    /// Creates new [`ChannelMembersView`]
    pub fn new() -> Self {
        Self {
            button: create_center_button(CONTINUE_BUTTON_TEXT),
        }
    }

    /// Returns the view's window.
    ///
    /// Receives the controller's app.
    pub fn get_view(
        &mut self,
        app: Application,
        clients: Vec<String>,
        nickname: String,
        channel: String,
        sender: Sender<ControllerMessage>,
    ) -> ApplicationWindow {
        let window = build_application_window();
        window.set_application(Some(&app));

        let main_box = create_main_box_add_view();

        main_box.append(&create_title(TITLE));

        if nickname == Self::get_operator(clients.clone()) {
            Self::list_members_operators(clients, channel, main_box.clone(), sender, window.clone())
        } else {
            Self::list_members(clients, main_box.clone());
        }

        main_box.append(&self.button);

        self.connect_button(window.clone());

        window.set_child(Some(&main_box));
        window
    }

    /// Connects exit button.
    ///
    /// Closes the window.
    fn connect_button(&mut self, window: ApplicationWindow) {
        self.button.connect_clicked(move |_| {
            window.close();
        });
    }

    /// Lists all members of the channel.
    ///
    /// Shows the operator of the channel.
    fn list_members(clients: Vec<String>, main_box: gtk::Box) {
        for client in &clients {
            let label: Label = if let Some(stripped) = client.strip_prefix(OPERATOR_FIRST_CHARACTER)
            {
                create_label(&format!("\t •\tOP: {stripped}"))
            } else {
                create_label(&format!("\t •\t{client}"))
            };
            label.set_halign(Start);
            label.set_margin_start(20);
            main_box.append(&label);
        }
    }

    /// Lists all members of the channel for the operator.
    ///
    /// List members with a kick button next to them.
    fn list_members_operators(
        clients: Vec<String>,
        channel: String,
        main_box: gtk::Box,
        sender: Sender<ControllerMessage>,
        window: ApplicationWindow,
    ) {
        for client in clients {
            let client_label_box = create_kick_label_box();

            if let Some(stripped) = client.strip_prefix(OPERATOR_FIRST_CHARACTER) {
                let label = create_kick_label(&format!("\t •\tOP: {stripped}"));
                client_label_box.append(&label);
            } else {
                let label = create_kick_label(&format!("\t •\t{client}"));
                let kick_button = create_kick_button();
                Self::connect_kick_button(
                    kick_button.clone(),
                    channel.clone(),
                    client,
                    sender.clone(),
                    window.clone(),
                );

                client_label_box.append(&label);
                client_label_box.append(&kick_button);
            }

            client_label_box.set_halign(Start);
            client_label_box.set_margin_start(20);
            main_box.append(&client_label_box);
        }
    }

    /// Connects kick button.
    ///
    /// Sends kick request to the controller.
    fn connect_kick_button(
        kick_button: Button,
        channel: String,
        member: String,
        sender: Sender<ControllerMessage>,
        window: ApplicationWindow,
    ) {
        kick_button.connect_clicked(move |_| {
            kick_request(channel.clone(), member.clone(), sender.clone());
            window.close();
        });
    }

    /// Gets operator from clients vec.
    ///
    /// Returns the operator of the channel.
    fn get_operator(clients: Vec<String>) -> String {
        for client in clients {
            if let Some(stripped) = client.strip_prefix(OPERATOR_FIRST_CHARACTER) {
                return stripped.to_string();
            }
        }
        "".to_string()
    }
}
