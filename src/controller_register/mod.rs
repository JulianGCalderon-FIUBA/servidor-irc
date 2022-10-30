use gtk4 as gtk;

use crate::view_register::RegisterView;
use gtk::prelude::*;

pub struct RegisterController {
    view: RegisterView,
}

impl RegisterController {
    pub fn new() -> Self {
        Self {
            view: RegisterView::new()
        }
    }

    pub fn start(&mut self, app: &gtk::Application) {
        self.view.get_view(app).show();
    }
}
