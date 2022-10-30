use gtk4 as gtk;

use crate::view_register::RegisterView;
use gtk::{prelude::*, glib::GString};

pub struct RegisterController {
    view: RegisterView,
}

impl RegisterController {
    pub fn new() -> Self {
        Self {
            view: RegisterView::new(),
        }
    }

    pub fn start(&mut self, app: &gtk::Application) {
        self.view.set_controller();

        self.view.get_view(app).show();
    }

    pub fn login_clicked(&self, pass: GString, nick: GString, username: GString) {
        println!("PASS {}", pass);
        println!("NICK {}", nick);
        println!("USER {} {} {} :{}", username, username, username, username);
        
    }
}
