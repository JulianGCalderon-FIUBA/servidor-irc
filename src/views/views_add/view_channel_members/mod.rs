pub mod requests;
pub mod widgets_creation;

use gtk::{glib::Sender, prelude::*, Align::Start, Application, ApplicationWindow, Button, Label};
use gtk4 as gtk;

use crate::{
    controller::controller_message::ControllerMessage,
    views::widgets_creation::{build_application_window, create_center_button, create_label},
};

use self::{
    requests::kick_request,
    widgets_creation::{create_kick_button, create_kick_label, create_kick_label_box},
};

use super::widgets_creation::{create_main_box_add_view, create_title};

pub struct ChannelMembersView {
    button: Button,
}

impl Default for ChannelMembersView {
    fn default() -> Self {
        Self::new()
    }
}

impl ChannelMembersView {
    pub fn new() -> Self {
        Self {
            button: create_center_button("ok"),
        }
    }

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

        main_box.append(&create_title("Members"));

        if nickname == Self::get_operator(clients.clone()) {
            Self::list_members_operators(clients, channel, main_box.clone(), sender);
        } else {
            Self::list_members(clients, main_box.clone());
        }

        main_box.append(&self.button);

        self.connect_button(window.clone());

        window.set_child(Some(&main_box));
        window
    }

    fn connect_button(&mut self, window: ApplicationWindow) {
        self.button.connect_clicked(move |_| {
            window.close();
        });
    }

    fn list_members(clients: Vec<String>, main_box: gtk::Box) {
        for client in &clients {
            //mejorar
            let label: Label = if let Some(stripped) = client.strip_prefix('@') {
                create_label(&format!("\t •\tOP: {}", stripped))
            } else {
                create_label(&format!("\t •\t{}", client))
            };
            label.set_halign(Start);
            label.set_margin_start(20);
            main_box.append(&label);
        }
    }

    fn list_members_operators(
        clients: Vec<String>,
        channel: String,
        main_box: gtk::Box,
        sender: Sender<ControllerMessage>,
    ) {
        for client in clients {
            let client_label_box = create_kick_label_box();

            if let Some(stripped) = client.strip_prefix('@') {
                let label = create_kick_label(&format!("\t •\tOP: {}", stripped));
                client_label_box.append(&label);
            } else {
                let label = create_kick_label(&format!("\t •\t{}", client));
                let kick_button = create_kick_button();
                Self::connect_kick_button(
                    kick_button.clone(),
                    channel.clone(),
                    client,
                    sender.clone(),
                );

                client_label_box.append(&label);
                client_label_box.append(&kick_button);
            }

            client_label_box.set_halign(Start);
            client_label_box.set_margin_start(20);
            main_box.append(&client_label_box);
        }
    }

    fn connect_kick_button(
        kick_button: Button,
        channel: String,
        member: String,
        sender: Sender<ControllerMessage>,
    ) {
        kick_button.connect_clicked(move |_| {
            kick_request(channel.clone(), member.clone(), sender.clone());
        });
    }

    fn get_operator(clients: Vec<String>) -> String {
        for client in clients {
            if let Some(stripped) = client.strip_prefix('@') {
                return stripped.to_string();
            }
        }
        "".to_string()
    }
}
