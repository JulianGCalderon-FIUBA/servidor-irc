use gtk4::{Align::Start, Box, Orientation::Vertical};

/// Creates the conversation info box.
///
/// Returns the box.
pub fn create_conv_info_box() -> Box {
    Box::builder()
        .orientation(Vertical)
        .width_request(250)
        .margin_end(12)
        .halign(Start)
        .build()
}
