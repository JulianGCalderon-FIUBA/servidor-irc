use gtk4 as gtk;
use gtk::{
    Align,
    Label,
    Button,
    Entry,
    Application,
    ApplicationWindow,
    Box,
    Orientation,
    prelude::*, glib::Sender,
};

use crate::controller::controller_message::ControllerMessage;

pub struct RegisterView {
    pub realname_entry: Entry,
    pub nick_entry: Entry,
    pub username_entry: Entry,
    pub pass_entry: Entry,
    pub login_button: Button,
    sender: Sender<ControllerMessage>,
}

impl RegisterView {
    pub fn new(sender: Sender<ControllerMessage>) -> Self {
        Self {
            realname_entry: create_entry(),
            nick_entry: create_entry(),
            username_entry: create_entry(),
            pass_entry: create_entry(),
            login_button: create_login_button("login"),
            sender,
        }
    }

    pub fn get_view(&mut self, app: Application) -> ApplicationWindow {
        let window = ApplicationWindow::builder().application(&app).title("Lemon Pie IRC").build();

        let main_box = create_main_box(Orientation::Vertical, 300, 300);
        main_box.add_css_class("main_box");

        let realname_box = Box::builder()
            .orientation(Orientation::Horizontal)
            .halign(gtk::Align::Center)
            .margin_top(20)
            .margin_bottom(20)
            .build();
        let label = create_label("Your name:");
        realname_box.append(&label);
        realname_box.append(&self.realname_entry);
        main_box.append(&realname_box);

        let nickname_box = Box::builder()
            .orientation(Orientation::Horizontal)
            .halign(gtk::Align::Center)
            .margin_top(20)
            .margin_bottom(20)
            .build();
        let label = create_label("Nickname:");
        nickname_box.append(&label);
        nickname_box.append(&self.nick_entry);
        main_box.append(&nickname_box);

        let username_box = Box::builder()
            .orientation(Orientation::Horizontal)
            .halign(gtk::Align::Center)
            .margin_top(20)
            .margin_bottom(20)        
            .build();
        let label = create_label("Username:");
        username_box.append(&label);
        username_box.append(&self.username_entry);
        main_box.append(&username_box);

        let password_box = Box::builder()
            .orientation(Orientation::Horizontal)
            .margin_top(20)
            .margin_bottom(20)
            .halign(gtk::Align::Center)
            .build();
        let label = create_label("Password:");
        password_box.append(&label);
        password_box.append(&self.pass_entry);
        main_box.append(&password_box);

        main_box.append(&self.login_button);

        self.connect_button(
            self.realname_entry.clone(),
            self.pass_entry.clone(),
            self.nick_entry.clone(),
            self.username_entry.clone(),
            self.sender.clone()
        );

        window.set_child(Some(&main_box));

        window
    }

    fn connect_button(
        &self,
        realname_entry: Entry,
        pass_entry: Entry,
        nick_entry: Entry,
        username_entry: Entry,
        sender: Sender<ControllerMessage>
    ) {
        self.login_button.connect_clicked(move |_| {
            if
                realname_entry.text().len() != 0 &&
                !!pass_entry.text().len() != 0 &&
                !!nick_entry.text().len() != 0 &&
                !!username_entry.text().len() != 0
            {
                // let pass_command = format!("PASS {}", pass_entry.text());
                // let nick_command = format!("NICK {}", nick_entry.text());
                // let user_command = format!(
                //     "USER {} {} {} :{}",
                //     username_entry.text(),
                //     username_entry.text(),
                //     username_entry.text(),
                //     username_entry.text()
                // );
                // sender.send(pass_command).expect("Error: pass command");
                // sender.send(nick_command).expect("Error: nick command");
                // sender.send(user_command).expect("Error: user command");

                // window.close();
                let register = ControllerMessage::Register { 
                                                                pass: pass_entry.text(), 
                                                                nickname: nick_entry.text(), 
                                                                username: username_entry.text(), 
                                                                realname: realname_entry.text() };
                sender.send(register).expect("Error: pass command");

                let change_view = ControllerMessage::ChangeViewToMain {};
                sender.send(change_view).expect("Error: pass command");
            }
        });
    }

    pub fn get_pass(&self) -> Entry {
        self.pass_entry.clone()
    }
}

fn create_label(label: &str) -> Label {
    Label::builder()
        .label(label)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .halign(Align::Center)
        .valign(Align::Center)
        .build()
}

fn create_entry() -> Entry {
    Entry::builder().build()
}

fn create_login_button(label: &str) -> Button {
    Button::builder()
        .label(label)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .build()
}

fn create_main_box(orientation: Orientation, height: i32, width: i32) -> Box {
    Box::builder()
        .orientation(orientation)
        .halign(gtk::Align::Center)
        .height_request(height)
        .width_request(width)
        .build()
}
