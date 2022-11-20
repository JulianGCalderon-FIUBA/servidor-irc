use gtk::{ prelude::*, Box, Button };
use gtk4 as gtk;

use super::{ INACTIVE_SELECT_BUTTON_CSS, ACTIVE_SELECT_BUTTON_CSS, DISABLE_SELECT_BUTTON_CSS };

pub fn active_button(button: Button) {
    button.remove_css_class(INACTIVE_SELECT_BUTTON_CSS);
    button.add_css_class(ACTIVE_SELECT_BUTTON_CSS);
}

pub fn disactive_button(button: Button) {
    button.remove_css_class(ACTIVE_SELECT_BUTTON_CSS);
    button.add_css_class(INACTIVE_SELECT_BUTTON_CSS);
}

pub fn disable_button(button: Button) {
    button.remove_css_class(ACTIVE_SELECT_BUTTON_CSS);
    button.add_css_class(DISABLE_SELECT_BUTTON_CSS);
}

pub fn switch_visibility(visible_box: Box, no_visible_box: Box) {
    visible_box.set_visible(true);
    no_visible_box.set_visible(false);
}