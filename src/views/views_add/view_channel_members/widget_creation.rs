use gtk4 as gtk;

use gtk::{
    Align::Center, Box, Orientation::Horizontal,
};
use crate::views::widgets_creation::{
    create_label,
};

pub fn create_kick_label(member: String) -> Box {
    let label_box = Box::builder()
        .orientation(Horizontal)
        .halign(Center)
        .margin_top(20)
        .margin_bottom(20)
        .build();

    let label = create_label(&format!("\t •\tOP: {}", &member[1..]));

    let kick_button = create_kick_button(member);
    
    label_box
}

fn create_kick_button(member: String) {

}
