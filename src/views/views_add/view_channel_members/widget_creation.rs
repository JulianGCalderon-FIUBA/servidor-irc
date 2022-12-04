use gtk4 as gtk;

use gtk::{
    Box, Orientation::Horizontal, Button,
};

const KICK_LABEL: &str = "Kick";

pub fn create_kick_label_box() -> Box {
    Box::builder()
        .orientation(Horizontal)
        .margin_top(20)
        .margin_bottom(20)
        .build()

    // if member.starts_with("@") {
    //     let label = create_label(&format!("\t •\tOP: {}", &member[1..]));
    //     label_box.append(&label);
    // } else {
    //     let label = create_label(&format!("\t •\t{}", member));
    //     let kick_button = create_kick_button();
    //     connect_kick_button(kick_button.clone(), member, sender.clone());

    //     label_box.append(&label);
    //     label_box.append(&kick_button);
    // }
    
    // label_box
}

pub fn create_kick_button() -> Button {
    Button::builder().label(KICK_LABEL).build()
}

// fn connect_kick_button(kick_button: Button, member: String, sender: Sender<ControllerMessage>) {
//     kick_button.connect_clicked(move |_| {
//         kick_request(member, sender);
//     });
// }
