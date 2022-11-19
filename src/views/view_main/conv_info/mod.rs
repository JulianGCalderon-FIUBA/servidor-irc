use gtk::{glib::Sender, prelude::*, Align, Box, Orientation};
use gtk4 as gtk;

use crate::{controller::controller_message::ControllerMessage, views::ERROR_TEXT};

use super::MainView;

impl MainView {
    pub fn create_conv_info(&mut self) -> Box {
        let conv_info = Box::builder()
            .orientation(Orientation::Vertical)
            .margin_end(12)
            .halign(Align::Start)
            .build();

        self.quit_channel.add_css_class("exit_channel");
        self.connect_quit_channel(self.sender.clone());
        conv_info.append(&self.quit_channel);

        conv_info.append(&self.channel_info);

        self.connect_func_channel(self.sender.clone());
        conv_info.append(&self.func_channel);

        conv_info
    }

    fn connect_quit_channel(&mut self, sender: Sender<ControllerMessage>) {
        self.quit_channel.connect_clicked(move |_| {
            let quit_channel = ControllerMessage::QuitChannel {};
            sender.send(quit_channel).expect(ERROR_TEXT);
            let change_conv = ControllerMessage::ChangeConversation {
                nickname: "".to_string(),
            };
            sender.send(change_conv).expect(ERROR_TEXT);
        });
    }

    fn connect_func_channel(&mut self, sender: Sender<ControllerMessage>) {
        self.func_channel.connect_clicked(move |_| {
            let add_invite_view = ControllerMessage::AddInviteView {};
            sender.send(add_invite_view).expect(ERROR_TEXT);
        });
    }

    pub fn remove_channel(&mut self, channel: String) {
        let mut counter = 0;
        for channel_button in &self.channels_button {
            if channel_button.label().unwrap() == channel {
                self.channels_box.remove(channel_button);
                break;
            }
            counter += 1;
        }
        if !self.channels_button.is_empty() {
            self.channels_button.remove(counter);
        }
        println!("Hola amigos");
    }
}
