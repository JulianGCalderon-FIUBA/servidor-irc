use gtk4 as gtk;

use gtk::{
    Align::Center, Box, Orientation::Horizontal, Button, traits::{BoxExt},
};
use crate::views::widgets_creation::{
    create_label,
};

const KICK_LABEL: &str = "Kick";

pub fn create_kick_label(member: String) -> Box {
    let label_box = Box::builder()
        .orientation(Horizontal)
        .halign(Center)
        .margin_top(20)
        .margin_bottom(20)
        .build();

    if member.starts_with("@") {
        let label = create_label(&format!("\t •\tOP: {}", &member[1..]));
        label_box.append(&label);
    } else {
        let label = create_label(&format!("\t •\t{}", member));
        let kick_button = create_kick_button();
        // connect_kick_button(kick_button, member);

        label_box.append(&label);
        label_box.append(&kick_button);
    }
    
    label_box
}

fn create_kick_button() -> Button {
    Button::builder().label(KICK_LABEL).build()
}

// fn connect_kick_button(kick_button: Button, member: String) {
//     kick_button.connect_clicked(move |_| {

//     });
// }
