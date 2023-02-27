use gtk4::{traits::WidgetExt, Box, Button};

use super::{ACTIVE_SELECT_BUTTON_CSS, DISABLE_SELECT_BUTTON_CSS, INACTIVE_SELECT_BUTTON_CSS};

/// Changes css to active button
pub fn activate_button(button: Button) {
    button.remove_css_class(INACTIVE_SELECT_BUTTON_CSS);
    button.add_css_class(ACTIVE_SELECT_BUTTON_CSS);
}

/// Changes css to disactive button
pub fn disactivate_button(button: Button) {
    button.remove_css_class(ACTIVE_SELECT_BUTTON_CSS);
    button.add_css_class(INACTIVE_SELECT_BUTTON_CSS);
    button.set_has_tooltip(false);
}

/// Changes css to disabled button
pub fn disable_button(button: Button, tooltip_text: &str) {
    button.remove_css_class(ACTIVE_SELECT_BUTTON_CSS);
    button.add_css_class(DISABLE_SELECT_BUTTON_CSS);
    button.set_sensitive(false);
    button.set_has_tooltip(true);
    button.set_tooltip_text(Some(tooltip_text));
}

/// Changes buttons visibility
pub fn switch_visibility(visible_box: Box, no_visible_box: Box) {
    visible_box.set_visible(true);
    no_visible_box.set_visible(false);
}
