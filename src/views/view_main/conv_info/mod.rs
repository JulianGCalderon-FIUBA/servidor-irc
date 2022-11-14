use gtk::{prelude::*, Align, Box, Orientation, glib::Sender};
use gtk4 as gtk;

use crate::controller::controller_message::ControllerMessage;

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

        conv_info.append(&self.func_channel);

        conv_info
    }

    fn connect_quit_channel(&mut self, sender: Sender<ControllerMessage>) {
        self.quit_channel.connect_clicked(move |_| {
            let quit_channel = ControllerMessage::QuitChannel {};
            sender.send(quit_channel).expect("ERROR");
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
        if self.channels_button.len() > 0 {
            self.channels_button.remove(counter);
        }
        println!("Hola amigos");
    }
}
