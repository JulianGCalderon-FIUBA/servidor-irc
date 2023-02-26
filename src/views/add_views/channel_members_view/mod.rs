/// Contains definition of used requests.
pub mod requests;

/// Contains multiple functions that create widgets for the view.
pub mod widgets_creation;

use gtk4::{
    glib::Sender,
    traits::{BoxExt, ButtonExt, GtkWindowExt},
    Application, ApplicationWindow, Box, Button,
};

use crate::{
    controller::controller_message::ControllerMessage,
    views::{
        add_views::channel_members_view::widgets_creation::create_kick_label_box,
        widgets_creation::{build_application_window, create_center_button, create_label},
    },
};

use self::{requests::kick_request, widgets_creation::create_kick_button};

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
    channel: String,
    clients: Vec<String>,
    nickname: String,
    sender: Sender<ControllerMessage>,
}

impl ChannelMembersView {
    /// Creates new [`ChannelMembersView`]
    pub fn new(
        channel: String,
        clients: Vec<String>,
        nickname: String,
        sender: Sender<ControllerMessage>,
    ) -> Self {
        Self {
            button: create_center_button(CONTINUE_BUTTON_TEXT),
            channel,
            clients,
            nickname,
            sender,
        }
    }

    /// Returns the view's window.
    ///
    /// Receives the controller's app.
    pub fn get_view(&mut self, app: Application) -> ApplicationWindow {
        let window = build_application_window();
        window.set_application(Some(&app));

        let main_box = create_main_box_add_view();

        main_box.append(&create_title(TITLE));

        self.list_members(main_box.clone(), window.clone());

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

    /// Lists all members of the channel for the operator.
    ///
    /// List members with a kick button next to them.
    fn list_members(&mut self, main_box: Box, window: ApplicationWindow) {
        let im_operator = self.nickname.clone() == self.get_operator();

        for client in self.clients.clone() {
            let client_label_box = create_kick_label_box();

            if let Some(stripped) = client.strip_prefix(OPERATOR_FIRST_CHARACTER) {
                let label = create_label(&format!("\t 👑\t {stripped}\t"));
                client_label_box.append(&label);
            } else {
                let label = create_label(&format!("\t ⦿\t {client}\t"));
                client_label_box.append(&label);

                if im_operator {
                    let kick_button = create_kick_button();
                    self.connect_kick_button(kick_button.clone(), client, window.clone());
                    client_label_box.append(&kick_button);
                }
            }

            main_box.append(&client_label_box);
        }
    }

    /// Connects kick button.
    ///
    /// Sends kick request to the controller.
    fn connect_kick_button(
        &mut self,
        kick_button: Button,
        member: String,
        window: ApplicationWindow,
    ) {
        let sender = self.sender.clone();
        let channel = self.channel.clone();

        kick_button.connect_clicked(move |_| {
            kick_request(channel.clone(), member.clone(), sender.clone());
            window.close();
        });
    }

    /// Gets operator from clients vec.
    ///
    /// Returns the operator of the channel.
    fn get_operator(&mut self) -> String {
        for client in self.clients.clone() {
            if let Some(stripped) = client.strip_prefix(OPERATOR_FIRST_CHARACTER) {
                return stripped.to_string();
            }
        }
        String::new()
    }
}
