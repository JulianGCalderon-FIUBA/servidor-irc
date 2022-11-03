use std::sync::mpsc::{Sender, channel, Receiver};

use gtk4 as gtk;

use crate::view_register::RegisterView;
use gtk::{prelude::*, Application};

pub struct Controller {
    app: Application,
    sender: Sender<String>,
    receiver: Receiver<String>
}

impl Controller {
    pub fn new(view: RegisterView) -> Self {

        let app = Application::new(Some("com.lemon-pie.demo"), Default::default());
        let (sender, receiver) = channel();
        Self {
            app,
            sender,
            receiver
        }
    }

    pub fn start(&mut self, sender: Sender<String>, receiver: Receiver<String>) {
        self.app.connect_activate(move |app| Self::build_ui(&app, sender.clone(), receiver));
        self.app.run();
        // self.view.get_view(app).show();
    }

    fn build_ui(app: &Application, sender: Sender<String>, receiver: Receiver<String>) {
        let mut view = RegisterView::new(sender, receiver);
        view.get_view(app).show();
    }
}
