use gtk4 as gtk;

use crate::view_register::RegisterView;
use gtk::prelude::*;

pub struct RegisterController {
    view: RegisterView
}

impl RegisterController {
    pub fn new(view: RegisterView) -> Self {
        Self {
            view
        }
    }

    pub fn start(&mut self, app: &gtk::Application) {
        self.view.get_view(app).show();
    }
}
