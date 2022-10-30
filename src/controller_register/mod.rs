use gtk4 as gtk;

use crate::view::RegisterView;
use gtk::prelude::*;

pub struct RegisterController {
    view: gtk::ApplicationWindow
}

impl RegisterController {
    pub fn new(app: &gtk::Application) -> Self {
        Self {
            view: RegisterView::new().get_view(app),
        }
    }

    pub fn start(&mut self) {
        self.view.show();
    }
}